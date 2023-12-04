use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub deepest_level: i32,
    pub most_items_held: i32,
    pub thyme_eaten: i32,
    pub min_hp: i32,
    pub mobs_killed: i32,
    pub traps_triggered: i32,
    pub portals_taken: i32,
    pub steps_taken: i32,
    pub cake: CakeStats,
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

impl Stats {
    pub fn new() -> Stats {
        Stats {
            deepest_level: 0,
            most_items_held: 0,
            thyme_eaten: 0,
            min_hp: 30,
            mobs_killed: 0,
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
}
