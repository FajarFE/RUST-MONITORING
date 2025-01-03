use sea_orm::entity::prelude::*;
use super::_entities::limitations::{ActiveModel, Entity};
pub type Limitations = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
