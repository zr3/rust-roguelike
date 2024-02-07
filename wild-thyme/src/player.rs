use crate::{
    calculate_cake,
    components::{
        CakeIngredient, Confusion, EntityMoved, GoodThyme, HungerClock, HungerState, Monster,
        WantsToSwap,
    },
    gamelog::LogEntry,
    get_visible_tooltips,
    map::TileType,
    particle_system::ParticleBuilder,
    stats::{LevelStats, OverallStats},
    window_fx, IS_DEBUG_MODE_ACTIVE,
};

use super::{
    gamelog::GameLog, CombatStats, Item, Map, Player, Position, RunState, State, Viewshed,
    WantsToMelee, WantsToPickupItem,
};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let mut entity_moved = ecs.write_storage::<EntityMoved>();
    let mut confused = ecs.write_storage::<Confusion>();
    let mut particle_builder = ecs.fetch_mut::<ParticleBuilder>();
    let mut wants_to_swap = ecs.write_storage::<WantsToSwap>();
    let mobs = ecs.read_storage::<Monster>();

    for (entity, _player, pos, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewsheds).join()
    {
        if let Some(i_am_confused) = confused.get_mut(entity) {
            i_am_confused.turns -= 1;
            if i_am_confused.turns < 1 {
                confused.remove(entity);
            }

            particle_builder.request(
                pos.x,
                pos.y,
                rltk::RGB::named(rltk::PURPLE),
                rltk::RGB::named(rltk::BLACK),
                rltk::to_cp437('?'),
                200.0,
            );

            let mut gamelog = ecs.fetch_mut::<GameLog>();
            gamelog.log(LogEntry::Alert {
                alert: "YOU are CONFUSED and cannot move!".to_string(),
            });
            continue;
        }

        let destination_idx = map.xy_idx(pos.x + dx, pos.y + dy);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("add target failed");
                return;
            } else if let Some(_) = mobs.get(*potential_target) {
                wants_to_swap
                    .insert(*potential_target, WantsToSwap { target: entity })
                    .expect("add target failed");
                return;
            }
        }

        if !map.blocked[destination_idx] {
            pos.x = pos.x + dx;
            pos.y = pos.y + dy;
            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
            entity_moved
                .insert(entity, EntityMoved {})
                .expect("should be able to add movement marker");
            ecs.write_resource::<LevelStats>().steps_taken += 1;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::CoreAwaitingInput,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, &mut gs.ecs)
            }

            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::B => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::N => try_move_player(-1, 1, &mut gs.ecs),

            VirtualKeyCode::I => return RunState::MenuInventory,
            VirtualKeyCode::D => return RunState::MenuDropItem,
            VirtualKeyCode::R => return RunState::MenuRemoveItem,

            VirtualKeyCode::Return => {
                return RunState::ActionShowObjects {
                    current: 0,
                    total: get_visible_tooltips(&gs.ecs).len() as i32,
                }
            }

            VirtualKeyCode::Numpad5 | VirtualKeyCode::Space => {
                if !get_item(&mut gs.ecs) {
                    if try_next_level(&mut gs.ecs) {
                        gs.ecs.fetch_mut::<OverallStats>().portals_taken += 1;
                        return RunState::CoreFadeToNextLevel {
                            level: gs.ecs.fetch::<Map>().depth + 1,
                            row: 0,
                        };
                    } else if try_place_ingredient(&mut gs.ecs) {
                        let log = &mut gs.ecs.fetch_mut::<GameLog>();
                        log.log(LogEntry::Notification {
                            notification: "[d]rop an item here to use it in your cake!".to_string(),
                        });
                    } else if try_judge_cake(&mut gs.ecs) {
                        calculate_cake(&mut gs.ecs);
                        let log = &mut gs.ecs.fetch_mut::<GameLog>();
                        log.log(LogEntry::Notification {
                            notification: "...".to_string(),
                        });
                        log.log(LogEntry::Notification {
                            notification: "...".to_string(),
                        });
                        log.log(LogEntry::Notification {
                            notification: "...".to_string(),
                        });
                        log.log(LogEntry::Notification {
                            notification: "...".to_string(),
                        });
                        log.log(LogEntry::Alert {
                            alert: "you did it! the cake is baking..".to_string(),
                        });
                        window_fx::player_won_effect(&gs.ecs.fetch::<OverallStats>());
                        return RunState::OuterCakeReveal {
                            row: 0,
                            iteration: 0,
                        };
                    } else {
                        skip_turn(&mut gs.ecs);
                    }
                }
            }

            VirtualKeyCode::Q => {
                if IS_DEBUG_MODE_ACTIVE {
                    window_fx::narrate(
                        &gs.ecs.fetch::<OverallStats>(),
                        &gs.ecs.fetch::<LevelStats>(),
                    );
                }
                return RunState::CoreAwaitingInput;
            }

            _ => return RunState::CoreAwaitingInput,
        },
    }
    RunState::CorePlayerTurn
}

