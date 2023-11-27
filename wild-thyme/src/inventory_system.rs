use crate::{
    components::{
        AreaOfEffect, CombatStats, Confusion, Consumable, Equippable, Equipped, HungerClock,
        HungerState, InflictsDamage, MagicMapper, ProvidesFood, ProvidesHealing, SufferDamage,
        TeleportsPlayer, WantsToDropItem, WantsToRemoveItem, WantsToUseItem,
    },
    map::Map,
    particle_system::ParticleBuilder,
    RunState,
};

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

pub struct UseItemSystem {}

impl<'a> System<'a> for UseItemSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, WantsToUseItem>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, ProvidesHealing>,
        ReadStorage<'a, InflictsDamage>,
        ReadStorage<'a, AreaOfEffect>,
        ReadStorage<'a, ProvidesFood>,
        ReadStorage<'a, MagicMapper>,
        WriteStorage<'a, Confusion>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        WriteStorage<'a, HungerClock>,
        ReadStorage<'a, Consumable>,
        ReadStorage<'a, Equippable>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, RunState>,
        ReadStorage<'a, TeleportsPlayer>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut gamelog,
            map,
            entities,
            mut wants_use,
            names,
            healing,
            inflicts_damage,
            aoe,
            provides_food,
            magic_mapper,
            mut confusion,
            mut combat_stats,
            mut suffer_damage,
            mut hunger_clocks,
            consumables,
            equippable,
            mut equipped,
            mut backpack,
            mut particle_builder,
            positions,
            mut runstate,
            teleports_player,
        ) = data;

        for (entity, used_item, stats) in (&entities, &wants_use, &mut combat_stats).join() {
            let mut item_was_used = true;

            // targeting
            let mut targets: Vec<Entity> = Vec::new();
            match used_item.target {
                None => {
                    targets.push(*player_entity);
                }
                Some(target) => {
                    let area_effect = aoe.get(used_item.item);
                    match area_effect {
                        None => {
                            let idx = map.xy_idx(target.x, target.y);
                            for mob in map.tile_content[idx].iter() {
                                targets.push(*mob);
                            }
                        }
                        Some(area_effect) => {
                            let mut blast_tiles =
                                rltk::field_of_view(target, area_effect.radius, &*map);
                            blast_tiles.retain(|p| {
                                p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height - 1
                            });
                            for tile_idx in blast_tiles.iter() {
                                let idx = map.xy_idx(tile_idx.x, tile_idx.y);
                                for mob in map.tile_content[idx].iter() {
                                    targets.push(*mob);
                                }

                                particle_builder.request(
                                    tile_idx.x,
                                    tile_idx.y,
                                    rltk::RGB::named(rltk::ORANGE),
                                    rltk::RGB::named(rltk::BLACK),
                                    rltk::to_cp437('▒'),
                                    350.0,
                                );
                            }
                        }
                    }
                }
            }

            // eqiuppable items
            let item_equippable = equippable.get(used_item.item);
            if let Some(can_equip) = item_equippable {
                let target_slot = can_equip.slot;
                let target = targets[0];

                // remove items the target has in this item's slot
                let mut to_unequip: Vec<Entity> = Vec::new();
                for (item_entity, already_equipped, name) in (&entities, &equipped, &names).join() {
                    if already_equipped.owner == target && already_equipped.slot == target_slot {
                        to_unequip.push(item_entity);
                        if target == *player_entity {
                            gamelog.entries.push(format!("YOU unequip {}.", name.name));
                        }
                    }
                }
                for item in to_unequip.iter() {
                    equipped.remove(*item);
                    backpack
                        .insert(*item, InBackpack { owner: target })
                        .expect("should be able to insert InBackpack for item");
                }

                // wield the item!
                equipped
                    .insert(
                        used_item.item,
                        Equipped {
                            owner: target,
                            slot: target_slot,
                        },
                    )
                    .expect("should be able to equip item");
                backpack.remove(used_item.item);
                if target == *player_entity {
                    gamelog.entries.push(format!(
                        "YOU equip {}.",
                        names
                            .get(used_item.item)
                            .expect("items should have names")
                            .name
                    ));
                }
            }

            // healing items
            let item_heals = healing.get(used_item.item);
            match item_heals {
                None => {}
                Some(healer) => {
                    stats.hp = i32::min(stats.max_hp, stats.hp + healer.heal_amount);
                    if entity == *player_entity {
                        gamelog.entries.push(format!(
                            "The {} healed {} hp!",
                            names.get(used_item.item).unwrap().name,
                            healer.heal_amount
                        ));
                    }
                    item_was_used = true;

                    let pos = positions.get(entity);
                    if let Some(pos) = pos {
                        particle_builder.request(
                            pos.x,
                            pos.y,
                            rltk::RGB::named(rltk::PINK),
                            rltk::RGB::named(rltk::BLACK),
                            rltk::to_cp437('♥'),
                            200.0,
                        );
                    }
                }
            }

            // damaging items
            let item_damages = inflicts_damage.get(used_item.item);
            match item_damages {
                None => {}
                Some(damage) => {
                    item_was_used = false;
                    for mob in targets.iter() {
                        SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
                        if entity == *player_entity {
                            let mob_name = names.get(*mob).expect("targets should have name");
                            let item_name =
                                names.get(used_item.item).expect("items should have name");
                            gamelog.entries.push(format!(
                                "YOU used {} on {}. {} damage!",
                                item_name.name, mob_name.name, damage.damage
                            ));
                        }

                        item_was_used = true;

                        let pos = positions.get(*mob);
                        if let Some(pos) = pos {
                            particle_builder.request(
                                pos.x,
                                pos.y,
                                rltk::RGB::named(rltk::ORANGE),
                                rltk::RGB::named(rltk::BLACK),
                                rltk::to_cp437('‼'),
                                100.0,
                            );
                        }
                    }
                }
            }

            // confusing items
            let mut add_confusion = Vec::new();
            {
                let causes_confusion = confusion.get(used_item.item);
                match causes_confusion {
                    None => {}
                    Some(confusion) => {
                        item_was_used = false;
                        for mob in targets.iter() {
                            add_confusion.push((*mob, confusion.turns));
                            if entity == *player_entity {
                                let mob_name = names.get(*mob).expect("targets should have a name");
                                let item_name =
                                    names.get(used_item.item).expect("items should have a name");
                                gamelog.entries.push(format!(
                                    "YOU used {} on {}, and it is CONFUSED!",
                                    item_name.name, mob_name.name
                                ));
                            }

                            let pos = positions.get(*mob);
                            if let Some(pos) = pos {
                                particle_builder.request(
                                    pos.x,
                                    pos.y,
                                    rltk::RGB::named(rltk::PURPLE),
                                    rltk::RGB::named(rltk::BLACK),
                                    rltk::to_cp437('?'),
                                    200.0,
                                );
                            }
                        }
                    }
                }
            }
            for mob in add_confusion.iter() {
                confusion
                    .insert(mob.0, Confusion { turns: mob.1 })
                    .expect("should be able to insert Confusion");
            }

            // edible items
            let item_edible = provides_food.get(used_item.item);
            match item_edible {
                None => {}
                Some(_) => {
                    let target = targets[0];
                    let hc = hunger_clocks.get_mut(target);
                    if let Some(hc) = hc {
                        hc.state = HungerState::Full;
                        hc.duration = 20;
                        if entity == *player_entity {
                            gamelog.entries.push(format!(
                                "YOU feel satisfied and full after eating the {}",
                                names.get(used_item.item).unwrap().name
                            ));
                        }
                    }
                    item_was_used = true;

                    let pos = positions.get(entity);
                    if let Some(pos) = pos {
                        particle_builder.request(
                            pos.x,
                            pos.y,
                            rltk::RGB::named(rltk::GREEN),
                            rltk::RGB::named(rltk::BLACK),
                            rltk::to_cp437('☺'),
                            1000.0,
                        );
                    }
                }
            }

            // magic mapping items
            let item_maps = magic_mapper.get(used_item.item);
            if let Some(_item_maps) = item_maps {
                gamelog.entries.push(format!("YOU can now SEE this level!"));
                item_was_used = true;
                *runstate = RunState::MagicMapReveal {
                    row: 0,
                    iteration: 0,
                };

                let pos = positions.get(entity);
                if let Some(pos) = pos {
                    particle_builder.request(
                        pos.x,
                        pos.y,
                        rltk::RGB::named(rltk::GREEN),
                        rltk::RGB::named(rltk::BLACK),
                        rltk::to_cp437('!'),
                        2000.0,
                    );
                }
            }

            // teleporting items
            if let Some(_) = teleports_player.get(used_item.item) {
                gamelog
                    .entries
                    .push(format!("YOU are carried to another level!"));
                item_was_used = true;

                let pos = positions.get(entity);
                if let Some(pos) = pos {
                    particle_builder.request(
                        pos.x,
                        pos.y,
                        rltk::RGB::named(rltk::GREEN),
                        rltk::RGB::named(rltk::BLACK),
                        rltk::to_cp437('!'),
                        2000.0,
                    );
                }
            }

            // consume if needed
            if item_was_used {
                let consumable = consumables.get(used_item.item);
                match consumable {
                    None => {}
                    Some(_) => {
                        entities
                            .delete(used_item.item)
                            .expect("item entity should exist if it's getting used");
                    }
                }
            }
        }

        wants_use.clear();
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

pub struct ItemRemoveSystem {}

impl<'a> System<'a> for ItemRemoveSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToRemoveItem>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_remove, mut equipped, mut backpack) = data;

        for (entity, to_remove) in (&entities, &wants_remove).join() {
            equipped.remove(to_remove.item);
            backpack
                .insert(to_remove.item, InBackpack { owner: entity })
                .expect("should be able to add unequipped item to backpack");
        }

        wants_remove.clear();
    }
}
