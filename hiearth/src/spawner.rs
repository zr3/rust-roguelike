use super::{
    BlocksTile, CombatStats, Item, Monster, Name, Player, Position, Potion, Rect, Renderable,
    Viewshed, MAPWIDTH,
};
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

pub fn spawn_room(ecs: &mut World, room: &Rect) {
    let mut monster_spawn_points: Vec<usize> = Vec::new();
    let mut item_spawn_points: Vec<usize> = Vec::new();
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
        let num_items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;

        for _i in 0..num_monsters {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !monster_spawn_points.contains(&idx) {
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }

        for _i in 0..num_items {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !item_spawn_points.contains(&idx) {
                    item_spawn_points.push(idx);
                    added = true;
                }
            }
        }
    }

    for idx in monster_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_monster(ecs, x as i32, y as i32);
    }

    for idx in item_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        health_potion(ecs, x as i32, y as i32);
    }
}

fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('¿'),
            fg: RGB::named(rltk::VIOLET_RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 3,
        })
        .with(Name {
            name: "HEALTH POTION".to_string(),
        })
        .with(Item {})
        .with(Potion { heal_amount: 8 })
        .build();
}

pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::FORESTGREEN),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "YOU".to_string(),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build()
}

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }
    match roll {
        1 => kobold(ecs, x, y),
        2 => mosquito(ecs, x, y),
        3 => spider(ecs, x, y),
        _ => ghost(ecs, x, y),
    }
}

fn kobold(ecs: &mut World, x: i32, y: i32) {
    let color_roll: i32;
    let color: RGB;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        color_roll = rng.roll_dice(1, 2);
        color = match color_roll {
            1 => RGB::from_u8(rng.range(0, 255), rng.range(0, 255), rng.range(0, 255)),
            _ => RGB::named(rltk::PURPLE),
        };
    }
    monster(ecs, x, y, rltk::to_cp437('k'), color, "KOBOLD")
}
fn mosquito(ecs: &mut World, x: i32, y: i32) {
    monster(
        ecs,
        x,
        y,
        rltk::to_cp437('m'),
        RGB::named(rltk::SADDLE_BROWN),
        "MOSQUITO",
    )
}
fn spider(ecs: &mut World, x: i32, y: i32) {
    monster(
        ecs,
        x,
        y,
        rltk::to_cp437('s'),
        RGB::named(rltk::GREY),
        "SPIDER",
    )
}
fn ghost(ecs: &mut World, x: i32, y: i32) {
    monster(
        ecs,
        x,
        y,
        rltk::to_cp437('g'),
        RGB::named(rltk::MINT_CREAM),
        "GHOST",
    )
}

fn monster<S: ToString>(
    ecs: &mut World,
    x: i32,
    y: i32,
    glyph: rltk::FontCharType,
    fg: rltk::RGB,
    name: S,
) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg,
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Monster {})
        .with(Name {
            name: name.to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .build();
}