fn skip_turn(ecs: &mut World) -> RunState {
    {
        let player_entity = ecs.fetch::<Entity>();
        let viewshed_components = ecs.read_storage::<Viewshed>();
        let monsters = ecs.read_storage::<Monster>();

        let worldmap_resource = ecs.fetch::<Map>();

        let mut can_heal = true;
        let viewshed = viewshed_components
            .get(*player_entity)
            .expect("player should have viewshed");
        for tile in viewshed.visible_tiles.iter() {
            let idx = worldmap_resource.xy_idx(tile.x, tile.y);
            for entity_id in worldmap_resource.tile_content[idx].iter() {
                let mob = monsters.get(*entity_id);
                if let Some(_) = mob {
                    can_heal = false;
                }
            }
        }
        let hunger_clocks = ecs.read_storage::<HungerClock>();
        let hc = hunger_clocks.get(*player_entity);
        if let Some(hc) = hc {
            match hc.state {
                HungerState::Hungry | HungerState::Starving => can_heal = false,
                _ => {}
            }
        }

        if can_heal {
            let mut health_components = ecs.write_storage::<CombatStats>();
            let player_hp = health_components
                .get_mut(*player_entity)
                .expect("player should have hp");
            player_hp.hp = i32::min(player_hp.hp + 1, player_hp.max_hp);
        }
    }
    {
        ecs.get_mut::<LevelStats>()
            .expect("level stats should always exist")
            .waits_taken += 1;
    }

    RunState::CorePlayerTurn
}

fn try_next_level(ecs: &mut World) -> bool {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    map.tiles[player_idx] == TileType::DownStairs
}

fn try_place_ingredient(ecs: &mut World) -> bool {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    map.tiles[player_idx] == TileType::IngredientTable
}

fn try_judge_cake(ecs: &mut World) -> bool {
    let log = &mut ecs.fetch_mut::<GameLog>();
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    if map.tiles[player_idx] != TileType::JudgeCake {
        return false;
    }
    let good_thymes = ecs.read_storage::<GoodThyme>();
    let positions = ecs.read_storage::<Position>();
    if !(&good_thymes, &positions)
        .join()
        .any(|(_, pos)| map.tiles[map.xy_idx(pos.x, pos.y)] == TileType::IngredientTable)
    {
        log.log(LogEntry::Notification {
            notification: format!("[d]rop your CAKE INGREDIENTS on the pedestals above.."),
        });
        log.log(LogEntry::Notification {
            notification: format!("some GOOD THYME is needed for a cake before it can be judged!"),
        });
        return false;
    }
    let ingredients = ecs.read_storage::<CakeIngredient>();
    let num_ingredients = (&ingredients, &positions).join().fold(0, |acc, (_, pos)| {
        if map.tiles[map.xy_idx(pos.x, pos.y)] != TileType::IngredientTable {
            return acc;
        } else {
            return acc + 1;
        }
    });
    if num_ingredients < 3 {
        log.log(LogEntry::Notification {
            notification: format!("[d]rop your CAKE INGREDIENTS on the pedestals above.."),
        });
        log.log(LogEntry::Notification {
            notification: format!("a good CAKE needs at least 3 INGREDIENTS!"),
        });
        return false;
    }
    return true;
}

fn get_item(ecs: &mut World) -> bool {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    if let Some(item) = target_item {
        let mut pickup = ecs.write_storage::<WantsToPickupItem>();
        pickup
            .insert(
                *player_entity,
                WantsToPickupItem {
                    collected_by: *player_entity,
                    item,
                },
            )
            .expect("should be able to insert WantsToPickupItem for player");
        return true;
    } else {
        return false;
    }
}
