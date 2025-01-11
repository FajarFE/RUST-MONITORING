use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex, oneshot}; // Use tokio::sync::Mutex and oneshot for signaling
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::signal; // Import signal handling
use loco_rs::cli;
use monitoringair::app::App;
use migration::Migrator;

#[derive(Debug, Serialize, Deserialize)]
struct DeviceMessage {
    device_id: String,
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn a task to run the CLI logic concurrently
    tokio::spawn(async {
        if let Err(e) = cli::main::<App, Migrator>().await {
            eprintln!("Error running CLI: {:?}", e);
        }
    });

    // WebSocket server setup
    let addr = "127.0.0.1:5000";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind address");

    println!("WebSocket server listening on ws://{}", addr);

    // Use tokio::sync::Mutex to allow safe async access to device_map
    let device_map: Arc<Mutex<HashMap<String, mpsc::Sender<Message>>>> = Arc::new(Mutex::new(HashMap::new()));

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let server_task = tokio::spawn(async move {
        // Accept incoming connections and handle them
        while let Ok((stream, _)) = listener.accept().await {
            let device_map = device_map.clone();

            tokio::spawn(async move {
                let ws_stream = accept_async(stream)
                    .await
                    .expect("Failed to accept WebSocket connection");

                let (mut tx, mut rx) = ws_stream.split();

                // Communication channel for this connection
                let (message_sender, mut message_receiver) = mpsc::channel(100);

                // Spawn a task to send messages to this client
                tokio::spawn(async move {
                    while let Some(message) = message_receiver.recv().await {
                        if tx.send(message).await.is_err() {
                            break;
                        }
                    }
                });

                // Process incoming messages
                while let Some(msg) = rx.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            match serde_json::from_str::<DeviceMessage>(&text) {
                                Ok(parsed_msg) => {
                                    let device_id = parsed_msg.device_id.clone();
                                    let mut map = device_map.lock().await; // Lock asynchronously with `await`

                                    // Register the device_id with the sender
                                    map.insert(device_id.clone(), message_sender.clone());

                                    // Handle message processing or broadcasting
                                    if let Some(sender) = map.get(&device_id) {
                                        let response = DeviceMessage {
                                            device_id,
                                            data: format!("Echo: {}", parsed_msg.data),
                                        };
                                        let response_message = Message::Text(serde_json::to_string(&response).unwrap().into());
                                        let _ = sender.send(response_message).await;
                                    }
                                }
                                Err(e) => eprintln!("Invalid message format: {:?}", e),
                            }
                        }
                        Ok(Message::Close(_)) => {
                            break;
                        }
                        _ => {}
                    }
                }

                println!("Connection closed");
            });
        }

        // Send shutdown signal
        let _ = shutdown_tx.send(());
    });

    // Wait for either Ctrl+C or server task completion
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
        }
        _ = shutdown_rx => {
            println!("Server task completed.");
        }
    }

    // Ensure server_task completes before exiting
    server_task.await?;

    Ok(())
}
