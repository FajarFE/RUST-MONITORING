use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(MonitoringData::Table)
                    .col(pk_auto(MonitoringData::Id))
                    .col(integer(MonitoringData::MonitoringId))
                    .col(string_null(MonitoringData::PhWater))
                    .col(string_null(MonitoringData::TurbidityWater))
                    .col(string_null(MonitoringData::TemperatureWater))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-monitoring_data-monitorings")
                            .from(MonitoringData::Table, MonitoringData::MonitoringId)
                            .to(Monitorings::Table, Monitorings::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MonitoringData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MonitoringData {
    Table,
    Id,
    MonitoringId,
    PhWater,
    TurbidityWater,
    TemperatureWater,
    
}


#[derive(DeriveIden)]
enum Monitorings {
    Table,
    Id,
}
