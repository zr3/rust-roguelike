const IS_ITEM_HIGHLIGHT_ENABLED: bool = false;

use std::collections::HashMap;

use hunger_system::HungerSystem;
use item_tutorial_system::ItemTutorialSystem;
use rltk::{GameState, Point, Rltk};
use spawn_system::{SpawnBuilder, SpawnRequest};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

mod map;
use map::*;
mod player;
use player::*;
mod rect;
use rect::*;
mod components;
mod stats;
use components::*;
use stats::*;
mod gamelog;
mod gui;
mod inventory_system;
mod spawners;
use inventory_system::*;
mod hunger_system;
mod item_tutorial_system;
mod menu;
mod particle_system;
mod quip_system;
mod random_table;
mod rex_assets;
mod saveload_system;
mod spawn_system;
mod trigger_system;
mod window_fx;

pub mod map_builders;

mod visibility_system;
use visibility_system::VisibilitySystem;
mod monster_ai_system;
use monster_ai_system::MonsterAI;
mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;
mod melee_combat_system;
use melee_combat_system::MeleeCombatSystem;
mod damage_system;
use damage_system::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    PostTurn,
    ShowInventory,
    ShowDropItem,
    ShowRemoveItem,
    ShowTargeting {
        range: i32,
        item: Entity,
    },
    ShowTooltips {
        current: i32,
        total: i32,
    },
    HighlightItem {},
    GameOver,
    MainMenu {
        menu_selection: gui::MainMenuSelection,
    },
    SaveGame,
    FadeToNextLevel {
        level: i32,
        row: i32,
    },
    NextLevel {
        level: i32,
    },
    MagicMapReveal {
        row: i32,
        iteration: i32,
    },
    CakeReveal {
        row: i32,
        iteration: i32,
    },
    CakeJudge,
}

pub struct State {
    pub ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.cls();
        particle_system::cull_dead_particles(&mut self.ecs, ctx);

