//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "monitoring_data")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub monitoring_id: i32,
    pub ph_water: Option<String>,
    pub turbidity_water: Option<String>,
    pub temperature_water: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::monitorings::Entity",
        from = "Column::MonitoringId",
        to = "super::monitorings::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Monitorings,
}

impl Related<super::monitorings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Monitorings.def()
    }
}
