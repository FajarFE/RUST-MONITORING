use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Limitations::Table)
                    .col(pk_auto(Limitations::Id))
                    .col(integer(Limitations::UsersId))
                    .col(string_null(Limitations::MaxPh))
                    .col(string_null(Limitations::MinPh))
                    .col(string_null(Limitations::MaxTemperature))
                    .col(string_null(Limitations::MinTemperature))
                    .col(string_null(Limitations::MaxTurbidity))
                    .col(string_null(Limitations::MinTurbidity))
                    .col(integer(Limitations::CategoryFishId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-limitations-users")
                            .from(Limitations::Table, Limitations::UsersId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-limitations-category_fishes")
                            .from(Limitations::Table, Limitations::CategoryFishId)
                            .to(CategoryFishes::Table, CategoryFishes::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Limitations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Limitations {
    Table,
    Id,
    UsersId,
    MaxPh,
    MinPh,
    MaxTemperature,
    MinTemperature,
    MaxTurbidity,
    MinTurbidity,
    CategoryFishId,
    
}


#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum CategoryFishes {
    Table,
    Id,
}
