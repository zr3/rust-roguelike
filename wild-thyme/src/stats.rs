use serde::{Deserialize, Serialize};

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub deepest_level: i32,
    pub most_items_held: i32,
    pub thyme_eaten: i32,
    pub mobs_killed: i32,
    pub traps_triggered: i32,
    pub portals_taken: i32,
    pub steps_taken: i32,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            deepest_level: 0,
            most_items_held: 0,
            thyme_eaten: 0,
            mobs_killed: 0,
            traps_triggered: 0,
            portals_taken: 0,
            steps_taken: 0,
        }
    }
}
