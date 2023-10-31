use std::collections::HashMap;

use crate::components::{
    AreaOfEffect, Confusion, Consumable, DefenseBonus, EquipmentSlot, Equippable, HungerClock,
    InflictsDamage, MagicMapper, MeleePowerBonus, ProvidesFood, ProvidesHealing, Ranged,
    SerializeMe,
};
use crate::random_table::RandomTable;

use super::{
    BlocksTile, CombatStats, Item, Monster, Name, Player, Position, Rect, Renderable, Viewshed,
    MAPWIDTH,
};
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

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

        match spawn.1.as_ref() {
            "CHAMELEON" => chameleon(ecs, x, y),
            "MOSQUITO" => mosquito(ecs, x, y),
            "SPIDER" => spider(ecs, x, y),
            "GHOST" => ghost(ecs, x, y),
            "HEALTH POTION" => health_potion(ecs, x, y),
            "SCROLL of FIREBALL" => fireball_scroll(ecs, x, y),
            "SCROLL of MAGIC MISSILE" => magic_missile_scroll(ecs, x, y),
            "SCROLL of CONFUSION" => confusion_scroll(ecs, x, y),
            "DAGGER" => dagger(ecs, x, y),
            "SHIELD" => shield(ecs, x, y),
            "LONGSWORD" => longsword(ecs, x, y),
            "TOWER SHIELD" => tower_shield(ecs, x, y),
            "FRENCH FRIES" => rations(ecs, x, y),
            "SCROLL of MAGIC MAPPING" => magic_mapping_scroll(ecs, x, y),
            _ => {}
        }
    }
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
            range: 8,
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
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn chameleon(ecs: &mut World, x: i32, y: i32) {
    let color_roll: i32;
    let color: RGB;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        color_roll = rng.roll_dice(1, 2);
        color = match color_roll {
            1 => RGB::from_u8(rng.range(0, 255), rng.range(0, 255), rng.range(0, 255)),
            _ => RGB::named(rltk::PURPLE),
        };
    }
    monster(ecs, x, y, rltk::to_cp437('c'), color, "CHAMELEON")
}
fn mosquito(ecs: &mut World, x: i32, y: i32) {
    monster(
        ecs,
        x,
        y,
        rltk::to_cp437('m'),
        RGB::named(rltk::SADDLE_BROWN),
        "MOSQUITO",
    )
}
fn spider(ecs: &mut World, x: i32, y: i32) {
    monster(
        ecs,
        x,
        y,
        rltk::to_cp437('s'),
        RGB::named(rltk::GREY),
        "SPIDER",
    )
}
fn ghost(ecs: &mut World, x: i32, y: i32) {
    monster(
        ecs,
        x,
        y,
        rltk::to_cp437('g'),
        RGB::named(rltk::MINT_CREAM),
        "GHOST",
    )
}

fn monster<S: ToString>(
    ecs: &mut World,
    x: i32,
    y: i32,
    glyph: rltk::FontCharType,
    fg: rltk::RGB,
    name: S,
) {
    ecs.create_entity()
        .with(Position { x, y })
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
        .with(Monster {})
        .with(Name {
            name: name.to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "SCROLL of MAGIC MISSILE".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "SCROLL of FIREBALL".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(AreaOfEffect { radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::PURPLE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "SCROLL of CONFUSION".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(Confusion { turns: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('¿'),
            fg: RGB::named(rltk::VIOLET_RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 3,
        })
        .with(Name {
            name: "HEALTH POTION".to_string(),
        })
        .with(Item {})
        .with(ProvidesHealing { heal_amount: 8 })
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("CHAMELEON", 1 + map_depth)
        .add("MOSQUITO", 10)
        .add("GHOST", 2 + map_depth)
        .add("SPIDER", 2 + map_depth)
        .add("SCROLL of FIREBALL", 2 + map_depth)
        .add("SCROLL of MAGIC MISSILE", 4)
        .add("SCROLL of CONFUSION", 2 + map_depth)
        .add("HEALTH POTION", 7)
        .add("DAGGER", 3)
        .add("SHIELD", 3)
        .add("LONGSWORD", map_depth - 1)
        .add("TOWER SHIELD", map_depth - 1)
        .add("FRENCH FRIES", 10)
        .add("SCROLL of MAGIC MAPPING", 2)
}

fn dagger(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::IVORY),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "DAGGER".to_string(),
        })
        .with(Item {})
        .with(Equippable {
            slot: EquipmentSlot::Melee,
        })
        .with(MeleePowerBonus { power: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('('),
            fg: RGB::named(rltk::IVORY),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "SHIELD".to_string(),
        })
        .with(Item {})
        .with(Equippable {
            slot: EquipmentSlot::Shield,
        })
        .with(DefenseBonus { defense: 1 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn longsword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "LONGSWORD".to_string(),
        })
        .with(Item {})
        .with(Equippable {
            slot: EquipmentSlot::Melee,
        })
        .with(MeleePowerBonus { power: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn tower_shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('('),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "TOWER SHIELD".to_string(),
        })
        .with(Item {})
        .with(Equippable {
            slot: EquipmentSlot::Shield,
        })
        .with(DefenseBonus { defense: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn rations(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('%'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "FRENCH FRIES".to_string(),
        })
        .with(Item {})
        .with(ProvidesFood {})
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn magic_mapping_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::CYAN2),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "SCROLL of MAGIC MAPPING".to_string(),
        })
        .with(Item {})
        .with(MagicMapper {})
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
