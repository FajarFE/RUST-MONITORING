//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "limitations")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub users_id: i32,
    pub max_ph: Option<String>,
    pub min_ph: Option<String>,
    pub max_temperature: Option<String>,
    pub min_temperature: Option<String>,
    pub max_turbidity: Option<String>,
    pub min_turbidity: Option<String>,
    pub category_fish_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category_fishes::Entity",
        from = "Column::CategoryFishId",
        to = "super::category_fishes::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    CategoryFishes,
    #[sea_orm(has_many = "super::monitorings::Entity")]
    Monitorings,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UsersId",
        to = "super::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::category_fishes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CategoryFishes.def()
    }
}

impl Related<super::monitorings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Monitorings.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}
