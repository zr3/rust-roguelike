use crate::components::*;
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

pub fn rock(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('o'),
            fg: RGB::from_hex("#9090a0").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "ROCK".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 7 })
        .with(CakeIngredient {
            adjective: "HARD".to_string(),
            super_adjective: "GRAVELLY".to_string(),
            overall_points: -1,
            moist_points: 0,
            sweet_points: 0,
            style_points: 1,
            hot_points: 0,
            mold_points: 2,
            edible_points: -1,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn dart_gun(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('/'),
            fg: RGB::from_hex("#a090e0").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "DART GUN".to_string(),
        })
        .with(Item {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn sparkling_powder(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('☼'),
            fg: RGB::from_hex("#a07020").expect("hardcoded"),
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
        .with(CakeIngredient {
            adjective: "SHINY".to_string(),
            super_adjective: "FLAMING".to_string(),
            overall_points: 5,
            moist_points: -1,
            sweet_points: 0,
            style_points: 2,
            hot_points: 1,
            mold_points: -1,
            edible_points: 0,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('☼'),
            fg: RGB::from_hex("#7040a0").expect("hardcoded"),
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
        .with(CakeIngredient {
            adjective: "ODD".to_string(),
            super_adjective: "BIZARRE".to_string(),
            overall_points: 2,
            moist_points: 0,
            sweet_points: 0,
            style_points: 1,
            hot_points: 0,
            mold_points: -1,
            edible_points: 5,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn healing_herbs(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('%'),
            fg: RGB::from_hex("#70c0a0").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 3,
        })
        .with(Name {
            name: "HEALING HERBS".to_string(),
        })
        .with(Item {})
        .with(ProvidesHealing { heal_amount: 8 })
        .with(Consumable {})
        .with(CakeIngredient {
            adjective: "FRESH".to_string(),
            super_adjective: "SPRUCED-UP".to_string(),
            overall_points: 1,
            moist_points: 0,
            sweet_points: 0,
            style_points: 0,
            hot_points: 0,
            mold_points: -1,
            edible_points: 1,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn loot_meat(ecs: &mut World, owner: Entity) {
    let l = meat(ecs);
    let _ = ecs
        .write_storage::<InBackpack>()
        .insert(l, InBackpack { owner });
    let _ = ecs
        .write_storage::<DropsLoot>()
        .insert(owner, DropsLoot { item: l });
}

pub fn pos_meat(ecs: &mut World, x: i32, y: i32) {
    let item = meat(ecs);
    let _ = ecs
        .write_storage::<Position>()
        .insert(item, Position { x, y });
}

pub fn meat(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437('%'),
            fg: RGB::from_hex("#c07070").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 3,
        })
        .with(Name {
            name: "RAW MEAT".to_string(),
        })
        .with(Item {})
        .with(ProvidesHealing { heal_amount: -2 })
        .with(Consumable {})
        .with(CakeIngredient {
            adjective: "SAVORY".to_string(),
            super_adjective: "BEEFY WHOPPER".to_string(),
            overall_points: 1,
            moist_points: 1,
            sweet_points: 0,
            style_points: 0,
            hot_points: 0,
            mold_points: 0,
            edible_points: 1,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn loot_milk(ecs: &mut World, owner: Entity) {
    let l = milk(ecs);
    let _ = ecs
        .write_storage::<InBackpack>()
        .insert(l, InBackpack { owner });
    let _ = ecs
        .write_storage::<DropsLoot>()
        .insert(owner, DropsLoot { item: l });
}

pub fn pos_milk(ecs: &mut World, x: i32, y: i32) {
    let item = milk(ecs);
    let _ = ecs
        .write_storage::<Position>()
        .insert(item, Position { x, y });
}

pub fn milk(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437('¿'),
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
        .with(CakeIngredient {
            adjective: "DELICIOUS".to_string(),
            super_adjective: "CREAMY".to_string(),
            overall_points: 2,
            moist_points: 2,
            sweet_points: 1,
            style_points: 0,
            hot_points: 0,
            mold_points: 0,
            edible_points: 1,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn loot_egg(ecs: &mut World, owner: Entity) {
    let l = egg(ecs);
    let _ = ecs
        .write_storage::<InBackpack>()
        .insert(l, InBackpack { owner });
    let _ = ecs
        .write_storage::<DropsLoot>()
        .insert(owner, DropsLoot { item: l });
}

pub fn pos_egg(ecs: &mut World, x: i32, y: i32) {
    let item = egg(ecs);
    let _ = ecs
        .write_storage::<Position>()
        .insert(item, Position { x, y });
}

pub fn egg(ecs: &mut World) -> Entity {
    ecs.create_entity()
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
        .with(CakeIngredient {
            adjective: "RICH".to_string(),
            super_adjective: "EGGY".to_string(),
            overall_points: 2,
            moist_points: 0,
            sweet_points: 0,
            style_points: 0,
            hot_points: 0,
            mold_points: 0,
            edible_points: 1,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn pointy_stick(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('/'),
            fg: RGB::from_hex("#a08060").expect("hardcoded"),
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
        .with(CakeIngredient {
            adjective: "SHARP".to_string(),
            super_adjective: "EXTRA POINTY".to_string(),
            overall_points: 0,
            moist_points: 0,
            sweet_points: 0,
            style_points: 1,
            hot_points: 0,
            mold_points: 0,
            edible_points: -1,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn cake_knife(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('/'),
            fg: RGB::from_hex("#a0a0c0").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "CAKE KNIFE".to_string(),
        })
        .with(Item {})
        .with(Equippable {
            slot: EquipmentSlot::Melee,
        })
        .with(MeleePowerBonus { power: 4 })
        .with(CakeIngredient {
            adjective: "DANGEROUS".to_string(),
            super_adjective: "DEADLY".to_string(),
            overall_points: -1,
            moist_points: 0,
            sweet_points: 0,
            style_points: 3,
            hot_points: 0,
            mold_points: 0,
            edible_points: -2,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn bark_armor(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('╦'),
            fg: RGB::from_hex("#a08060").expect("hardcoded"),
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
        .with(CakeIngredient {
            adjective: "SHELLED".to_string(),
            super_adjective: "DRY AF".to_string(),
            overall_points: 1,
            moist_points: -2,
            sweet_points: 0,
            style_points: 2,
            hot_points: 0,
            mold_points: 1,
            edible_points: -1,
        })
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
        .with(CakeIngredient {
            adjective: "FRUITY".to_string(),
            super_adjective: "WONDERFUL".to_string(),
            overall_points: 1,
            moist_points: 1,
            sweet_points: 1,
            style_points: 0,
            hot_points: 0,
            mold_points: 0,
            edible_points: 1,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn thyme(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('¥'),
            fg: RGB::from_hex("#70e0a0").expect("hardcoded"),
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
        .with(CakeIngredient {
            adjective: "EXQUISITE".to_string(),
            super_adjective: "EXEMPLARY".to_string(),
            overall_points: 5,
            moist_points: 0,
            sweet_points: 0,
            style_points: 5,
            hot_points: 0,
            mold_points: 0,
            edible_points: 0,
        })
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
            fg: RGB::from_hex("#996040").expect("hardcoded"),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: name.clone() + " MUSHROOM",
        })
        .with(Item {})
        .with(ProvidesFood {})
        .with(Consumable {})
        .with(ProvidesHealing { heal_amount: hp })
        .with(CakeIngredient {
            adjective: name.to_string(),
            super_adjective: "FUNGAL".to_string(),
            overall_points: 1,
            moist_points: 1,
            sweet_points: -1,
            style_points: 0,
            hot_points: 0,
            mold_points: 1,
            edible_points: 0,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn friendly_crow(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('ç'),
            fg: RGB::named(rltk::BLACK),
            bg: RGB::from_hex("#707090").expect("hardcoded"),
            render_order: 2,
        })
        .with(Name {
            name: "FRIENDLY CROW".to_string(),
        })
        .with(Item {})
        .with(MagicMapper {})
        .with(Consumable {})
        .with(CakeIngredient {
            adjective: "DISGUSTING".to_string(),
            super_adjective: "WTF".to_string(),
            overall_points: -3,
            moist_points: 0,
            sweet_points: 0,
            style_points: 0,
            hot_points: 0,
            mold_points: 3,
            edible_points: -3,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn friendly_eagle(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('ë'),
            fg: RGB::named(rltk::BLACK),
            bg: RGB::from_hex("#907080").expect("hardcoded"),
            render_order: 2,
        })
        .with(Name {
            name: "FRIENDLY EAGLE".to_string(),
        })
        .with(Item {})
        .with(TeleportsPlayer { level: 1 })
        .with(Consumable {})
        .with(CakeIngredient {
            adjective: "HORRIBLE".to_string(),
            super_adjective: "EVIL".to_string(),
            overall_points: -5,
            moist_points: 0,
            sweet_points: 0,
            style_points: 0,
            hot_points: 0,
            mold_points: 3,
            edible_points: -3,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
