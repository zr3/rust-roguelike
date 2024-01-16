use crate::{
    components::{
        HighlightObject, Ranged, TeleportsPlayer, WantsToDropItem, WantsToRemoveItem,
        WantsToUseItem,
    },
    discovery_system, gui,
    map::{Map, MAPHEIGHT, MAPWIDTH},
    menu,
    player::*,
    saveload_system,
    spawn_system::{SpawnBuilder, SpawnRequest},
    spawners,
    stats::{LevelStats, OverallStats},
    window_fx, RunState, State, UIConfig,
};
use rltk::Rltk;
use specs::prelude::*;

impl State {
    pub fn run_game_loop(&mut self, ctx: &mut Rltk, current_runstate: RunState) -> RunState {
        match current_runstate {
            // core game loop
            RunState::CoreLevelStart => {
                self.run_systems();
                return RunState::CoreAwaitingInput;
            }

            RunState::CorePreRound => {
                let mut discovery = discovery_system::DiscoverySystem {};
                discovery.run_now(&self.ecs);
                return match *self.ecs.fetch::<RunState>() {
                    RunState::ActionHighlightObjects {} => RunState::ActionHighlightObjects {},
                    _ => RunState::CoreAwaitingInput,
                };
            }
            RunState::CoreAwaitingInput => {
                return player_input(self, ctx);
            }
            RunState::CorePlayerTurn => {
                self.run_systems();

                return match *self.ecs.fetch::<RunState>() {
                    RunState::ActionMagicMapReveal { .. } => RunState::ActionMagicMapReveal {
                        row: 0,
                        iteration: 0,
                    },
                    _ => RunState::CoreMonsterTurn,
                };
            }
            RunState::CoreMonsterTurn => {
                self.run_systems();
                return RunState::CorePostRound;
            }
            RunState::CorePostRound => {
                self.run_systems();
                let mut requests = Vec::new();
                {
                    let sb = self.ecs.fetch::<SpawnBuilder>();
                    for new_spawn in sb.requests.iter() {
                        requests.push(SpawnRequest {
                            x: new_spawn.x,
                            y: new_spawn.y,
                            spawn_name: new_spawn.spawn_name.clone(),
                        });
                    }
                }
                for new_spawn in requests {
                    spawners::spawn_specific_on_point(
                        &mut self.ecs,
                        (new_spawn.x, new_spawn.y),
                        &new_spawn.spawn_name,
                    );
                }
                {
                    let sb = self
                        .ecs
                        .get_mut::<SpawnBuilder>()
                        .expect("SpawnBuilder should be permanently registered");
                    sb.requests.clear();
                }
                self.ecs.maintain();
                return RunState::CorePreRound;
            }

            RunState::CoreFadeToNextLevel { level, row } => {
                window_fx::warp_effect();
                let mut map = self.ecs.fetch_mut::<Map>();
                for x in 0..MAPWIDTH as i32 {
                    let idx = map.xy_idx(x as i32, row);
                    map.revealed_tiles[idx] = false;
                    map.visible_tiles[idx] = false;
                }
                if row as usize == MAPHEIGHT - 1 {
                    return RunState::CoreNextLevel { level };
                } else {
                    return RunState::CoreFadeToNextLevel {
                        level,
                        row: row + 1,
                    };
                }
            }
            RunState::CoreNextLevel { level } => {
                {
                    let mut stats = self.ecs.fetch_mut::<OverallStats>();
                    let mut level_stats = self.ecs.fetch_mut::<LevelStats>();
                    stats.apply_level(*level_stats);
                    window_fx::narrate(&stats, &level_stats);
                    level_stats.reset(level);
                }
                self.goto_level(level);
                return RunState::CoreLevelStart;
            }

            // breakout menu loops
            RunState::MenuInventory => {
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => return RunState::CoreAwaitingInput,
                    gui::ItemMenuResult::NoResponse => return current_runstate,
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.expect(
                            "show_inventory always should return entity with Selected response",
                        );
                        let is_ranged = self.ecs.read_storage::<Ranged>();
                        let is_item_ranged = is_ranged.get(item_entity);
                        let is_player_teleporting = self.ecs.read_storage::<TeleportsPlayer>();
                        if let Some(is_item_ranged) = is_item_ranged {
                            return RunState::ActionTargeting {
                                range: is_item_ranged.range,
                                item: item_entity,
                            };
                        } else if let Some(is_player_teleporting) =
                            is_player_teleporting.get(item_entity)
                        {
                            return RunState::CoreNextLevel {
                                level: is_player_teleporting.level,
                            };
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            intent
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToUseItem {
                                        item: item_entity,
                                        target: None,
                                    },
                                )
                                .expect(
                                    "should be able to insert intent to drink potion for player",
                                );
                            return RunState::CorePlayerTurn;
                        }
                    }
                }
            }
            RunState::MenuDropItem => {
                let result = gui::show_drop_item(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => return RunState::CoreAwaitingInput,
                    gui::ItemMenuResult::NoResponse => return current_runstate,
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.expect(
                            "show_drop_item always should return entity with Selected response",
                        );
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToDropItem { item: item_entity },
                            )
                            .expect("should be able to insert intent to drop item for player");
                        return RunState::CorePlayerTurn;
                    }
                }
            }
            RunState::MenuRemoveItem => {
                let result = gui::show_remove_item(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => return RunState::CoreAwaitingInput,
                    gui::ItemMenuResult::NoResponse => return current_runstate,
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.expect(
                            "show_remove_item always should return entity with Selected response",
                        );
                        let mut intent = self.ecs.write_storage::<WantsToRemoveItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToRemoveItem { item: item_entity },
                            )
                            .expect("should be able to insert intent to unequip item for player");
                        return RunState::CorePlayerTurn;
                    }
                }
            }

            // breakout action states
            RunState::ActionTargeting { range, item } => {
                let result = gui::ranged_target(self, ctx, range);
                match result.0 {
                    gui::ItemMenuResult::Cancel => return RunState::CoreAwaitingInput,
                    gui::ItemMenuResult::NoResponse => return current_runstate,
                    gui::ItemMenuResult::Selected => {
                        let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToUseItem {
                                    item,
                                    target: result.1,
                                },
                            )
                            .expect("should be able to insert intent to use item");
                        return RunState::CorePlayerTurn;
                    }
                }
            }
            RunState::ActionMagicMapReveal { row, iteration } => {
                let mut map = self.ecs.fetch_mut::<Map>();
                for x in (0..MAPWIDTH as i32).filter(|x| ((x + row) % 2) == iteration) {
                    let idx = map.xy_idx(x as i32, row);
                    map.revealed_tiles[idx] = true;
                }
                if row as usize == MAPHEIGHT - 1 {
                    if iteration == 1 {
                        return RunState::CoreMonsterTurn;
                    } else {
                        return RunState::ActionMagicMapReveal {
                            row: 0,
                            iteration: iteration + 1,
                        };
                    }
                } else {
                    return RunState::ActionMagicMapReveal {
                        row: row + 1,
                        iteration,
                    };
                }
            }
            RunState::ActionShowObjects { current, total } => match ctx.key {
                Some(rltk::VirtualKeyCode::Space) => {
                    if current < total - 1 {
                        return RunState::ActionShowObjects {
                            current: current + 1,
                            total,
                        };
                    } else {
                        return RunState::CoreAwaitingInput;
                    }
                }
                _ => return current_runstate,
            },
            RunState::ActionHighlightObjects {} => {
                match ctx.key {
                    Some(rltk::VirtualKeyCode::Escape) => {
                        let mut ui_config = self.ecs.write_resource::<UIConfig>();
                        ui_config.highlight_discoveries = false;
                    }
                    _ => {}
                }
                match ctx.key {
                    Some(rltk::VirtualKeyCode::Space) | Some(rltk::VirtualKeyCode::Escape) => {
                        let mut to_delete = Vec::new();
                        {
                            for (entity, _highlight_item) in (
                                &self.ecs.entities(),
                                &self.ecs.read_storage::<HighlightObject>(),
                            )
                                .join()
                            {
                                to_delete.push(entity);
                            }
                        }
                        for entity in to_delete {
                            let _ = self.ecs.delete_entity(entity);
                        }
                        return RunState::CorePreRound;
                    }
                    _ => {}
                }
                return current_runstate;
            }

            // outer loop states
            RunState::OuterMainMenu { .. } => {
                let result = menu::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => {
                        return RunState::OuterMainMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::MainMenuResult::Selected { selected } => match selected {
                        gui::MainMenuSelection::NewGame => return RunState::CoreLevelStart,
                        gui::MainMenuSelection::LoadGame => {
                            saveload_system::load_game(&mut self.ecs);
                            saveload_system::delete_save();
                            return RunState::CorePreRound;
                        }
                        gui::MainMenuSelection::Quit => {
                            ::std::process::exit(0);
                        }
                    },
                }
            }
            RunState::OuterSaveGame => {
                saveload_system::save_game(&mut self.ecs);
                return RunState::OuterMainMenu {
                    menu_selection: gui::MainMenuSelection::LoadGame,
                };
            }
            RunState::OuterCakeReveal { row, iteration } => {
                let mut map = self.ecs.fetch_mut::<Map>();
                for x in (0..MAPWIDTH as i32).filter(|x| ((x + row) % 2) == iteration) {
                    let idx = map.xy_idx(x as i32, row);
                    map.revealed_tiles[idx] = false;
                }
                if row as usize == MAPHEIGHT - 1 {
                    if iteration == 1 {
                        return RunState::OuterCakeJudge;
                    } else {
                        return RunState::OuterCakeReveal {
                            row: 0,
                            iteration: iteration + 1,
                        };
                    }
                } else {
                    return RunState::OuterCakeReveal {
                        row: row + 1,
                        iteration,
                    };
                }
            }
            RunState::OuterCakeJudge => {
                let result = gui::cake_judge(ctx, &self.ecs.fetch::<OverallStats>());
                match result {
                    gui::GameOverResult::NoSelection => return current_runstate,
                    gui::GameOverResult::QuitToMenu => {
                        self.game_over_cleanup();
                        return RunState::CoreLevelStart;
                    }
                }
            }
            RunState::OuterGameOver => {
                let result = gui::game_over(ctx, &self.ecs.fetch::<OverallStats>());
                match result {
                    gui::GameOverResult::NoSelection => return current_runstate,
                    gui::GameOverResult::QuitToMenu => {
                        self.game_over_cleanup();
                        return RunState::CoreLevelStart;
                    }
                }
            }
        }
    }
}
