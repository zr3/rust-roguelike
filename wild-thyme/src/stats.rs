use std::cmp::{max, min};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct OverallStats {
    pub name: String,
    pub deepest_level: i32,
    pub most_items_held: i32,
    pub thyme_eaten: i32,
    pub min_hp: i32,
    pub critters_killed: i32,
    pub monsters_killed: i32,
    pub traps_triggered: i32,
    pub portals_taken: i32,
    pub steps_taken: i32,
    pub cake: CakeStats,
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct LevelStats {
    pub level: i32,
    pub items_held: i32,
    pub thyme_eaten: i32,
    pub min_hp: i32,
    pub current_hp: i32,
    pub critters_killed: i32,
    pub monsters_killed: i32,
    pub traps_triggered: i32,
    pub steps_taken: i32,
    pub waits_taken: i32,
    pub food_eaten: i32,
    pub hunger_steps: i32,
    pub starving_steps: i32,
    pub well_fed_steps: i32,
}

impl LevelStats {
    pub fn new(level: i32, items_held: i32, current_hp: i32) -> LevelStats {
        LevelStats {
            level,
            items_held,
            thyme_eaten: 0,
            min_hp: current_hp,
            current_hp,
            critters_killed: 0,
            monsters_killed: 0,
            traps_triggered: 0,
            steps_taken: 0,
            waits_taken: 0,
            food_eaten: 0,
            hunger_steps: 0,
            starving_steps: 0,
            well_fed_steps: 0,
        }
    }
    pub fn reset(&mut self, level: i32) {
        self.level = level;
        self.thyme_eaten = 0;
        self.min_hp = self.current_hp;
        self.critters_killed = 0;
        self.monsters_killed = 0;
        self.traps_triggered = 0;
        self.steps_taken = 0;
        self.waits_taken = 0;
        self.food_eaten = 0;
        self.hunger_steps = 0;
        self.starving_steps = 0;
        self.well_fed_steps = 0;
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct CakeStats {
    pub description: String,
    pub overall_points: i32,
    pub moist_points: i32,
    pub sweet_points: i32,
    pub style_points: i32,
    pub hot_points: i32,
    pub mold_points: i32,
    pub edible_points: i32,
}

impl OverallStats {
    pub fn new() -> OverallStats {
        OverallStats {
            name: format!("???"),
            deepest_level: 0,
            most_items_held: 0,
            thyme_eaten: 0,
            min_hp: 30,
            critters_killed: 0,
            monsters_killed: 0,
            traps_triggered: 0,
            portals_taken: 0,
            steps_taken: 0,
            cake: CakeStats {
                description: "".to_string(),
                overall_points: 0,
                moist_points: 0,
                sweet_points: 0,
                style_points: 0,
                hot_points: 0,
                mold_points: 0,
                edible_points: 0,
            },
        }
    }

    pub fn apply_level(&mut self, level_stats: LevelStats) {
        self.deepest_level = max(self.deepest_level, level_stats.level);
        self.most_items_held = max(self.most_items_held, level_stats.items_held);
        self.thyme_eaten += level_stats.thyme_eaten;
        self.min_hp = min(self.min_hp, level_stats.min_hp);
        self.critters_killed += level_stats.critters_killed;
        self.monsters_killed += level_stats.monsters_killed;
        self.traps_triggered += level_stats.traps_triggered;
        self.portals_taken += 1;
        self.steps_taken += level_stats.steps_taken;
    }
}
