use crate::stats::Stats;
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
    unsafe {
        warp();
    }
}

pub fn nudge_effect() {
    unsafe {
        nudge();
    }
}

pub fn narrate(stats: &Stats) {
    unsafe {
        update_stats(
            stats.deepest_level,
            stats.most_items_held,
            stats.thyme_eaten,
            stats.min_hp,
            stats.mobs_killed,
            stats.traps_triggered,
            stats.portals_taken,
            stats.steps_taken,
        );
    }
}

pub fn player_died_effect(stats: &Stats) {
    unsafe {
        player_died(
            stats.deepest_level,
            stats.most_items_held,
            stats.thyme_eaten,
            stats.min_hp,
            stats.mobs_killed,
            stats.traps_triggered,
            stats.portals_taken,
            stats.steps_taken,
        );
    }
}

pub fn player_won_effect(stats: &Stats) {
    unsafe {
        player_won(
            stats.deepest_level,
            stats.most_items_held,
            stats.thyme_eaten,
            stats.min_hp,
            stats.mobs_killed,
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
