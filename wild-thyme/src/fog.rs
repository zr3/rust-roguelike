use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs::saveload::MarkedBuilder;
use specs::{saveload::SimpleMarker, World};

use crate::components::{Fog, Name, Position, Renderable, SerializeMe};
use crate::map::Map;
use crate::RunState;

const FOG_LIFETIME_ROUNDS: i32 = 3;

pub struct FogSystem {}

impl<'a> System<'a> for FogSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Fog>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteExpect<'a, RandomNumberGenerator>,
        ReadExpect<'a, Map>,
        ReadExpect<'a, RunState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut fog, mut position, mut renderable, mut rng, map, run_state) = data;
        for (_, mut fog, mut renderable, mut position) in
            (&entities, &mut fog, &mut renderable, &mut position).join()
        {
            fog.lifetime_rounds -= 1;
            if fog.lifetime_rounds <= 0 {
                fog.lifetime_rounds = rng.roll_dice(1, FOG_LIFETIME_ROUNDS);
                position.x = rng.roll_dice(1, map.width);
                position.y = rng.roll_dice(1, 10); // todo: linear gradient
            }
            renderable.glyph = match fog.lifetime_rounds {
                1 => rltk::to_cp437('.'),
                2 => rltk::to_cp437('o'),
                _ => rltk::to_cp437('O'),
            };
        }
    }
}

pub fn respawn_fog(ecs: &mut World) {
    // delete all old fogs
    let mut delete_fogs: Vec<Entity> = Vec::new();
    {
        let fogs = ecs.read_storage::<Fog>();
        let entities = ecs.entities();
        for (entity, _fog) in (&entities, &fogs).join() {
            delete_fogs.push(entity);
        }
    }
    for entity in delete_fogs {
        ecs.delete_entity(entity)
            .expect("should be able to delete fog entity");
    }

    // make all new fogs
    let width: i32;
    {
        let map = ecs.read_resource::<Map>();
        width = map.width;
    }
    for y in 0..10 {
        for x in 0..width {
            // fade in fog from top
            let glyph;
            let fg;
            {
                let mut rng = ecs.write_resource::<RandomNumberGenerator>();
                if rng.roll_dice(1, (y - 2).clamp(1, 10)) != 1 {
                    continue;
                }

                // random glyphs for nice effect
                glyph = match rng.roll_dice(1, 3) {
                    1 => rltk::to_cp437('.'),
                    2 => rltk::to_cp437('o'),
                    _ => rltk::to_cp437('O'),
                };

                // stable color for consistency
                fg = RGB::from_hex("#707860").expect("hardcoded");
            }

            // make the entity
            ecs.create_entity()
                .with(Position { x, y })
                .with(Renderable {
                    glyph,
                    fg,
                    bg: RGB::named(rltk::BLACK),
                    render_order: 5,
                })
                .with(Fog {
                    lifetime_rounds: FOG_LIFETIME_ROUNDS,
                })
                .with(Name {
                    name: "MYSTERIOUS FOG".to_string(),
                })
                .marked::<SimpleMarker<SerializeMe>>()
                .build();
        }
    }
}
