use crate::{
    components::{Name, Position},
    map::Map,
};

use super::{gamelog::GameLog, CombatStats, Player, Renderable, RunState, SufferDamage};
use rltk::RGB;
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, Map>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage, positions, mut map, entities) = data;

        for (entity, mut stats, damage) in (&entities, &mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
            let pos = positions.get(entity);
            if let Some(pos) = pos {
                let idx = map.xy_idx(pos.x, pos.y);
                map.bloodstains.insert(idx);
            }
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
                let names = ecs.read_storage::<Name>();
                let player = players.get(entity);
                match player {
                    None => {
                        let victim_name = names.get(entity);
                        let mut log = ecs.fetch_mut::<GameLog>();
                        if let Some(victim_name) = victim_name {
                            log.entries.push(format!("{} is dead", &victim_name.name));
                        }
                        dead.push(entity)
                    }
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
