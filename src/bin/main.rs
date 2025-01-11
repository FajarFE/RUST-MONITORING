use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tokio::sync::mpsc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;

use loco_rs::cli;
use monitoringair::app::App;
use migration::Migrator;

type Tx = mpsc::Sender<Message>;
type ClientMap = Arc<Mutex<HashMap<String, Tx>>>;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    // Set up your WebSocket listener
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let client_map: ClientMap = Arc::new(Mutex::new(HashMap::new()));
    
    // Start the WebSocket listener task
    tokio::spawn(start_websocket_server(listener, client_map.clone()));
    
    // Run the CLI tool (assuming it's part of the application)
    cli::main::<App, Migrator>().await
}

async fn start_websocket_server(listener: TcpListener, clients: ClientMap) {
    println!("WebSocket server running on ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        let clients = clients.clone();

        // Spawn a new task to handle the WebSocket connection
        tokio::spawn(handle_connection(stream, clients));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, clients: ClientMap) {
    // Upgrade the TCP stream to a WebSocket stream
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    println!("New WebSocket connection");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // You might want to identify clients with a unique ID, e.g., a random string or UUID
    let client_id = uuid::Uuid::new_v4().to_string();
    println!("Client ID: {}", client_id);

    // Add the client to the shared map
    let (tx, _rx) = mpsc::channel::<Message>(32);
    clients.lock().unwrap().insert(client_id.clone(), tx);

    // Send a welcome message to the new client
    if let Err(e) = ws_sender.send(Message::Text("Welcome!".into())).await {
        println!("Error sending message to client {}: {}", client_id, e);
        return;
    }

    // Listen for incoming messages from the client
    while let Some(message) = ws_receiver.next().await {
        match message {
            Ok(Message::Text(text)) => {
                println!("Received from {}: {}", client_id, text);
                
                // Handle incoming message, for example: broadcast or process it
            }
            Ok(Message::Close(_)) => {
                println!("Client {} disconnected", client_id);
                break;
            }
            Err(e) => {
                println!("Error with client {}: {}", client_id, e);
                break;
            }
            _ => {}
        }
    }

    // Remove the client when disconnected
    clients.lock().unwrap().remove(&client_id);
}