        match newrunstate {
            RunState::MainMenu { .. } => {}
            _ => {
                draw_map(&self.ecs, ctx);

                {
                    let positions = self.ecs.read_storage::<Position>();
                    let renderables = self.ecs.read_storage::<Renderable>();
                    let hidden = self.ecs.read_storage::<Hidden>();
                    let map = self.ecs.fetch::<Map>();

                    let mut sorted_renderables = (&positions, &renderables, !&hidden)
                        .join()
                        .collect::<Vec<_>>();
                    sorted_renderables.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
                    for (pos, render, _hidden) in sorted_renderables.iter() {
                        let idx = map.xy_idx(pos.x, pos.y);
                        if map.visible_tiles[idx] {
                            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                        }
                    }
                    gui::draw_ui(&self.ecs, ctx);
                }
            }
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();

                newrunstate = match *self.ecs.fetch::<RunState>() {
                    RunState::MagicMapReveal { .. } => RunState::MagicMapReveal {
                        row: 0,
                        iteration: 0,
                    },
                    _ => RunState::MonsterTurn,
                }
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::PostTurn;
            }
            RunState::PostTurn => {
                self.run_systems();
                if IS_ITEM_HIGHLIGHT_ENABLED {
                    let mut itemtutorial = ItemTutorialSystem {};
                    itemtutorial.run_now(&self.ecs);
                }
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
                newrunstate = match *self.ecs.fetch::<RunState>() {
                    RunState::HighlightItem {} => RunState::HighlightItem {},
                    _ => RunState::AwaitingInput,
                }
            }
            RunState::ShowInventory => {
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.expect(
                            "show_inventory always should return entity with Selected response",
                        );
                        let is_ranged = self.ecs.read_storage::<Ranged>();
                        let is_item_ranged = is_ranged.get(item_entity);
                        let is_player_teleporting = self.ecs.read_storage::<TeleportsPlayer>();
                        if let Some(is_item_ranged) = is_item_ranged {
                            newrunstate = RunState::ShowTargeting {
                                range: is_item_ranged.range,
                                item: item_entity,
                            };
                        } else if let Some(is_player_teleporting) =
                            is_player_teleporting.get(item_entity)
                        {
                            newrunstate = RunState::NextLevel {
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
                            newrunstate = RunState::PlayerTurn;
                        }
                    }
                }
            }
            RunState::ShowDropItem => {
                let result = gui::show_drop_item(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
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
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowRemoveItem => {
                let result = gui::show_remove_item(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
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
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowTargeting { range, item } => {
                let result = gui::ranged_target(self, ctx, range);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
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
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::GameOver => {
                let result = gui::game_over(ctx, &self.ecs.fetch::<Stats>());
                match result {
                    gui::GameOverResult::NoSelection => {}
                    gui::GameOverResult::QuitToMenu => {
                        self.game_over_cleanup();
                        newrunstate = RunState::PreRun;
                    }
                }
            }
            RunState::MainMenu { .. } => {
                let result = menu::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => {
                        newrunstate = RunState::MainMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::MainMenuResult::Selected { selected } => match selected {
                        gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                        gui::MainMenuSelection::LoadGame => {
                            saveload_system::load_game(&mut self.ecs);
                            newrunstate = RunState::AwaitingInput;
                            saveload_system::delete_save();
                        }
                        gui::MainMenuSelection::Quit => {
                            ::std::process::exit(0);
                        }
                    },
                }
            }
            RunState::SaveGame => {
                saveload_system::save_game(&mut self.ecs);
                newrunstate = RunState::MainMenu {
                    menu_selection: gui::MainMenuSelection::LoadGame,
                };
            }
            RunState::FadeToNextLevel { level, row } => {
                window_fx::warp_effect();
                let mut map = self.ecs.fetch_mut::<Map>();
                for x in 0..MAPWIDTH as i32 {
                    let idx = map.xy_idx(x as i32, row);
                    map.revealed_tiles[idx] = false;
                    map.visible_tiles[idx] = false;
                }
                if row as usize == MAPHEIGHT - 1 {
                    newrunstate = RunState::NextLevel { level };
                } else {
                    newrunstate = RunState::FadeToNextLevel {
                        level,
                        row: row + 1,
                    };
                }
            }
            RunState::NextLevel { level } => {
                self.goto_level(level);
                let mut stats = self.ecs.fetch_mut::<Stats>();
                if stats.deepest_level < level {
                    stats.deepest_level = level;
                }
                window_fx::narrate(&stats);
                newrunstate = RunState::PreRun;
            }
            RunState::MagicMapReveal { row, iteration } => {
                let mut map = self.ecs.fetch_mut::<Map>();
                for x in (0..MAPWIDTH as i32).filter(|x| ((x + row) % 2) == iteration) {
                    let idx = map.xy_idx(x as i32, row);
                    map.revealed_tiles[idx] = true;
                }
                if row as usize == MAPHEIGHT - 1 {
                    if iteration == 1 {
                        newrunstate = RunState::MonsterTurn;
                    } else {
                        newrunstate = RunState::MagicMapReveal {
                            row: 0,
                            iteration: iteration + 1,
                        };
                    }
                } else {
                    newrunstate = RunState::MagicMapReveal {
                        row: row + 1,
                        iteration,
                    };
                }
            }
            RunState::CakeReveal { row, iteration } => {
                let mut map = self.ecs.fetch_mut::<Map>();
                for x in (0..MAPWIDTH as i32).filter(|x| ((x + row) % 2) == iteration) {
                    let idx = map.xy_idx(x as i32, row);
                    map.revealed_tiles[idx] = false;
                }
                if row as usize == MAPHEIGHT - 1 {
                    if iteration == 1 {
                        newrunstate = RunState::CakeJudge;
                    } else {
                        newrunstate = RunState::CakeReveal {
                            row: 0,
                            iteration: iteration + 1,
                        };
                    }
                } else {
                    newrunstate = RunState::CakeReveal {
                        row: row + 1,
                        iteration,
                    };
                }
            }
            RunState::CakeJudge => {
                let result = gui::cake_judge(ctx, &self.ecs.fetch::<Stats>());
                match result {
                    gui::GameOverResult::NoSelection => {}
                    gui::GameOverResult::QuitToMenu => {
                        self.game_over_cleanup();
                        newrunstate = RunState::PreRun;
                    }
                }
            }
            RunState::ShowTooltips { current, total } => match ctx.key {
                None => {}
                Some(_) => {
                    if current < total - 1 {
                        newrunstate = RunState::ShowTooltips {
                            current: current + 1,
                            total,
                        };
                    } else {
                        newrunstate = RunState::AwaitingInput;
                    }
                }
            },
            RunState::HighlightItem {} => match ctx.key {
                Some(rltk::VirtualKeyCode::Space) => {
                    let mut to_delete = Vec::new();
                    {
                        for (entity, _highlight_item, _position, _name) in (
                            &self.ecs.entities(),
                            &self.ecs.read_storage::<HighlightItem>(),
                            &self.ecs.read_storage::<Position>(),
                            &self.ecs.read_storage::<Name>(),
                        )
                            .join()
                        {
                            to_delete.push(entity);
                        }
                    }
                    for entity in to_delete {
                        let _ = self.ecs.delete_entity(entity);
                    }
                    newrunstate = RunState::AwaitingInput;
                }
                _ => {}
            },
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        delete_the_dead(&mut self.ecs);
    }
}

fn calculate_cake(ecs: &mut World) {
    let mut stats = ecs.fetch_mut::<Stats>();
    let map = ecs.fetch::<Map>();
    let ingredients = ecs.read_storage::<CakeIngredient>();
    let positions = ecs.read_storage::<Position>();
    let mut used_adjectives = HashMap::new();
    for (ingredient, pos) in (&ingredients, &positions).join() {
        if map.tiles[map.xy_idx(pos.x, pos.y)] != TileType::IngredientTable {
            continue;
        }
        // add ingredient to cake
        if !used_adjectives.contains_key(&ingredient.adjective) {
            used_adjectives.insert(&ingredient.adjective, 1);
            stats.cake.description = format!("{} {}", stats.cake.description, ingredient.adjective);
        } else if *used_adjectives
            .get(&ingredient.adjective)
            .expect("validated")
            < 2
        {
            *used_adjectives
                .get_mut(&ingredient.adjective)
                .expect("validated") = 2;
            stats.cake.description =
                format!("{} {}", stats.cake.description, ingredient.super_adjective);
        }
        stats.cake.overall_points += ingredient.overall_points;
        stats.cake.moist_points += ingredient.moist_points;
        stats.cake.sweet_points += ingredient.sweet_points;
        stats.cake.style_points += ingredient.style_points;
        stats.cake.hot_points += ingredient.hot_points;
        stats.cake.mold_points += ingredient.mold_points;
        stats.cake.edible_points += ingredient.edible_points;
    }
    stats.cake.description = format!("a{} cake!", stats.cake.description);
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut triggers = trigger_system::TriggerSystem {};
        triggers.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);
        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.ecs);
        let mut potions = UseItemSystem {};
        potions.run_now(&self.ecs);
        let mut drop_items = ItemDropSystem {};
        drop_items.run_now(&self.ecs);
        let mut remove_items = ItemRemoveSystem {};
        remove_items.run_now(&self.ecs);
        let mut hunger = HungerSystem {};
        hunger.run_now(&self.ecs);
        quip_system::QuipSystem {}.run_now(&self.ecs);
        // let mut fog = fog::FogSystem {};
        // fog.run_now(&self.ecs);
        let mut particles = particle_system::ParticleSpawnSystem {};
        particles.run_now(&self.ecs);

        self.ecs.maintain();
    }

    fn entities_to_remove_on_level_change(&mut self) -> Vec<Entity> {
        let entities = self.ecs.entities();
        let player = self.ecs.read_storage::<Player>();
        let backpack = self.ecs.read_storage::<InBackpack>();
        let player_entity = self.ecs.fetch::<Entity>();
        let equipped = self.ecs.read_storage::<Equipped>();

        let mut to_delete: Vec<Entity> = Vec::new();
        for entity in entities.join() {
            let mut should_delete = true;

            let p = player.get(entity);
            if let Some(_p) = p {
                should_delete = false;
            }

            let bp = backpack.get(entity);
            if let Some(bp) = bp {
                if bp.owner == *player_entity {
                    should_delete = false;
                }
            }

            let eq = equipped.get(entity);
            if let Some(eq) = eq {
                if eq.owner == *player_entity {
                    should_delete = false;
                }
            }

            if should_delete {
                to_delete.push(entity);
            }
        }

        to_delete
    }

    fn goto_level(&mut self, level: i32) {
        let to_delete = self.entities_to_remove_on_level_change();
        for target in to_delete {
            self.ecs
                .delete_entity(target)
                .expect("should be able to delete entity");
        }

        self.generate_world_map(level);

        let player_entity = self.ecs.fetch::<Entity>();
        let mut gamelog = self.ecs.fetch_mut::<gamelog::GameLog>();
        gamelog
            .log("YOU pass through the forest portal! and rest for a few minutes...".to_string());
        let mut player_health_store = self.ecs.write_storage::<CombatStats>();
        let player_health = player_health_store.get_mut(*player_entity);
        if let Some(player_health) = player_health {
            player_health.hp = i32::max(player_health.hp, player_health.max_hp / 2);
        }
    }

    fn game_over_cleanup(&mut self) {
        // delete everything
        let mut to_delete = Vec::new();
        for e in self.ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            self.ecs
                .delete_entity(*del)
                .expect("should be able to delete anything");
        }

        self.reset_game();
    }

    fn reset_game(&mut self) {
        let player_entity = spawners::player(&mut self.ecs, 0, 0);
        self.ecs.insert(player_entity);
        self.ecs.insert(Stats::new());
        self.ecs.insert(Map::new(1));
        self.ecs.insert(Point::new(0, 0));
        self.ecs.insert(RunState::PreRun);
        self.ecs.insert(particle_system::ParticleBuilder::new());
        self.ecs.insert(gamelog::GameLog::new(vec![
            "you find yourself in a dark af forest...".to_string(),
        ]));
        self.generate_world_map(1);
    }

    fn generate_world_map(&mut self, new_depth: i32) {
        // build new map
        let mut builder = map_builders::make_builder(new_depth);
        builder.build_map();
        let player_pos;
        {
            let mut worldmap_resource = self.ecs.write_resource::<Map>();
            *worldmap_resource = builder.get_map();
            player_pos = builder.get_starting_position();
        }

        builder.spawn_entities(&mut self.ecs);

        // restart everything
        let mut player_position = self.ecs.write_resource::<Point>();
        *player_position = Point::new(player_pos.x, player_pos.y);
        let mut position_components = self.ecs.write_storage::<Position>();
        let player_entity = self.ecs.write_resource::<Entity>();
        let player_pos_comp = position_components.get_mut(*player_entity);
        if let Some(player_pos_comp) = player_pos_comp {
            player_pos_comp.x = player_pos.x;
            player_pos_comp.y = player_pos.y;
        }

        // Mark the player's visibility as dirty
        let mut viewshed_components = self.ecs.write_storage::<Viewshed>();
        let vs = viewshed_components.get_mut(*player_entity);
        if let Some(vs) = vs {
            vs.dirty = true;
        }
    }
}

