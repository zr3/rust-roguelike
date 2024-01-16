use crate::stats::{LevelStats, OverallStats};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = windowfx)]
extern "C" {
    fn warp();
    fn nudge();
    fn update_stats(
        deepest_level: i32,
        most_items_held: i32,
        thyme_eaten: i32,
        min_hp: i32,
        mobs_killed: i32,
        traps_triggered: i32,
        portals_taken: i32,
        steps_taken: i32,
        level_stats: JsValue,
    );
    fn player_died(
        deepest_level: i32,
        most_items_held: i32,
        thyme_eaten: i32,
        min_hp: i32,
        mobs_killed: i32,
        traps_triggered: i32,
        portals_taken: i32,
        steps_taken: i32,
    );
    fn player_won(
        deepest_level: i32,
        most_items_held: i32,
        thyme_eaten: i32,
        min_hp: i32,
        mobs_killed: i32,
        traps_triggered: i32,
        portals_taken: i32,
        steps_taken: i32,
        description: String,
        overall_points: i32,
        moist_points: i32,
        sweet_points: i32,
        style_points: i32,
        hot_points: i32,
        mold_points: i32,
        edible_points: i32,
    );
}

pub fn warp_effect() {
    #[allow(unused_unsafe)]
    unsafe {
        warp();
    }
}

pub fn nudge_effect() {
    #[allow(unused_unsafe)]
    unsafe {
        nudge();
    }
}

pub fn narrate(stats: &OverallStats, level_stats: &LevelStats) {
    let serialized_level_stats =
        serde_wasm_bindgen::to_value(&level_stats).expect("level stats should all be serializable");
    #[allow(unused_unsafe)]
    unsafe {
        update_stats(
            stats.deepest_level,
            stats.most_items_held,
            stats.thyme_eaten,
            stats.min_hp,
            stats.critters_killed,
            stats.traps_triggered,
            stats.portals_taken,
            stats.steps_taken,
            serialized_level_stats,
        );
    }
}

pub fn player_died_effect(stats: &OverallStats) {
    #[allow(unused_unsafe)]
    unsafe {
        player_died(
            stats.deepest_level,
            stats.most_items_held,
            stats.thyme_eaten,
            stats.min_hp,
            stats.critters_killed,
            stats.traps_triggered,
            stats.portals_taken,
            stats.steps_taken,
        );
    }
}

pub fn player_won_effect(stats: &OverallStats) {
    #[allow(unused_unsafe)]
    unsafe {
        player_won(
            stats.deepest_level,
            stats.most_items_held,
            stats.thyme_eaten,
            stats.min_hp,
            stats.critters_killed,
            stats.traps_triggered,
            stats.portals_taken,
            stats.steps_taken,
            stats.cake.description.clone(),
            stats.cake.overall_points,
            stats.cake.moist_points,
            stats.cake.sweet_points,
            stats.cake.style_points,
            stats.cake.hot_points,
            stats.cake.mold_points,
            stats.cake.edible_points,
        );
    }
}
