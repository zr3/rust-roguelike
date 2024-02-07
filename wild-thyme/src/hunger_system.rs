use specs::prelude::*;

use crate::{
    components::{HungerClock, HungerState, Position, SufferDamage},
    gamelog::{GameLog, LogEntry},
    particle_system::ParticleBuilder,
    window_fx, RunState,
};

pub struct HungerSystem {}

impl<'a> System<'a> for HungerSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, HungerClock>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        WriteStorage<'a, SufferDamage>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, ParticleBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut hunger_clock,
            player_entity,
            runstate,
            mut inflict_damage,
            mut log,
            positions,
            mut particle_builder,
        ) = data;

        for (entity, mut clock) in (&entities, &mut hunger_clock).join() {
            let mut proceed = false;

            match *runstate {
                RunState::CorePlayerTurn => {
                    if entity == *player_entity {
                        proceed = true;
                    }
                }
                RunState::CoreMonsterTurn => {
                    if entity != *player_entity {
                        proceed = true;
                    }
                }
                _ => proceed = false,
            }

            if proceed {
                clock.duration -= 1;
                if clock.duration < 1 {
                    match clock.state {
                        HungerState::Full => {
                            clock.state = HungerState::Normal;
                            clock.duration = 200;
                            if entity == *player_entity {
                                log.log(LogEntry::Notification {
                                    notification: "YOU are no longer well fed.".to_string(),
                                });
                            }
                        }
                        HungerState::Normal => {
                            clock.state = HungerState::Hungry;
                            clock.duration = 200;
                            if entity == *player_entity {
                                log.log(LogEntry::Notification {
                                    notification: "YOU are hungry.".to_string(),
                                });
                            }
                        }
                        HungerState::Hungry => {
                            clock.state = HungerState::Starving;
                            if entity == *player_entity {
                                log.log(LogEntry::Alert {
                                    alert: "YOU are STARVING!".to_string(),
                                });
                                let pos = positions
                                    .get(*player_entity)
                                    .expect("player should always have pos");
                                particle_builder.request(
                                    pos.x,
                                    pos.y,
                                    rltk::RGB::named(rltk::TOMATO),
                                    rltk::RGB::named(rltk::BLACK),
                                    rltk::to_cp437('‼'),
                                    150.0,
                                );
                                window_fx::nudge_effect();
                            }
                        }
                        HungerState::Starving => {
                            if entity == *player_entity {
                                log.log(LogEntry::Alert {
                                    alert: "YOU feel pain from the hunger D:".to_string(),
                                });
                                let pos = positions
                                    .get(*player_entity)
                                    .expect("player should always have pos");
                                particle_builder.request(
                                    pos.x,
                                    pos.y,
                                    rltk::RGB::named(rltk::TOMATO),
                                    rltk::RGB::named(rltk::BLACK),
                                    rltk::to_cp437('‼'),
                                    150.0,
                                );
                            }
                            SufferDamage::new_damage(&mut inflict_damage, entity, 1);
                        }
                    }
                }
            }
        }
    }
}
