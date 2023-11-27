use rltk::RandomNumberGenerator;
use specs::World;

use crate::{
    components::Position,
    map::{Map, TileType},
    map_builders::common::{
        apply_horizontal_tunnel, apply_room_to_map, apply_vertical_tunnel, release_a_drunk,
    },
    rect::Rect,
    spawner,
};

use super::MapBuilder;

pub struct NestLevelBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
    drunk_spawns: Vec<(i32, i32)>,
}

impl MapBuilder for NestLevelBuilder {
    fn build_map(&mut self) {
        self.rooms_and_corridors();
        let start = self.map.rooms[0].center();
        self.starting_position = Position {
            x: start.0,
            y: start.1,
        };
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        let nest_room = self.map.rooms[1];
        spawner::spawn_nest_room(ecs, &nest_room, self.depth);
        let stairs_room = self.map.rooms[2];
        spawner::spawn_treeportal(ecs, &stairs_room);
        for point in &self.drunk_spawns {
            spawner::spawn_random_on_point(ecs, *point, self.depth);
        }
        for room in self.map.rooms.iter().skip(2) {
            spawner::spawn_room(ecs, room, self.depth);
        }
    }

    fn get_map(&mut self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&mut self) -> Position {
        self.starting_position.clone()
    }
}

impl NestLevelBuilder {
    pub fn new(new_depth: i32) -> NestLevelBuilder {
        NestLevelBuilder {
            map: Map::new(new_depth),
            starting_position: Position { x: 0, y: 0 },
            depth: new_depth,
            drunk_spawns: Vec::new(),
        }
    }
    fn rooms_and_corridors(&mut self) {
        const MAX_ROOMS: i32 = 6;
        const MIN_SIZE: i32 = 3;
        const MAX_SIZE: i32 = 4;
        const DRUNKARD_STEPS: i32 = 250;
        const SPAWN_STEPS: i32 = 80;

        let mut rng = RandomNumberGenerator::new();

        for room_num in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, self.map.width - w - 1) - 1;
            let y = rng.roll_dice(1, self.map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in self.map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                }
            }
            if ok {
                apply_room_to_map(&mut self.map, &new_room);
                let steps = release_a_drunk(
                    &mut self.map,
                    new_room.center(),
                    DRUNKARD_STEPS,
                    &mut rng,
                    SPAWN_STEPS,
                );
                if room_num > 0 {
                    self.drunk_spawns.extend(steps);
                }

                if !self.map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.map.rooms[self.map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                        let steps = release_a_drunk(
                            &mut self.map,
                            (new_x, prev_y),
                            DRUNKARD_STEPS / 4,
                            &mut rng,
                            DRUNKARD_STEPS / 8,
                        );
                        self.drunk_spawns.extend(steps);
                    } else {
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, new_y);
                        let steps = release_a_drunk(
                            &mut self.map,
                            (prev_x, new_y),
                            DRUNKARD_STEPS / 4,
                            &mut rng,
                            DRUNKARD_STEPS / 8,
                        );
                        self.drunk_spawns.extend(steps);
                    }
                }

                self.map.rooms.push(new_room);
            }
        }

        let stairs_room = self.map.rooms[0];
        let stairs_position = stairs_room.center();
        let stairs_idx = self.map.xy_idx(stairs_position.0, stairs_position.1);
        self.map.tiles[stairs_idx] = TileType::DownStairs;
    }
}
