use sea_orm::entity::prelude::*;
use super::_entities::category_fishes::{ActiveModel, Entity};
pub type CategoryFishes = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
