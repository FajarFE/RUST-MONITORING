#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20250102_105614_category_fishes;
mod m20250102_105718_limitations;
mod m20250102_105814_monitorings;
mod m20250102_105946_monitoring_data;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250102_105614_category_fishes::Migration),
            Box::new(m20250102_105718_limitations::Migration),
            Box::new(m20250102_105814_monitorings::Migration),
            Box::new(m20250102_105946_monitoring_data::Migration),
            // inject-below (do not remove this comment)
            // inject-above (do not remove this comment)
        ]
    }
}