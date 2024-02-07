use crate::{
    components::{DropsLoot, Herbivore, Name, Position, WantsToDropItem},
    gamelog::LogEntry,
    map::Map,
    stats::{LevelStats, OverallStats},
    window_fx,
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
        ReadStorage<'a, DropsLoot>,
        WriteStorage<'a, WantsToDropItem>,
        WriteExpect<'a, LevelStats>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Herbivore>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut stats,
            mut damage,
            positions,
            mut map,
            entities,
            drops_loot,
            mut wants_to_drop,
            mut level_stats,
            players,
            herbivore,
        ) = data;

        for (entity, mut stats, damage) in (&entities, &mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
            let pos = positions.get(entity);
            if let Some(pos) = pos {
                let idx = map.xy_idx(pos.x, pos.y);
                map.bloodstains.insert(idx);
            }
            if stats.hp <= 0 {
                if let Some(loot) = drops_loot.get(entity) {
                    let _ = wants_to_drop.insert(entity, WantsToDropItem { item: loot.item });
                }
                if herbivore.get(entity).is_some() {
                    level_stats.critters_killed += 1;
                } else if players.get(entity).is_none() {
                    level_stats.monsters_killed += 1;
                }
                window_fx::nudge_effect();
            }
            if let Some(_) = players.get(entity) {
                level_stats.min_hp = std::cmp::min(level_stats.min_hp, stats.hp);
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
                            log.log(LogEntry::Alert {
                                alert: format!("{} is dead", &victim_name.name),
                            });
                        }

                        dead.push(entity);
                    }
                    Some(_) => {
                        let mut runwriter = ecs.write_resource::<RunState>();
                        if *runwriter == RunState::OuterGameOver {
                            return;
                        }
                        *runwriter = RunState::OuterGameOver;
                        let mut player_renderables = ecs.write_storage::<Renderable>();
                        let mut pr = player_renderables
                            .get_mut(entity)
                            .expect("player always has a Renderable");
                        pr.glyph = rltk::to_cp437('X');
                        pr.fg = RGB::named(rltk::WHITE);
                        pr.bg = RGB::named(rltk::BLACK);
                        let mut log = ecs.fetch_mut::<GameLog>();
                        log.log(LogEntry::Notification {
                            notification: "".to_string(),
                        });
                        log.log(LogEntry::Alert {
                            alert: "RIP :(".to_string(),
                        });
                        window_fx::player_died_effect(
                            &ecs.fetch::<OverallStats>(),
                            &ecs.fetch::<LevelStats>(),
                        );
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
