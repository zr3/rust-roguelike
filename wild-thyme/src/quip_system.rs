use crate::{
    components::{Name, Quips},
    gamelog::GameLog,
    particle_system::ParticleBuilder,
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
        WriteExpect<'a, ParticleBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (runstate, mut rng, map, mut log, names, mut quips, positions, mut particle_builder) =
            data;

        if *runstate != RunState::PostTurn {
            return;
        }

        for (name, pos, quips) in (&names, &positions, &mut quips).join() {
            // do nothing if out of view of player
            if !map.visible_tiles[map.xy_idx(pos.x, pos.y)] {
                continue;
            }
            quips.countdown -= 1;
            if quips.countdown > 0 {
                continue;
            }
            // pick random quip and log!
            quips.countdown = rng.roll_dice(1, quips.max_countdown / 2) + (quips.max_countdown / 2);
            if let Some(quip) = rng.random_slice_entry(&quips.quips) {
                log.log(format!("{}: {}", name.name, quip));
                particle_builder.request(
                    pos.x,
                    pos.y,
                    rltk::RGB::named(rltk::WHITE),
                    rltk::RGB::named(rltk::BLACK),
                    rltk::to_cp437('♫'),
                    200.0,
                );
            }
        }
    }
}
