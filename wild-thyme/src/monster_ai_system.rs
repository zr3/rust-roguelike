use crate::{
    components::{Confusion, EntityMoved, HostileToPlayer, WantsToSwap},
    particle_system::ParticleBuilder,
};

use super::{Map, Monster, Position, RunState, Viewshed, WantsToMelee};
use rltk::{Point, RandomNumberGenerator};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, WantsToSwap>,
        WriteStorage<'a, Confusion>,
        WriteExpect<'a, ParticleBuilder>,
        WriteStorage<'a, EntityMoved>,
        ReadStorage<'a, HostileToPlayer>,
        WriteExpect<'a, RandomNumberGenerator>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player_entity,
            runstate,
            entities,
            mut viewshed,
            monster,
            mut positions,
            mut wants_to_melee,
            mut wants_to_swap,
            mut confused,
            mut particle_builder,
            mut entity_moved,
            hostile,
            mut rng,
        ) = data;

        if *runstate != RunState::CoreMonsterTurn {
            return;
        }

        for (entity, mut viewshed, _monster, mut pos) in
            (&entities, &mut viewshed, &monster, &mut positions).join()
        {
            let mut can_act = true;

            let is_confused = confused.get_mut(entity);
            if let Some(i_am_confused) = is_confused {
                i_am_confused.turns -= 1;
                if i_am_confused.turns < 1 {
                    confused.remove(entity);
                }
                can_act = false;

                particle_builder.request(
                    pos.x,
                    pos.y,
                    rltk::RGB::named(rltk::PURPLE),
                    rltk::RGB::named(rltk::BLACK),
                    rltk::to_cp437('?'),
                    200.0,
                );
            }

            if !can_act {
                continue;
            }

            let distance =
                rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if let Some(_) = hostile.get(entity) {
                // if hostile, move close and attack
                if distance < 1.5 {
                    wants_to_melee
                        .insert(
                            entity,
                            WantsToMelee {
                                target: *player_entity,
                            },
                        )
                        .expect("unable to insert attack");
                } else if viewshed.visible_tiles.contains(&*player_pos) {
                    let path = rltk::a_star_search(
                        map.xy_idx(pos.x, pos.y) as i32,
                        map.xy_idx(player_pos.x, player_pos.y) as i32,
                        &mut *map,
                    );
                    if path.success && path.steps.len() > 1 {
                        let mut idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = false;
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;
                        idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = true;
                        viewshed.dirty = true;
                        entity_moved
                            .insert(entity, EntityMoved {})
                            .expect("should be able to add movement marker");
                    }
                }
            } else {
                // if not hostile, just wander
                if distance > 2. {
                    let new_pos = (
                        pos.x + rng.roll_dice(1, 3) - 2,
                        pos.y + rng.roll_dice(1, 3) - 2,
                    );
                    let new_pos_idx = map.xy_idx(new_pos.0, new_pos.1);
                    if !map.blocked[new_pos_idx] {
                        let old_pos_idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[old_pos_idx] = false;
                        pos.x = new_pos.0;
                        pos.y = new_pos.1;
                        map.blocked[new_pos_idx] = true;
                        viewshed.dirty = true;
                        entity_moved
                            .insert(entity, EntityMoved {})
                            .expect("should be able to add movement marker");
                    }
                }
            }
        }
        for (swapper_entity, swapper) in (&entities, &wants_to_swap).join() {
            let (old_swapper_x, old_swapper_y);
            if let Some(swapper_pos) = positions.get_mut(swapper_entity) {
                (old_swapper_x, old_swapper_y) = (swapper_pos.x, swapper_pos.y);
            } else {
                continue;
            }
            let (old_target_x, old_target_y);
            if let Some(target_pos) = positions.get_mut(swapper.target) {
                (old_target_x, old_target_y) = (target_pos.x, target_pos.y);
            } else {
                continue;
            }
            if let Some(target_pos) = positions.get_mut(swapper.target) {
                target_pos.x = old_swapper_x;
                target_pos.y = old_swapper_y;
            }
            if let Some(swapper_pos) = positions.get_mut(swapper_entity) {
                swapper_pos.x = old_target_x;
                swapper_pos.y = old_target_y;
            }
        }
        wants_to_swap.clear();
    }
}
