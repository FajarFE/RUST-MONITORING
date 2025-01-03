use super::_entities::{
    monitoring_data::{ActiveModel, Entity, Model, Relation},
    monitorings::Column as ColumnMonitorings,
};
use sea_orm::{entity::prelude::*, JoinType, QuerySelect};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub type MonitoringData = Entity;

impl ActiveModelBehavior for ActiveModel {
    // Implementasi ActiveModelBehavior jika diperlukan
}

impl Entity {
    pub async fn write_data_to_excel(
        db: &DatabaseConnection,
        users: i32,
        month: Option<u32>,
        year: Option<i32>,
        device_code: String,
    ) -> Result<Option<Vec<CustomResult>>, DbErr> {
        // Membuat query dasar dengan inner join ke entitas MonitoringsEntity
        let mut query = Entity::find()
            .join(JoinType::InnerJoin, Relation::Monitorings.def())
            .filter(ColumnMonitorings::CodeDevice.eq(device_code))
            .filter(ColumnMonitorings::UsersId.eq(users));

        // Filter berdasarkan tahun
        if let Some(y) = year {
            query = query.filter(Expr::cust_with_values(
                "EXTRACT(YEAR FROM create_at) = ?",
                vec![Into::<Value>::into(y)],
            ));
        }

        // Filter berdasarkan bulan
        if let Some(m) = month {
            query = query.filter(Expr::cust_with_values(
                "EXTRACT(MONTH FROM create_at) = ?",
                vec![Into::<Value>::into(m)],
            ));
        }

        // Mendapatkan data dari database
        let data = query.all(db).await?;

        // Mengelompokkan data berdasarkan bulan atau hari (tanggal 1-30) berdasarkan input
        let grouped_data = if let Some(_) = month {
            group_data_by_day(&data) // Kelompokkan berdasarkan hari jika bulan diberikan
        } else {
            group_data_by_month(&data) // Kelompokkan berdasarkan bulan jika hanya tahun diberikan
        };

        // Menghitung rata-rata untuk setiap kelompok
        let results: Vec<CustomResult> = grouped_data
            .into_iter()
            .map(|(group, items)| CustomResult {
                group,
                avg_ph_water: calculate_average(
                    items
                        .iter()
                        .map(|item| item.ph_water.as_ref().and_then(|s| s.parse::<f64>().ok())),
                ),
                avg_temperature: calculate_average(items.iter().map(|item| {
                    item.temperature_water
                        .as_ref()
                        .and_then(|s| s.parse::<f64>().ok())
                })),
                avg_turbidity: calculate_average(items.iter().map(|item| {
                    item.turbidity_water
                        .as_ref()
                        .and_then(|s| s.parse::<f64>().ok())
                })),
                last_created_at: items.last().map(|item| item.created_at.naive_local()),
            })
            .collect();

        Ok(Some(results))
    }
}

fn group_data_by_day(data: &[Model]) -> HashMap<String, Vec<Model>> {
    let mut grouped = HashMap::new();
    for item in data {
        let day_key = item.created_at.format("%Y-%m-%d").to_string();
        grouped
            .entry(day_key)
            .or_insert_with(Vec::new)
            .push(item.clone());
    }
    grouped
}

fn group_data_by_month(data: &[Model]) -> HashMap<String, Vec<Model>> {
    let mut grouped = HashMap::new();
    for item in data {
        let month_key = item.created_at.format("%Y-%m").to_string();
        grouped
            .entry(month_key)
            .or_insert_with(Vec::new)
            .push(item.clone());
    }
    grouped
}

fn calculate_average<I>(values: I) -> Option<f64>
where
    I: Iterator<Item = Option<f64>>,
{
    let (sum, count) = values.fold((0.0, 0), |(sum, count), value| {
        if let Some(v) = value {
            (sum + v, count + 1)
        } else {
            (sum, count)
        }
    });
    if count > 0 {
        Some(sum / count as f64)
    } else {
        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomResult {
    pub group: String, // Hari atau bulan
    pub avg_ph_water: Option<f64>,
    pub avg_temperature: Option<f64>,
    pub avg_turbidity: Option<f64>,
    pub last_created_at: Option<chrono::NaiveDateTime>,
}

// Execute query and process results
