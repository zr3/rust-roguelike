use std::cmp::min;

use crate::{
    components::{
        CombatStats, EntityMoved, Equipped, HostileToPlayer, HungerClock, HungerState, InBackpack,
    },
    stats::LevelStats,
};
use specs::prelude::*;

pub struct StatsSystem {}

impl<'a> System<'a> for StatsSystem {
    type SystemData = (
        WriteExpect<'a, LevelStats>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, HostileToPlayer>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, InBackpack>,
        ReadStorage<'a, Equipped>,
        ReadStorage<'a, EntityMoved>,
        ReadStorage<'a, HungerClock>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut level_stats,
            player_entity,
            hostiles,
            combat_stats,
            in_backpacks,
            equipped,
            moved_entities,
            hunger_clocks,
        ) = data;
        // current number of player-held items
        level_stats.items_held = equipped
            .join()
            .filter(|&e| e.owner == *player_entity)
            .count() as i32
            + in_backpacks
                .join()
                .filter(|&ib| ib.owner == *player_entity)
                .count() as i32;
        // thyme eaten this round (handled imperatively in inventory system..)
        // level_stats.thyme_eaten += 0;
        // hp stats
        let player_stats = combat_stats
            .get(*player_entity)
            .expect("player should always have combat stats");
        level_stats.min_hp = min(level_stats.min_hp, player_stats.hp);
        level_stats.current_hp = player_stats.hp;
        // kills based on whether hostile to player or not
        level_stats.critters_killed += (&combat_stats, !&hostiles)
            .join()
            .filter(|&c| c.0.hp <= 0)
            .count() as i32;
        level_stats.monsters_killed += (&combat_stats, &hostiles)
            .join()
            .filter(|&c| c.0.hp <= 0)
            .count() as i32;
        // traps handled imperatively in trigger_system..
        // level_stats.traps_triggered += 0;
        // steps based on player moving
        if moved_entities.contains(*player_entity) {
            level_stats.steps_taken += 1;
        }
        // waits handled imperatively in player.rs..
        // level_stats.waits_taken += 0;
        // food eaten this round handled imperatively in inventory system..
        // level_stats.food_eaten += 0;
        match hunger_clocks
            .get(*player_entity)
            .expect("player should always have hunger clock")
            .state
        {
            HungerState::Full => level_stats.well_fed_steps += 1,
            HungerState::Hungry => level_stats.hunger_steps += 1,
            HungerState::Starving => level_stats.starving_steps += 1,
            _ => {}
        }
    }
}
