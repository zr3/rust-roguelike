use serde::{Deserialize, Serialize};

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub portals_taken: i32,
    pub deepest_level: i32,
    pub mobs_killed: i32,
    pub traps_triggered: i32,
    pub thyme_eaten: i32,
    pub steps_taken: i32,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            deepest_level: 0,
            thyme_eaten: 0,
            mobs_killed: 0,
            portals_taken: 0,
            traps_triggered: 0,
            steps_taken: 0,
        }
    }
}
