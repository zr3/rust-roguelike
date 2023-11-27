use std::cmp::{max, min};

use rltk::RandomNumberGenerator;

use crate::{
    map::{Map, TileType},
    rect::Rect,
};

pub fn release_a_drunk(
    map: &mut Map,
    start_pos: (i32, i32),
    steps: i32,
    rng: &mut RandomNumberGenerator,
    return_steps: i32,
) -> Vec<(i32, i32)> {
    let mut drunk_x = start_pos.0;
    let mut drunk_y = start_pos.1;
    let mut result_steps = Vec::new();
    for step in 0..steps {
        match rng.roll_dice(1, 4) {
            1 => {
                if drunk_x > 2 {
                    drunk_x -= 1;
                }
            }
            2 => {
                if drunk_x < map.width - 2 {
                    drunk_x += 1;
                }
            }
            3 => {
                if drunk_y > 2 {
                    drunk_y -= 1;
                }
            }
            _ => {
                if drunk_y < map.height - 2 {
                    drunk_y += 1;
                }
            }
        }
        if step % return_steps == 1 {
            result_steps.push((drunk_x, drunk_y));
        }
        let idx = map.xy_idx(drunk_x, drunk_y);
        map.tiles[idx] = TileType::Floor;
    }
    result_steps
}

pub fn apply_room_to_map(map: &mut Map, room: &Rect) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }
}

pub fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = map.xy_idx(x, y);
        if idx > 0 && idx < map.width as usize * map.height as usize {
            map.tiles[idx as usize] = TileType::Floor;
        }
    }
}

pub fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = map.xy_idx(x, y);
        if idx > 0 && idx < map.width as usize * map.height as usize {
            map.tiles[idx as usize] = TileType::Floor;
        }
    }
}
