use specs::World;

use crate::{
    components::Position,
    map::{Map, TileType},
    map_builders::common::{apply_horizontal_tunnel, apply_room_to_map},
    rect::Rect,
    spawner,
};

use super::MapBuilder;

pub struct TownLevelBuilder {
    map: Map,
    starting_position: Position,
    start_room: Rect,
    portal_room: Rect,
}

impl MapBuilder for TownLevelBuilder {
    fn build_map(&mut self) {
        self.start_room = Rect::new(35, 17, 10, 10);
        apply_room_to_map(&mut self.map, &self.start_room);
        self.map.rooms.push(self.start_room);

        self.portal_room = Rect::new(52, 20, 10, 6);
        apply_room_to_map(&mut self.map, &self.portal_room);
        apply_horizontal_tunnel(&mut self.map, 45, 52, 20);
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
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        spawner::spawn_treeportal(ecs, &self.portal_room);
    }

    fn get_map(&mut self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&mut self) -> Position {
        self.starting_position.clone()
    }
}

impl TownLevelBuilder {
    pub fn new() -> TownLevelBuilder {
        TownLevelBuilder {
            map: Map::new(1),
            starting_position: Position { x: 0, y: 0 },
            start_room: Rect::new(0, 0, 0, 0),
            portal_room: Rect::new(0, 0, 0, 0),
        }
    }
}
