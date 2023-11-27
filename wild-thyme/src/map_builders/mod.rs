mod common;
mod drunkards_walk;
mod nest_level;
mod simple_map;
mod town_level;

use crate::{Map, Position, World};

use self::{
    drunkards_walk::DrunkardsWalkBuilder, nest_level::NestLevelBuilder,
    town_level::TownLevelBuilder,
};

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&mut self) -> Map;
    fn get_starting_position(&mut self) -> Position;
}

pub fn make_builder(new_depth: i32) -> Box<dyn MapBuilder> {
    // Box::new(NestLevelBuilder::new(new_depth))
    if new_depth == 1 {
        Box::new(TownLevelBuilder::new())
    } else if new_depth % 3 == 0 {
        Box::new(NestLevelBuilder::new(new_depth))
    } else {
        Box::new(DrunkardsWalkBuilder::new(new_depth))
    }
}
