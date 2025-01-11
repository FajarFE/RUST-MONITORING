#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::_entities::monitoring_data::{ActiveModel, Entity, Model};
use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::primitives::DateTime as AwsDateTime;
use aws_sdk_s3::Client;
use axum::{debug_handler, extract::Query};
use chrono::Utc;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use xlsxwriter::Workbook;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub ph_water: Option<String>,
    pub turbidity_water: Option<String>,
    pub temperature_water: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.ph_water = Set(self.ph_water.clone());
        item.turbidity_water = Set(self.turbidity_water.clone());
        item.temperature_water = Set(self.temperature_water.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[derive(Deserialize)]
pub struct ListParams {
    pub month: u32,
    pub year: i32,
    pub device_code: String,
}

#[debug_handler]
pub async fn list(
    auth: auth::JWT,
    query: Query<ListParams>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let _current_user = crate::models::users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // Ambil data
    let data = Entity::write_data_to_excel(
        &ctx.db,
        1,
        Some(query.month),
        Some(query.year),
        query.device_code.clone(),
    )
    .await?;

    // Menyusun file Excel
    let workbook_filename = format!("data-{}.xlsx", _current_user.name);
    let workbook_path = format!("/tmp/{}", &workbook_filename); // Menyimpan file di direktori sementara
    let data_clone = data.clone();
    let workbook_path_clone = workbook_path.clone();
    tokio::task::spawn_blocking(move || {
        let workbook = Workbook::new(&workbook_path_clone).unwrap();
        let mut sheet = workbook.add_worksheet(Some("Data")).unwrap();

        // Menulis data ke worksheet
        sheet.write_string(0, 0, "Group", None).unwrap();
        sheet.write_string(0, 1, "Avg PH Water", None).unwrap();
        sheet.write_string(0, 2, "Avg Temperature", None).unwrap();
        sheet.write_string(0, 3, "Avg Turbidity", None).unwrap();
        sheet.write_string(0, 4, "Last Created At", None).unwrap();

        let mut row = 1;
        if let Some(results) = data_clone {
            for result in results {
                sheet.write_string(row, 0, &result.group, None).unwrap();
                sheet
                    .write_number(row, 1, result.avg_ph_water.unwrap_or(0.0), None)
                    .unwrap();
                sheet
                    .write_number(row, 2, result.avg_temperature.unwrap_or(0.0), None)
                    .unwrap();
                sheet
                    .write_number(row, 3, result.avg_turbidity.unwrap_or(0.0), None)
                    .unwrap();
                sheet
                    .write_string(row, 4, &result.last_created_at.unwrap().to_string(), None)
                    .unwrap();
                row += 1;
            }
        }

        workbook.close().unwrap();
    })
    .await?;

    // Mendapatkan informasi S3 dari env
    let access_key_id = std::env::var("AWS_ACCESS_KEY_ID").unwrap();
    let secret_access_key = std::env::var("AWS_SECRET_ACCESS_KEY").unwrap();
    let aws_url = std::env::var("AWS_URL").unwrap();

    let creds = Credentials::from_keys(access_key_id, secret_access_key, None);

    let cfg = aws_config::from_env()
        .endpoint_url(aws_url)
        .region(Region::new("eu-west-2"))
        .credentials_provider(creds)
        .load()
        .await;

    let s3 = Client::new(&cfg);
    let bucket_name = std::env::var("AWS_BUCKET_NAME").unwrap();
    let bucket_name_clone = bucket_name.clone();
    let object_key = workbook_filename.clone();
    let file_path = workbook_path;

    // Create expiration time 24 hours from now
    let expiration: chrono::DateTime<Utc> = Utc::now() + chrono::Duration::hours(24);

    // Convert chrono::DateTime<Utc> to SystemTime, then to AwsDateTime
    let system_time: std::time::SystemTime = expiration.into();
    let expiration_aws: AwsDateTime = system_time.into();
    let byte_stream = aws_sdk_s3::primitives::ByteStream::from_path(&file_path)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;
    let input = s3
        .put_object()
        .bucket(bucket_name_clone)
        .body(byte_stream)
        .expires(expiration_aws) // Add expiration time
        .send()
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

    format::json(json!({
        "message": "Data berhasil disimpan",
        "filename": workbook_filename,
        "s3_url": format!("https://{}.s3.amazonaws.com/{}", bucket_name, object_key),
        "s3_response": {
            "e_tag": input.e_tag,
            "server_side_encryption": input.server_side_encryption.map(|s| s.as_str().to_string()),
            "version_id": input.version_id,
        }
    }))
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/monitoring_data/")
        .add("/", post(add))
        .add(":id", get(get_one))
        .add(":id", delete(remove))
        .add(":id", put(update))
        .add(":id", patch(update))
}
