use super::_entities::monitorings::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Monitorings = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
