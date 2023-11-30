use crate::components::*;
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

pub fn rock(ecs: &mut World, x: i32, y: i32) {
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

pub fn dart_gun(ecs: &mut World, x: i32, y: i32) {
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

pub fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "SPARKLING POWDER".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(AreaOfEffect { radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
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

pub fn healing_herbs(ecs: &mut World, x: i32, y: i32) {
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

pub fn loot_meat(ecs: &mut World, owner: Entity) {
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

pub fn loot_milk(ecs: &mut World, owner: Entity) {
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

pub fn loot_egg(ecs: &mut World, owner: Entity) {
    let l = ecs
        .create_entity()
        .with(InBackpack { owner })
        .with(Renderable {
            glyph: rltk::to_cp437('0'),
            fg: RGB::from_hex("#bbccbb").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 3,
        })
        .with(Name {
            name: "EGG".to_string(),
        })
        .with(Item {})
        .with(ProvidesHealing { heal_amount: 10 })
        .with(ProvidesFood {})
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    let _ = ecs
        .write_storage::<DropsLoot>()
        .insert(owner, DropsLoot { item: l });
}

pub fn pointy_stick(ecs: &mut World, x: i32, y: i32) {
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

pub fn bark_armor(ecs: &mut World, x: i32, y: i32) {
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

pub fn goodberry(ecs: &mut World, x: i32, y: i32) {
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

pub fn thyme(ecs: &mut World, x: i32, y: i32) {
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
        .with(GoodThyme {})
        .with(ProvidesFood {})
        .with(Consumable {})
        .with(ProvidesHealing { heal_amount: 100 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn mushroom(ecs: &mut World, x: i32, y: i32, name: String, low_hp: i32, high_hp: i32) {
    let hp = ecs
        .get_mut::<RandomNumberGenerator>()
        .expect("rng should always be available")
        .roll_dice(1, high_hp - low_hp)
        + low_hp;
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('♣'),
            fg: RGB::from_hex("#696040").expect("hardcoded"),
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

pub fn friendly_crow(ecs: &mut World, x: i32, y: i32) {
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

pub fn friendly_eagle(ecs: &mut World, x: i32, y: i32) {
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
