use std::collections::HashMap;

use rltk::{RandomNumberGenerator, RGB};
use specs::*;

use crate::{
    components::Position,
    map::{Map, TileType},
    map_builders::common::apply_room_to_map,
    random_table::RandomTable,
    rect::Rect,
    spawners::{self, spawn_specific_on_point},
};

use super::{common::release_drunk, MapBuilder};

pub struct WizardLevelBuilder {
    map: Map,
    starting_position: Position,
    start_room: Rect,
    portal_room: Rect,
}

impl MapBuilder for WizardLevelBuilder {
    fn build_map(&mut self) {
        self.start_room = Rect::new(30, 10, 20, 20);
        apply_room_to_map(&mut self.map, &self.start_room);
        self.map.rooms.push(self.start_room);

        self.portal_room = Rect::new(40, 30, 10, 6);
        apply_room_to_map(&mut self.map, &self.portal_room);
        self.map.rooms.push(self.portal_room);
        let start_center = self.portal_room.center();
        let start_center_idx = self.map.xy_idx(start_center.0, start_center.1);
        self.map.tiles[start_center_idx] = TileType::DownStairs;

        let start = self.start_room.center();
        self.starting_position = Position {
            x: start.0,
            y: start.1,
        };

        for i in 0..self.map.tiles.len() {
            if i % 13 == 0
                || i % 17 == 0
                || i % 31 == 0
                || i & 43 == 0
                || i % 11 == 0
                || i % 19 == 0
            {
                self.map.tiles[i] = TileType::Floor;
            }
        }

        let mut rng = RandomNumberGenerator::new();
        release_drunk(&mut self.map, (35, 14), 20, &mut rng, 10, TileType::Water);
        release_drunk(&mut self.map, (36, 14), 20, &mut rng, 10, TileType::Water);
        release_drunk(&mut self.map, (37, 14), 20, &mut rng, 10, TileType::Water);
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        spawners::npc(
            ecs,
            40,
            18,
            rltk::to_cp437('☺'),
            RGB::from_hex("#805080").expect("hardcoded"),
            "FOREST WIZARD",
            vec![
                "WELCOME!".to_string(),
                "a FRIENDLY EAGLE will help you get home".to_string(),
                "make sure you have the ingredients you need!".to_string(),
                "hello, friend".to_string(),
                "I sure love to cook".to_string(),
                "the old ways teach us to GIVE".to_string(),
                "there is nothing but PAIN beyond this level".to_string(),
                "if you go deeper, you'll find nothing".to_string(),
                "only a sense of PRIDE and SATISFACTION beyond here".to_string(),
            ],
        );
        spawners::spawn_treeportal(ecs, &self.portal_room);

        let spawn_table = RandomTable::new()
            .add("BERRY BUSH", 10)
            .add("GOOD THYME", 5)
            .add("ROCK", 5)
            .add("MAGIC MUSHROOM", 10)
            .add("MOREL MUSHROOM", 5)
            .add("FROG", 5)
            .add("FRIENDLY EAGLE", 3)
            .add("BUTTERFLY", 10);
        let room = self.start_room;
        let mut spawn_points: HashMap<usize, String> = HashMap::new();
        {
            let mut rng = ecs.write_resource::<RandomNumberGenerator>();
            let num_spawns = 30;

            for _i in 0..num_spawns {
                let mut added = false;
                let mut tries = 0;
                while !added && tries < 20 {
                    let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                    let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                    let idx = (y * self.map.width as usize) + x;
                    if !spawn_points.contains_key(&idx) {
                        spawn_points.insert(idx, spawn_table.roll(&mut rng));
                        added = true;
                    } else {
                        tries += 1;
                    }
                }
            }
        }

        for spawn in spawn_points.iter() {
            let x = (*spawn.0 % self.map.width as usize) as i32;
            let y = (*spawn.0 / self.map.width as usize) as i32;
            spawn_specific_on_point(ecs, (x, y), spawn.1);
        }
    }

    fn get_map(&mut self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&mut self) -> Position {
        self.starting_position.clone()
    }
}

impl WizardLevelBuilder {
    pub fn new(depth: i32) -> WizardLevelBuilder {
        WizardLevelBuilder {
            map: Map::new(depth),
            starting_position: Position { x: 0, y: 0 },
            start_room: Rect::new(0, 0, 0, 0),
            portal_room: Rect::new(0, 0, 0, 0),
        }
    }
}
