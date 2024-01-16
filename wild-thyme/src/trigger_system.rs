use std::cmp;

use rltk::RandomNumberGenerator;
use specs::prelude::*;

use crate::{
    components::{
        Confusion, EntityMoved, EntryTrigger, Hidden, InflictsDamage, Name, Position,
        SingleActivation, SpawnsMobs, SufferDamage, VisibleToPlayer,
    },
    gamelog::GameLog,
    map::Map,
    particle_system::ParticleBuilder,
    spawn_system::SpawnBuilder,
    stats::OverallStats,
};

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, EntityMoved>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, EntryTrigger>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, Name>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, InflictsDamage>,
        WriteStorage<'a, SufferDamage>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, SingleActivation>,
        WriteStorage<'a, Confusion>,
        ReadStorage<'a, SpawnsMobs>,
        WriteExpect<'a, SpawnBuilder>,
        WriteExpect<'a, RandomNumberGenerator>,
        WriteExpect<'a, OverallStats>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, VisibleToPlayer>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            mut entity_moved,
            position,
            entry_trigger,
            mut hidden,
            names,
            entities,
            mut log,
            inflicts_damage,
            mut suffer_damage,
            mut particle_builder,
            single_activation,
            mut confusion,
            spawns_mobs,
            mut spawn_builder,
            mut rng,
            mut stats,
            player,
            visible_to_player,
        ) = data;

        let mut remove_entities: Vec<Entity> = Vec::new();
        for (triggering_entity, mut _entity_moved, pos) in
            (&entities, &mut entity_moved, &position).join()
        {
            let idx = map.xy_idx(pos.x, pos.y);
            for triggered_entity in map.tile_content[idx].iter() {
                if triggering_entity == *triggered_entity {
                    continue;
                }
                if let Some(trigger) = entry_trigger.get(*triggered_entity) {
                    if triggering_entity == *player {
                        stats.traps_triggered += 1;
                    }
                    let log_suffix;
                    if visible_to_player.get(*triggered_entity).is_some() {
                        log_suffix = "!";
                    } else {
                        log_suffix = " somewhere, beyond the trees..."
                    }
                    if let Some(name) = names.get(*triggered_entity) {
                        log.log(format!("{} {}{}", &name.name, &trigger.verb, log_suffix));
                    }

                    hidden.remove(*triggered_entity);

                    // inflict damage if needed
                    if let Some(damage) = inflicts_damage.get(*triggered_entity) {
                        particle_builder.request(
                            pos.x,
                            pos.y,
                            rltk::RGB::named(rltk::ORANGE),
                            rltk::RGB::named(rltk::BLACK),
                            rltk::to_cp437('‼'),
                            200.0,
                        );
                        SufferDamage::new_damage(
                            &mut suffer_damage,
                            triggering_entity,
                            damage.damage,
                        );
                    }

                    // inflict confusion if needed
                    let mut turns = 0;
                    if let Some(confused) = confusion.get(*triggered_entity) {
                        turns = confused.turns;
                    }
                    if turns > 0 {
                        particle_builder.request(
                            pos.x,
                            pos.y,
                            rltk::RGB::named(rltk::ORANGE),
                            rltk::RGB::named(rltk::BLACK),
                            rltk::to_cp437('?'),
                            200.0,
                        );

                        Confusion::new_confusion(&mut confusion, triggering_entity, turns);
                    }

                    // spawn things if needed
                    if let Some(spawns_mobs) = spawns_mobs.get(*triggered_entity) {
                        particle_builder.request(
                            pos.x,
                            pos.y,
                            rltk::RGB::named(rltk::ORANGE),
                            rltk::RGB::named(rltk::BLACK),
                            rltk::to_cp437('‼'),
                            200.0,
                        );

                        let mut spawn_points = Vec::new();
                        for nx in cmp::max(0, pos.x - 2)..cmp::min(map.width - 1, pos.x + 2) {
                            for ny in cmp::max(0, pos.y - 2)..cmp::min(map.height - 1, pos.y + 2) {
                                if !map.blocked[map.xy_idx(nx, ny)] {
                                    spawn_points.push((nx, ny));
                                }
                            }
                        }
                        for _ in 0..spawns_mobs.num_mobs {
                            if let Some(idx) = rng.random_slice_index(&spawn_points) {
                                let (x, y) = spawn_points[idx];
                                spawn_builder.request(x, y, spawns_mobs.mob_type.clone());
                                particle_builder.request(
                                    x,
                                    y,
                                    rltk::RGB::named(rltk::ORANGE),
                                    rltk::RGB::named(rltk::BLACK),
                                    rltk::to_cp437('‼'),
                                    200.0,
                                );
                                spawn_points.remove(idx);
                            }
                        }

                        // single activation
                        if let Some(_sa) = single_activation.get(*triggered_entity) {
                            remove_entities.push(*triggered_entity);
                        }
                    }
                }
            }
        }

        for trap in remove_entities.iter() {
            entities
                .delete(*trap)
                .expect("trigger system should be able to delete trap entity");
        }

        entity_moved.clear();
    }
}
