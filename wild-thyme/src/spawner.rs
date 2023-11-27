use std::collections::HashMap;

use crate::components::{
    AreaOfEffect, Confusion, Consumable, Creature, DefenseBonus, DropsLoot, EntryTrigger,
    EquipmentSlot, Equippable, Herbivore, Hidden, HostileToPlayer, HungerClock, InBackpack,
    InflictsDamage, MagicMapper, MeleePowerBonus, ProvidesFood, ProvidesHealing, Quips, Ranged,
    SerializeMe, SingleActivation, SpawnsMobs, TeleportsPlayer,
};
use crate::random_table::RandomTable;

use super::{
    BlocksTile, CombatStats, Item, Monster, Name, Player, Position, Rect, Renderable, Viewshed,
    MAPWIDTH,
};
use rltk::{Point, RandomNumberGenerator, RGB};
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
        "SPARKLING POWDER" => fireball_scroll(ecs, x, y),

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
        "MOSQUITO" => mosquito(ecs, x, y),
        "SPIDER" => spider(ecs, x, y),
        "GOAT" => goat(ecs, x, y),
        "COW" => cow(ecs, x, y),
        "GHOST" => ghost(ecs, x, y),
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
    monster(ecs, x, y, rltk::to_cp437('c'), color, "CHAMELEON", 16, 1, 4)
}
fn mosquito(ecs: &mut World, x: i32, y: i32) {
    monster(
        ecs,
        x,
        y,
        rltk::to_cp437('m'),
        RGB::named(rltk::SADDLE_BROWN),
        "MOSQUITO",
        5,
        1,
        4,
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
        16,
        1,
        4,
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
        16,
        2,
        4,
    )
}
fn sparrow(ecs: &mut World, x: i32, y: i32) {
    monster(
        ecs,
        x,
        y,
        rltk::to_cp437('b'),
        RGB::named(rltk::TAN),
        "ANGRY SPARROW",
        5,
        1,
        3,
    )
}
fn ostrich(ecs: &mut World, x: i32, y: i32) {
    monster(
        ecs,
        x,
        y,
        rltk::to_cp437('b'),
        RGB::named(rltk::SLATEGRAY),
        "ANGRY OSTRICH",
        8,
        1,
        4,
    )
}
fn dilophosaurus(ecs: &mut World, x: i32, y: i32) {
    monster(
        ecs,
        x,
        y,
        rltk::to_cp437('D'),
        RGB::named(rltk::DARK_ORANGE),
        "ANGRY DILOPHOSAURUS",
        8,
        2,
        8,
    )
}

