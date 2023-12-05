use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};
use std::collections::HashMap;

use crate::components::*;
use crate::map::MAPWIDTH;
use crate::random_table::RandomTable;
use crate::rect::Rect;

pub mod items;
use items::*;
pub mod mobs;
use mobs::*;
mod traps;
use traps::*;

const MAX_MONSTERS: i32 = 4;

pub fn spawn_room(ecs: &mut World, room: &Rect, map_depth: i32) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, String> = HashMap::new();
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_spawns = rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3;

        for _i in 0..num_spawns {
            let mut added = false;
            let mut tries = 0;
            while !added && tries < 20 {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
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
        let x = (*spawn.0 % MAPWIDTH) as i32;
        let y = (*spawn.0 / MAPWIDTH) as i32;
        spawn_specific_on_point(ecs, (x, y), spawn.1);
    }
}

pub fn spawn_random_on_point(ecs: &mut World, point: (i32, i32), map_depth: i32) {
    let spawn_table = room_table(map_depth);
    let spawnable;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        spawnable = spawn_table.roll(&mut rng);
    }
    spawn_specific_on_point(ecs, point, &spawnable);
}

pub fn spawn_specific_on_point(ecs: &mut World, point: (i32, i32), spawnable: &String) {
    let x = point.0;
    let y = point.1;
    match spawnable.as_ref() {
        "FRIENDLY CROW" => friendly_crow(ecs, x, y),
        "FRIENDLY EAGLE" => friendly_eagle(ecs, x, y),

        "HEALING HERBS" => healing_herbs(ecs, x, y),
        "GOODBERRY" => goodberry(ecs, x, y),
        "GOOD THYME" => thyme(ecs, x, y),
        "WEIRD CONFUSING POWDER" => confusion_scroll(ecs, x, y),
        "SPARKLING POWDER" => sparkling_powder(ecs, x, y),

        "BEAR TRAP" => bear_trap(ecs, x, y),
        "PITFALL" => pitfall(ecs, x, y),

        "DART GUN" => dart_gun(ecs, x, y),
        "POINTY STICK" => pointy_stick(ecs, x, y),
        "BARK ARMOR" => bark_armor(ecs, x, y),
        "BUCKET" => {} // hold liquids, splash liquids
        "TORCH" => {}  // provide light, fire

        "ROCK" => rock(ecs, x, y),

        "PUFFER MUSHROOM" => mushroom(ecs, x, y, "PUFFER".to_string(), 1, 5),
        "MYSTERIOUS MUSHROOM" => mushroom(ecs, x, y, "MAGIC".to_string(), -15, 40),
        "MOREL MUSHROOM" => mushroom(ecs, x, y, "MOREL".to_string(), 10, 15),

        "BERRY BUSH" => berry_bush(ecs, x, y),

        "BIRD NEST" => bird_nest(ecs, x, y),
        "SPARROW" => sparrow(ecs, x, y),
        "OSTRICH NEST" => ostrich_nest(ecs, x, y),
        "OSTRICH" => ostrich(ecs, x, y),
        "DINOSAUR NEST" => dino_nest(ecs, x, y),
        "DILOPHOSAURUS" => dilophosaurus(ecs, x, y),

        "DEER" => deer(ecs, x, y),
        "SQUIRREL" => squirrel(ecs, x, y),
        "FROG" => frog(ecs, x, y),
        "BUTTERFLY" => butterfly(ecs, x, y),
        "MOSQUITO" => mosquito(ecs, x, y),
        "SPIDER" => spider(ecs, x, y),
        "GOAT" => goat(ecs, x, y),
        "COW" => cow(ecs, x, y),
        "GHOST" => ghost(ecs, x, y),
        "REY" => rey(ecs, x, y),
        "PEPPERMINT WHOPPER" => pep(ecs, x, y),
        "FIREFLY" => {} // provides light

        _ => {}
    }
}

pub fn spawn_nest_room(ecs: &mut World, room: &Rect, map_depth: i32) {
    let spawn_table = nest_table(map_depth);
    let roll;
    let x;
    let y;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = spawn_table.roll(&mut rng);
        x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as i32;
        y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as i32;
    }
    match roll.as_ref() {
        "BIRD NEST" => bird_nest(ecs, x, y),
        "OSTRICH NEST" => ostrich_nest(ecs, x, y),
        "DINOSAUR NEST" => dino_nest(ecs, x, y),
        _ => {}
    }
}

pub fn spawn_treeportal(ecs: &mut World, room: &Rect) {
    let center = room.center();
    ecs.create_entity()
        .with(Position {
            x: center.0,
            y: center.1,
        })
        .with(Name {
            name: "TREE PORTAL".to_string(),
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::FORESTGREEN),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 12,
            dirty: true,
        })
        .with(Name {
            name: "YOU".to_string(),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .with(HungerClock {
            state: crate::components::HungerState::Full,
            duration: 20,
        })
        .with(Backpack {
            capacity: 10,
            items: 0,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("FRIENDLY CROW", 2)
        .add("FRIENDLY EAGLE", std::cmp::max(0, map_depth - 3))
        .add("HEALING HERBS", 10)
        .add("GOODBERRY", 4)
        .add("SPARKLING POWDER", 1 + map_depth)
        .add("BERRY BUSH", 5)
        .add("GOOD THYME", 1 + map_depth / 2)
        .add("WEIRD CONFUSING POWDER", 2 + map_depth)
        .add("BEAR TRAP", 3 + map_depth * 2)
        .add("PITFALL", 10)
        .add("POINTY STICK", 3)
        .add("BARK ARMOR", 3)
        .add("ROCK", 15)
        .add("PUFFER MUSHROOM", 5 + map_depth * 2)
        .add("MAGIC MUSHROOM", 1 + map_depth * 2)
        .add("MOREL MUSHROOM", 3 + map_depth * 2)
        .add("DEER", 10)
        .add("FROG", 15)
        .add("SPARROW", 15)
        .add("SQUIRREL", 15)
        .add("COW", 10 + map_depth)
        .add("GOAT", 10 + map_depth)
        .add("SPIDER", 2 + map_depth * 2)
        .add("OSTRICH", 2 + map_depth * 2)
        .add("GHOST", map_depth)
        .add("DILOPHOSAURUS", map_depth)
        .add("REY", map_depth / 2)
        .add("PEPPERMINT WHOPPER", map_depth / 2)
}

fn nest_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("BIRD NEST", 8)
        .add("OSTRICH NEST", 4 + map_depth)
        .add("DINO NEST", map_depth)
}

pub fn npc<S: ToString>(
    ecs: &mut World,
    x: i32,
    y: i32,
    glyph: rltk::FontCharType,
    fg: rltk::RGB,
    name: S,
    quips: Vec<String>,
) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Monster {})
        .with(Renderable {
            glyph,
            fg,
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: name.to_string(),
        })
        .with(BlocksTile {})
        .with(Quips {
            quips,
            max_countdown: 10,
            countdown: 0,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
