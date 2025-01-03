use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Monitorings::Table)
                    .col(pk_auto(Monitorings::Id))
                    .col(integer(Monitorings::UsersId))
                    .col(string_null(Monitorings::NameMonitoring))
                    .col(string_null(Monitorings::CodeDevice))
                    .col(integer(Monitorings::LimitationId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-monitorings-users")
                            .from(Monitorings::Table, Monitorings::UsersId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-monitorings-limitations")
                            .from(Monitorings::Table, Monitorings::LimitationId)
                            .to(Limitations::Table, Limitations::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Monitorings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Monitorings {
    Table,
    Id,
    UsersId,
    NameMonitoring,
    CodeDevice,
    LimitationId,
    
}


#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Limitations {
    Table,
    Id,
}