fn monster<S: ToString>(
    ecs: &mut World,
    x: i32,
    y: i32,
    glyph: rltk::FontCharType,
    fg: rltk::RGB,
    name: S,
    hp: i32,
    defense: i32,
    power: i32,
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
        .with(HostileToPlayer {})
        .with(Name {
            name: name.to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: hp,
            hp,
            defense,
            power,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
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
            max_countdown: 5,
            countdown: 3,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn creature<S: ToString>(
    ecs: &mut World,
    x: i32,
    y: i32,
    glyph: rltk::FontCharType,
    fg: rltk::RGB,
    name: S,
) -> Entity {
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
        .with(Creature {})
        .with(Monster {})
        .with(Name {
            name: name.to_string(),
        })
        .with(BlocksTile {})
        .with(Herbivore {})
        .with(CombatStats {
            max_hp: 8,
            hp: 8,
            defense: 0,
            power: 1,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn deer(ecs: &mut World, x: i32, y: i32) {
    let c = creature(
        ecs,
        x,
        y,
        rltk::to_cp437('d'),
        RGB::named(rltk::SADDLE_BROWN),
        "DEER",
    );
    loot_meat(ecs, c);
}

fn squirrel(ecs: &mut World, x: i32, y: i32) {
    let c = creature(
        ecs,
        x,
        y,
        rltk::to_cp437('s'),
        RGB::named(rltk::SADDLE_BROWN),
        "SQUIRREL",
    );
    loot_meat(ecs, c);
}

fn frog(ecs: &mut World, x: i32, y: i32) {
    let c = creature(
        ecs,
        x,
        y,
        rltk::to_cp437('f'),
        RGB::named(rltk::PALE_GREEN),
        "FROG",
    );
    loot_meat(ecs, c);
}

fn goat(ecs: &mut World, x: i32, y: i32) {
    let c = creature(
        ecs,
        x,
        y,
        rltk::to_cp437('g'),
        RGB::from_hex("#888888").expect("hardcoded"),
        "GOAT",
    );
    loot_milk(ecs, c);
}

fn cow(ecs: &mut World, x: i32, y: i32) {
    let c = creature(
        ecs,
        x,
        y,
        rltk::to_cp437('c'),
        RGB::from_hex("#888888").expect("hardcoded"),
        "COW",
    );
    loot_milk(ecs, c);
}

fn rock(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::GREY),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "ROCK".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn dart_gun(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::SADDLE_BROWN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "DART GUN".to_string(),
        })
        .with(Item {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 2 })
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
            name: "WEIRD CONFUSING POWDER".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(Confusion { turns: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn healing_herbs(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('%'),
            fg: RGB::named(rltk::LIME_GREEN),
            bg: RGB::named(rltk::BLACK),
            render_order: 3,
        })
        .with(Name {
            name: "HEALING HERBS".to_string(),
        })
        .with(Item {})
        .with(ProvidesHealing { heal_amount: 8 })
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn loot_meat(ecs: &mut World, owner: Entity) {
    let l = ecs
        .create_entity()
        .with(InBackpack { owner })
        .with(Renderable {
            glyph: rltk::to_cp437('%'),
            fg: RGB::named(rltk::ORANGE_RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 3,
        })
        .with(Name {
            name: "RAW MEAT".to_string(),
        })
        .with(Item {})
        .with(ProvidesHealing { heal_amount: -2 })
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    let _ = ecs
        .write_storage::<DropsLoot>()
        .insert(owner, DropsLoot { item: l });
}

fn loot_milk(ecs: &mut World, owner: Entity) {
    let l = ecs
        .create_entity()
        .with(InBackpack { owner })
        .with(Renderable {
            glyph: rltk::to_cp437('%'),
            fg: RGB::from_hex("#d0d0c0").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 3,
        })
        .with(Name {
            name: "MILK".to_string(),
        })
        .with(Item {})
        .with(ProvidesHealing { heal_amount: 2 })
        .with(ProvidesFood {})
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    let _ = ecs
        .write_storage::<DropsLoot>()
        .insert(owner, DropsLoot { item: l });
}

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("FRIENDLY CROW", 2)
        .add("FRIENDLY EAGLE", 100)
        .add("HEALING HERBS", 10)
        .add("GOODBERRY", 4)
        .add("SPARKLING POWDER", 1 + map_depth)
        .add("BERRY BUSH", 5)
        .add("GOOD THYME", 100 + map_depth / 2)
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
        .add("SPARROW", 15)
        .add("SQUIRREL", 15)
        .add("SPIDER", 2 + map_depth * 2)
        .add("OSTRICH", 2 + map_depth * 2)
        .add("GHOST", map_depth)
        .add("DILOPHOSAURUS", map_depth)
}

fn nest_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("BIRD NEST", 8)
        .add("OSTRICH NEST", 4 + map_depth)
        .add("DINO NEST", map_depth)
}

fn pointy_stick(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::BROWN1),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "POINTY STICK".to_string(),
        })
        .with(Item {})
        .with(Equippable {
            slot: EquipmentSlot::Melee,
        })
        .with(MeleePowerBonus { power: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn bark_armor(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('('),
            fg: RGB::named(rltk::BROWN1),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "BARK ARMOR".to_string(),
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

fn goodberry(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('%'),
            fg: RGB::named(rltk::POWDER_BLUE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "GOODBERRY".to_string(),
        })
        .with(Item {})
        .with(ProvidesFood {})
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn thyme(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('%'),
            fg: RGB::named(rltk::LIME_GREEN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "GOOD THYME".to_string(),
        })
        .with(Item {})
        .with(ProvidesFood {})
        .with(Consumable {})
        .with(ProvidesHealing { heal_amount: 100 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn mushroom(ecs: &mut World, x: i32, y: i32, name: String, low_hp: i32, high_hp: i32) {
    let hp = ecs
        .get_mut::<RandomNumberGenerator>()
        .expect("rng should always be available")
        .roll_dice(1, high_hp - low_hp)
        + low_hp;
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('♣'),
            fg: RGB::named(rltk::BEIGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: name + " MUSHROOM",
        })
        .with(Item {})
        .with(ProvidesFood {})
        .with(Consumable {})
        .with(ProvidesHealing { heal_amount: hp })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn friendly_crow(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('ç'),
            fg: RGB::named(rltk::BLACK),
            bg: RGB::named(rltk::WHITE),
            render_order: 2,
        })
        .with(Name {
            name: "FRIENDLY CROW".to_string(),
        })
        .with(Item {})
        .with(MagicMapper {})
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn friendly_eagle(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('ë'),
            fg: RGB::named(rltk::BLACK),
            bg: RGB::named(rltk::WHITE),
            render_order: 2,
        })
        .with(Name {
            name: "FRIENDLY EAGLE".to_string(),
        })
        .with(Item {})
        .with(TeleportsPlayer { level: 1 })
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn bear_trap(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('^'),
            fg: RGB::named(rltk::STEELBLUE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "BEAR TRAP".to_string(),
        })
        .with(Hidden {})
        .with(EntryTrigger {
            verb: "springs".to_string(),
        })
        .with(InflictsDamage { damage: 6 })
        .with(SingleActivation {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn pitfall(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('^'),
            fg: RGB::from_hex("#888822").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "PITFALL".to_string(),
        })
        .with(Hidden {})
        .with(EntryTrigger {
            verb: "opens up".to_string(),
        })
        .with(Confusion { turns: 4 })
        .with(InflictsDamage { damage: 1 })
        .with(SingleActivation {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn berry_bush(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('♣'),
            fg: RGB::named(rltk::FORESTGREEN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "BUSH".to_string(),
        })
        .with(EntryTrigger {
            verb: "breaks and GOODBERRIES scatter".to_string(),
        })
        .with(SpawnsMobs {
            mob_type: "GOODBERRY".to_string(),
            num_mobs: 5,
        })
        .with(SingleActivation {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn bird_nest(ecs: &mut World, x: i32, y: i32) {
    let m = ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('o'),
            fg: RGB::named(rltk::SADDLE_BROWN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "NEST".to_string(),
        })
        .with(EntryTrigger {
            verb: "breaks! and ANGRY SPARROWS appear".to_string(),
        })
        .with(SpawnsMobs {
            mob_type: "SPARROW".to_string(),
            num_mobs: 5,
        })
        .with(SingleActivation {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn ostrich_nest(ecs: &mut World, x: i32, y: i32) {
    let m = ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('o'),
            fg: RGB::named(rltk::LIGHT_SLATE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "NEST".to_string(),
        })
        .with(EntryTrigger {
            verb: "breaks! and ANGRY OSTRICHES appear".to_string(),
        })
        .with(SpawnsMobs {
            mob_type: "OSTRICH".to_string(),
            num_mobs: 3,
        })
        .with(SingleActivation {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn dino_nest(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('o'),
            fg: RGB::named(rltk::DARK_ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "NEST".to_string(),
        })
        .with(EntryTrigger {
            verb: "breaks! and ANGRY DINOS appear".to_string(),
        })
        .with(SpawnsMobs {
            mob_type: "DILOPHOSAURUS".to_string(),
            num_mobs: 3,
        })
        .with(SingleActivation {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
