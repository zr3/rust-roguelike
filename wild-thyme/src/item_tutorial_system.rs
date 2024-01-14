use std::collections::{HashMap, HashSet};

use crate::{
    components::{HighlightItem, Name, Position, SeenByPlayer, VisibleToPlayer},
    gamelog::GameLog,
    RunState,
};

use specs::prelude::*;

pub struct ItemTutorialSystem {}

impl<'a> System<'a> for ItemTutorialSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, RunState>,
        ReadStorage<'a, VisibleToPlayer>,
        WriteStorage<'a, SeenByPlayer>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, HighlightItem>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut log,
            mut runstate,
            visible_to_player,
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
            let _ = highlights.insert(highlight_entity, HighlightItem {});
            // log to record the new sighting
            log.log(format!("YOU saw {}.", name.name));
        }
        // set runstate to view stack of new things and log
        if !new_sights.is_empty() {
            *runstate = RunState::HighlightItem {};
        }
    }
}
