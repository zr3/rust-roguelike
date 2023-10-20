use crate::components::{CombatStats, Potion, WantsToDrinkPotion, WantsToDropItem};

use super::{gamelog::GameLog, InBackpack, Name, Position, WantsToPickupItem};
use specs::prelude::*;

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToPickupItem>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, mut wants_pickup, mut positions, names, mut backpack) =
            data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);
            backpack
                .insert(
                    pickup.item,
                    InBackpack {
                        owner: pickup.collected_by,
                    },
                )
                .expect("should be able to insert backpack entity");

            if pickup.collected_by == *player_entity {
                gamelog.entries.push(format!(
                    "YOU pick up the {}.",
                    names
                        .get(pickup.item)
                        .expect("items should always have Name")
                        .name
                ))
            }
        }

        wants_pickup.clear();
    }
}

pub struct DrinkPotionSystem {}

impl<'a> System<'a> for DrinkPotionSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, WantsToDrinkPotion>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Potion>,
        WriteStorage<'a, CombatStats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut gamelog,
            entities,
            mut wants_drink,
            names,
            potions,
            mut combat_stats,
        ) = data;

        for (entity, drink, stats) in (&entities, &wants_drink, &mut combat_stats).join() {
            let potion = potions.get(drink.potion);
            match potion {
                None => {}
                Some(potion) => {
                    stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);
                    if entity == *player_entity {
                        gamelog.entries.push(format!(
                            "YOU drank the {}, healing {} hp!",
                            names.get(drink.potion).unwrap().name,
                            potion.heal_amount
                        ));
                    }
                    entities
                        .delete(drink.potion)
                        .expect("potion entity should exist if it's getting drunk");
                }
            }
        }

        wants_drink.clear();
    }
}

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, WantsToDropItem>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut gamelog,
            entities,
            mut wants_drop,
            names,
            mut positions,
            mut backpack,
        ) = data;
        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let mut dropper_pos: Position = Position { x: 0, y: 0 };
            {
                let dropped_pos = positions
                    .get(entity)
                    .expect("dropper of item should have a position");
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }
            positions
                .insert(
                    to_drop.item,
                    Position {
                        x: dropper_pos.x,
                        y: dropper_pos.y,
                    },
                )
                .expect("should be able to add position for newly dropped item");
            backpack.remove(to_drop.item);

            if entity == *player_entity {
                gamelog.entries.push(format!(
                    "YOU dropped the {}..",
                    names.get(to_drop.item).unwrap().name
                ));
            }
        }
        wants_drop.clear();
    }
}
