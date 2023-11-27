use crate::{
    components::{Name, Quips},
    gamelog::GameLog,
    RunState,
};

use super::{Map, Position};
use specs::prelude::*;

pub struct QuipSystem {}

impl<'a> System<'a> for QuipSystem {
    type SystemData = (
        ReadExpect<'a, RunState>,
        WriteExpect<'a, rltk::RandomNumberGenerator>,
        ReadExpect<'a, Map>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Quips>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (runstate, mut rng, map, mut log, names, mut quips, positions) = data;

        if *runstate != RunState::PostTurn {
            return;
        }

        for (name, position, quips) in (&names, &positions, &mut quips).join() {
            // do nothing if out of view of player
            if !map.visible_tiles[map.xy_idx(position.x, position.y)] {
                continue;
            }
            quips.countdown -= 1;
            if quips.countdown > 0 {
                continue;
            }
            // pick random quip and log!
            quips.countdown = quips.max_countdown;
            if let Some(quip) = rng.random_slice_entry(&quips.quips) {
                log.entries.push(format!("{}: {}", name.name, quip));
            }
        }
    }
}
