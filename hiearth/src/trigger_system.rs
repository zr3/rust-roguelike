use specs::prelude::*;

use crate::{
    components::{
        EntityMoved, EntryTrigger, Hidden, InflictsDamage, Name, Position, SingleActivation,
        SufferDamage,
    },
    gamelog::GameLog,
    map::Map,
    particle_system::ParticleBuilder,
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
        ) = data;

        let mut remove_entities: Vec<Entity> = Vec::new();
        for (entity, mut _entity_moved, pos) in (&entities, &mut entity_moved, &position).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            for entity_id in map.tile_content[idx].iter() {
                if entity != *entity_id {
                    if let Some(_trigger) = entry_trigger.get(*entity_id) {
                        if let Some(name) = names.get(*entity_id) {
                            log.entries.push(format!("{} triggers!", &name.name));
                        }

                        hidden.remove(*entity_id);

                        // inflict damage if needed
                        if let Some(damage) = inflicts_damage.get(*entity_id) {
                            particle_builder.request(
                                pos.x,
                                pos.y,
                                rltk::RGB::named(rltk::ORANGE),
                                rltk::RGB::named(rltk::BLACK),
                                rltk::to_cp437('‼'),
                                200.0,
                            );
                            SufferDamage::new_damage(&mut suffer_damage, entity, damage.damage);
                        }

                        if let Some(_sa) = single_activation.get(*entity_id) {
                            remove_entities.push(*entity_id);
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
