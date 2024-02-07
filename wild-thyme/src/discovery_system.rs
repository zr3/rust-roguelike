use std::collections::{HashMap, HashSet};

use crate::{
    components::{HighlightObject, Name, Position, Rare, SeenByPlayer, VisibleToPlayer},
    gamelog::{GameLog, LogEntry},
    particle_system::ParticleBuilder,
    RunState, UIConfig,
};

use rltk::RandomNumberGenerator;
use specs::prelude::*;

pub struct DiscoverySystem {}

impl<'a> System<'a> for DiscoverySystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, RandomNumberGenerator>,
        WriteExpect<'a, RunState>,
        ReadExpect<'a, UIConfig>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, VisibleToPlayer>,
        ReadStorage<'a, Rare>,
        WriteStorage<'a, SeenByPlayer>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, HighlightObject>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut log,
            mut rng,
            mut runstate,
            ui_config,
            mut particle_builder,
            visible_to_player,
            rares,
            mut seen_by_player,
            mut names,
            mut positions,
            mut highlights,
            entities,
        ) = data;

        let mut seen_sights = HashSet::new();
        for (name, _seen) in (&names, &seen_by_player).join() {
            seen_sights.insert(name.name.clone());
        }
        // let a hashmap of Renderables
        let mut new_sights = HashMap::new();
        for (_visible, name, position) in (&visible_to_player, &names, &positions).join() {
            // add renderable and name to hashmap if not seen yet
            if !seen_sights.contains(&name.name) {
                new_sights.insert(name.name.clone(), (name.clone(), position.clone()));
            }
        }
        // add entity with renderable, name, seenbyplayer
        for (name, position) in new_sights.values() {
            // add entity to track seen items
            let tracking_entity = entities.create();
            let _ = seen_by_player.insert(tracking_entity, SeenByPlayer {});
            let _ = names.insert(tracking_entity, name.clone());
            // add entity to display a tooltip
            let highlight_entity = entities.create();
            let _ = positions.insert(highlight_entity, position.clone());
            let _ = highlights.insert(highlight_entity, HighlightObject {});
            // log to record the new sighting
            log.log(LogEntry::Notification {
                notification: format!("YOU saw {} for the first time.", name.name),
            });
        }
        // set runstate to view stack of new things and log
        if ui_config.highlight_discoveries && !new_sights.is_empty() {
            *runstate = RunState::ActionHighlightObjects {};
        }
        // add shiny fx to rare items
        for (_visible, _rare, pos) in (&visible_to_player, &rares, &positions).join() {
            if rng.roll_dice(1, 4) == 1 {
                particle_builder.request_with_order(
                    pos.x,
                    pos.y,
                    rltk::RGB::from_hex("#e0d080").expect("hardcoded"),
                    rltk::RGB::named(rltk::BLACK),
                    rltk::to_cp437('*'),
                    50.0,
                    -6,
                );
                particle_builder.request_with_order(
                    pos.x,
                    pos.y,
                    rltk::RGB::from_hex("#e0d080").expect("hardcoded"),
                    rltk::RGB::named(rltk::BLACK),
                    rltk::to_cp437('\\'),
                    100.0,
                    -5,
                );
                particle_builder.request_with_order(
                    pos.x,
                    pos.y,
                    rltk::RGB::from_hex("#e0d080").expect("hardcoded"),
                    rltk::RGB::named(rltk::BLACK),
                    rltk::to_cp437('|'),
                    150.0,
                    -4,
                );
                particle_builder.request_with_order(
                    pos.x,
                    pos.y,
                    rltk::RGB::from_hex("#e0d080").expect("hardcoded"),
                    rltk::RGB::named(rltk::BLACK),
                    rltk::to_cp437('/'),
                    200.0,
                    -3,
                );
                particle_builder.request_with_order(
                    pos.x,
                    pos.y,
                    rltk::RGB::from_hex("#e0d080").expect("hardcoded"),
                    rltk::RGB::named(rltk::BLACK),
                    rltk::to_cp437('-'),
                    250.0,
                    -2,
                );
                particle_builder.request_with_order(
                    pos.x,
                    pos.y,
                    rltk::RGB::from_hex("#e0d080").expect("hardcoded"),
                    rltk::RGB::named(rltk::BLACK),
                    rltk::to_cp437('*'),
                    325.0,
                    -1,
                );
            }
        }
    }
}
