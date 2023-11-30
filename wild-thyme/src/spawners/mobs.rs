use crate::components::*;
use crate::spawners::items::*;
use rltk::RGB;
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

pub fn mosquito(ecs: &mut World, x: i32, y: i32) {
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
    );
}
pub fn spider(ecs: &mut World, x: i32, y: i32) {
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
    );
}
pub fn ghost(ecs: &mut World, x: i32, y: i32) {
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
    );
}
pub fn sparrow(ecs: &mut World, x: i32, y: i32) {
    let m = monster(
        ecs,
        x,
        y,
        rltk::to_cp437('b'),
        RGB::named(rltk::TAN),
        "ANGRY SPARROW",
        5,
        1,
        3,
    );
    loot_egg(ecs, m);
}
pub fn ostrich(ecs: &mut World, x: i32, y: i32) {
    let m = monster(
        ecs,
        x,
        y,
        rltk::to_cp437('b'),
        RGB::named(rltk::SLATEGRAY),
        "ANGRY OSTRICH",
        8,
        1,
        4,
    );
    loot_egg(ecs, m);
}
pub fn dilophosaurus(ecs: &mut World, x: i32, y: i32) {
    let m = monster(
        ecs,
        x,
        y,
        rltk::to_cp437('D'),
        RGB::named(rltk::DARK_ORANGE),
        "ANGRY DILOPHOSAURUS",
        8,
        2,
        8,
    );
    loot_egg(ecs, m);
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
        .build()
}

pub fn rey(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::from_hex("#a0a020").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 9,
            dirty: true,
        })
        .with(Monster {})
        .with(Quips {
            quips: vec![
                "I'll zap you!".to_string(),
                "one of these days...".to_string(),
            ],
            max_countdown: 10,
            countdown: 0,
        })
        .with(Name {
            name: "REY".to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 10,
            hp: 10,
            defense: 4,
            power: 2,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn pep(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::from_hex("#a04080").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 9,
            dirty: true,
        })
        .with(Monster {})
        .with(Quips {
            quips: vec![
                "wow I sure love pointy things".to_string(),
                "thpthpthptph.. yummy".to_string(),
                "nothing like a goooood burger".to_string(),
                "stabstabstabstabstab".to_string(),
            ],
            max_countdown: 10,
            countdown: 0,
        })
        .with(Name {
            name: "PEPPERMINT WHOPPER MCGILLICUDY III".to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 10,
            hp: 10,
            defense: 4,
            power: 2,
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

pub fn deer(ecs: &mut World, x: i32, y: i32) {
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

pub fn squirrel(ecs: &mut World, x: i32, y: i32) {
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

pub fn frog(ecs: &mut World, x: i32, y: i32) {
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

pub fn butterfly(ecs: &mut World, x: i32, y: i32) {
    let c = creature(
        ecs,
        x,
        y,
        rltk::to_cp437('*'),
        RGB::named(rltk::LAVENDER),
        "BUTTERFLY",
    );
    loot_meat(ecs, c);
}

pub fn goat(ecs: &mut World, x: i32, y: i32) {
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

pub fn cow(ecs: &mut World, x: i32, y: i32) {
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
