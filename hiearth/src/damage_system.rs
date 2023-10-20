use super::{gamelog::GameLog, CombatStats, Player, Renderable, RunState, SufferDamage};
use rltk::RGB;
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                let player = players.get(entity);
                match player {
                    None => dead.push(entity),
                    Some(_) => {
                        let mut runwriter = ecs.write_resource::<RunState>();
                        if *runwriter == RunState::GameOver {
                            return;
                        }
                        *runwriter = RunState::GameOver;
                        let mut player_renderables = ecs.write_storage::<Renderable>();
                        let mut pr = player_renderables
                            .get_mut(entity)
                            .expect("player always has a Renderable");
                        pr.glyph = rltk::to_cp437('X');
                        pr.fg = RGB::named(rltk::WHITE);
                        pr.bg = RGB::named(rltk::BLACK);
                        let mut log = ecs.fetch_mut::<GameLog>();
                        log.entries.push("".to_string());
                        log.entries.push("RIP you".to_string());
                    }
                }
            }
        }
    }
    for victim in dead {
        ecs.delete_entity(victim)
            .expect("unable to delete dead entity");
    }
}
