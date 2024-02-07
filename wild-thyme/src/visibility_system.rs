use crate::{
    components::{Hidden, Name, VisibleToPlayer},
    gamelog::{GameLog, LogEntry},
};

use super::{Map, Player, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Hidden>,
        WriteExpect<'a, rltk::RandomNumberGenerator>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, VisibleToPlayer>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            entities,
            mut viewshed,
            pos,
            player,
            mut hidden,
            mut rng,
            mut log,
            names,
            mut visible_to_player,
        ) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed
                    .visible_tiles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

                if let Some(_p) = player.get(ent) {
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    visible_to_player.clear();
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;

                        // sometimes reveal hidden tiles
                        for e in map.tile_content[idx].iter() {
                            if let Some(_hidden) = hidden.get(*e) {
                                if rng.roll_dice(1, 24) == 1 {
                                    if let Some(name) = names.get(*e) {
                                        log.log(LogEntry::Alert {
                                            alert: format!("YOU spotted a {}!", &name.name),
                                        });
                                        let _ = visible_to_player.insert(*e, VisibleToPlayer {});
                                    }
                                    hidden.remove(*e);
                                }
                            } else {
                                let _ = visible_to_player.insert(*e, VisibleToPlayer {});
                            }
                        }
                    }
                }
            }
        }
    }
}
