use sea_orm::entity::prelude::*;
use super::_entities::example_table::{ActiveModel, Entity};
pub type ExampleTable = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
