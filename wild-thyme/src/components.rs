use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

#[derive(Component, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct VisibleToPlayer {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SeenByPlayer {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct HighlightObject {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Monster {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Herbivore {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct HostileToPlayer {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Creature {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Quips {
    pub quips: Vec<String>,
    pub max_countdown: i32,
    pub countdown: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct DropsLoot {
    pub item: Entity,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("unable to insert damage");
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Item {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Rare {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct CakeIngredient {
    pub adjective: String,
    pub super_adjective: String,
    pub overall_points: i32,
    pub moist_points: i32,
    pub sweet_points: i32,
    pub style_points: i32,
    pub hot_points: i32,
    pub mold_points: i32,
    pub edible_points: i32,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct GoodThyme {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Backpack {
    pub capacity: i32,
    pub items: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<rltk::Point>,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToRemoveItem {
    pub item: Entity,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Consumable {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct TeleportsPlayer {
    pub level: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Confusion {
    pub turns: i32,
}

impl Confusion {
    pub fn new_confusion(store: &mut WriteStorage<Confusion>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.turns += amount;
        } else {
            let confused = Confusion { turns: amount };
            store
                .insert(victim, confused)
                .expect("should be able to insert confused status");
        }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct SpawnsMobs {
    pub mob_type: String,
    pub num_mobs: i32,
}

pub struct SerializeMe;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: super::map::Map,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Melee,
    Shield,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Equippable {
    pub slot: EquipmentSlot,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: EquipmentSlot,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct MeleePowerBonus {
    pub power: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct DefenseBonus {
    pub defense: i32,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum HungerState {
    Full,
    Normal,
    Hungry,
    Starving,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct HungerClock {
    pub state: HungerState,
    pub duration: i32,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct ProvidesFood {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct MagicMapper {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Hidden {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct EntryTrigger {
    pub verb: String,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct EntityMoved {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct SingleActivation {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Fog {
    pub lifetime_rounds: i32,
}
