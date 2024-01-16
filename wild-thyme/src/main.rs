use std::collections::HashMap;

use hunger_system::HungerSystem;
use rltk::{GameState, Point, Rltk};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

mod map;
use map::*;
mod player;
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
mod discovery_system;
mod game_loop;
mod hunger_system;
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
    CoreLevelStart,

    CorePreRound,
    CoreAwaitingInput,
    CorePlayerTurn,
    CoreMonsterTurn,
    CorePostRound,

    CoreFadeToNextLevel {
        level: i32,
        row: i32,
    },
    CoreNextLevel {
        level: i32,
    },

    MenuInventory,
    MenuDropItem,
    MenuRemoveItem,

    ActionTargeting {
        range: i32,
        item: Entity,
    },
    ActionMagicMapReveal {
        row: i32,
        iteration: i32,
    },
    ActionShowObjects {
        current: i32,
        total: i32,
    },
    ActionHighlightObjects {},

    OuterMainMenu {
        menu_selection: gui::MainMenuSelection,
    },
    OuterSaveGame,
    OuterCakeReveal {
        row: i32,
        iteration: i32,
    },
    OuterCakeJudge,
    OuterGameOver,
}

pub struct State {
    pub ecs: World,
}
pub struct UIConfig {
    highlight_discoveries: bool,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let current_runstate = *self.ecs.fetch::<RunState>();

        // clear terminal buffer and cleanup fx
        ctx.cls();
        particle_system::cull_dead_particles(&mut self.ecs, ctx);

        // render map if game is active
        match current_runstate {
            RunState::OuterMainMenu { .. } => {}
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

        // main game loop
        let next_runstate = self.run_game_loop(ctx, current_runstate);
        *self.ecs.fetch_mut::<RunState>() = next_runstate;

        // clean up dead entities
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
        let seen_things = self.ecs.read_storage::<SeenByPlayer>();

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

            let seen = seen_things.get(entity);
            if seen.is_some() {
                should_delete = false;
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
        self.ecs.insert(RunState::CoreLevelStart);
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
        .with_title("..And We Had a Wild Thyme")
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
    gs.ecs.register::<HighlightObject>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Rare>();
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
    gs.ecs.insert(UIConfig {
        highlight_discoveries: true,
    });

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
