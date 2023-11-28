mod common;
mod drunkards_walk;
mod nest_level;
mod simple_map;
mod town_level;
mod wizard_level;

use crate::{Map, Position, World};

use self::{
    drunkards_walk::DrunkardsWalkBuilder, nest_level::NestLevelBuilder,
    simple_map::SimpleMapBuilder, town_level::TownLevelBuilder, wizard_level::WizardLevelBuilder,
};

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&mut self) -> Map;
    fn get_starting_position(&mut self) -> Position;
}

pub fn make_builder(new_depth: i32) -> Box<dyn MapBuilder> {
    // Box::new(TownLevelBuilder::new())
    // Box::new(WizardLevelBuilder::new())
    if new_depth == 1 {
        Box::new(TownLevelBuilder::new())
    } else if new_depth % 3 == 0 {
        Box::new(NestLevelBuilder::new(new_depth))
    } else if new_depth % 10 == 0 {
        Box::new(WizardLevelBuilder::new())
    } else if new_depth == 13 {
        Box::new(SimpleMapBuilder::new(new_depth))
    } else {
        Box::new(DrunkardsWalkBuilder::new(new_depth))
    }
}
