use crate::components::*;
use rltk::RGB;
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

pub fn bear_trap(ecs: &mut World, x: i32, y: i32) {
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

pub fn pitfall(ecs: &mut World, x: i32, y: i32) {
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

pub fn berry_bush(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('♣'),
            fg: RGB::from_hex("#804080").expect("hardcoded"),
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

pub fn bird_nest(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
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

pub fn ostrich_nest(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
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

pub fn dino_nest(ecs: &mut World, x: i32, y: i32) {
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