fn main() -> rltk::BError {
    // build context and game state
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("and we had a wild thyme")
        .with_gutter(16)
        .with_tile_dimensions(16, 16)
        .build()?;
    let mut gs = State { ecs: World::new() };

    // register components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<VisibleToPlayer>();
    gs.ecs.register::<SeenByPlayer>();
    gs.ecs.register::<HighlightItem>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToUseItem>();
    gs.ecs.register::<WantsToDropItem>();
    gs.ecs.register::<WantsToRemoveItem>();
    gs.ecs.register::<Consumable>();
    gs.ecs.register::<ProvidesHealing>();
    gs.ecs.register::<Ranged>();
    gs.ecs.register::<InflictsDamage>();
    gs.ecs.register::<AreaOfEffect>();
    gs.ecs.register::<Confusion>();
    gs.ecs.register::<SimpleMarker<SerializeMe>>();
    gs.ecs.register::<SerializationHelper>();
    gs.ecs.register::<Equippable>();
    gs.ecs.register::<Equipped>();
    gs.ecs.register::<MeleePowerBonus>();
    gs.ecs.register::<DefenseBonus>();
    gs.ecs.register::<ParticleLifetime>();
    gs.ecs.register::<HungerClock>();
    gs.ecs.register::<ProvidesFood>();
    gs.ecs.register::<MagicMapper>();
    gs.ecs.register::<Hidden>();
    gs.ecs.register::<EntryTrigger>();
    gs.ecs.register::<EntityMoved>();
    gs.ecs.register::<SingleActivation>();
    gs.ecs.register::<Fog>();
    gs.ecs.register::<Creature>();
    gs.ecs.register::<Herbivore>();
    gs.ecs.register::<HostileToPlayer>();
    gs.ecs.register::<DropsLoot>();
    gs.ecs.register::<SpawnsMobs>();
    gs.ecs.register::<TeleportsPlayer>();
    gs.ecs.register::<Quips>();
    gs.ecs.register::<Backpack>();
    gs.ecs.register::<GoodThyme>();
    gs.ecs.register::<CakeIngredient>();
    // new component register here

    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    // add resources
    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    gs.ecs.insert(spawn_system::SpawnBuilder::new());
    gs.ecs.insert(rex_assets::RexAssets::new());

    // build the first level
    gs.reset_game();

    // start main loop
    rltk::main_loop(context, gs)
}

pub fn get_visible_tooltips(ecs: &World) -> Vec<(Position, String)> {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();
    let hidden = ecs.read_storage::<Hidden>();

    let mut visible_tooltips = Vec::new();
    for (name, pos, _hidden) in (&names, &positions, !&hidden).join() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            visible_tooltips.push((pos.clone(), name.name.to_string()));
        }
    }
    visible_tooltips
}
