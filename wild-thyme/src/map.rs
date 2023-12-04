use super::{Entity, Rect, World};
use rltk::{Algorithm2D, BaseMap, Point, Rltk, RGB};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const MAPWIDTH: usize = 80;
pub const MAPHEIGHT: usize = 43;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Water,
    Floor,
    DownStairs,
    IngredientTable,
    JudgeCake,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,
    pub bloodstains: HashSet<usize>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new(new_depth: i32) -> Map {
        Map {
            tiles: vec![TileType::Wall; MAPCOUNT],
            rooms: Vec::new(),
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: vec![false; MAPCOUNT],
            visible_tiles: vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
            tile_content: vec![Vec::new(); MAPCOUNT],
            depth: new_depth,
            bloodstains: HashSet::new(),
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        y as usize * self.width as usize + x as usize
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall || *tile == TileType::Water;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    fg = RGB::from_hex("#39441b").expect("hardcoded");
                    glyph = match prime_pattern(idx) {
                        PatternMode::A => rltk::to_cp437('.'),
                        PatternMode::B => rltk::to_cp437(','),
                        PatternMode::C => rltk::to_cp437(' '),
                        PatternMode::D => rltk::to_cp437('.'),
                    };
                }
                TileType::Wall => {
                    fg = RGB::from_hex("#39561b").expect("hardcoded");
                    glyph = match prime_pattern_a(idx) {
                        true => rltk::to_cp437('♠'),
                        false => rltk::to_cp437('♣'),
                    };
                }
                TileType::Water => {
                    fg = RGB::from_hex("#104070").expect("hardcoded");
                    glyph = rltk::to_cp437('░'); // wall_glyph(&*map, x, y);
                }
                TileType::DownStairs => {
                    fg = RGB::from_hex("#8f7a4a").expect("hardcoded");
                    glyph = rltk::to_cp437('Ö');
                }
                TileType::IngredientTable => {
                    fg = RGB::from_hex("#904070").expect("hardcoded");
                    glyph = rltk::to_cp437('O'); // wall_glyph(&*map, x, y);
                }
                TileType::JudgeCake => {
                    fg = RGB::from_hex("#904070").expect("hardcoded");
                    glyph = rltk::to_cp437('░'); // wall_glyph(&*map, x, y);
                }
            }
            if map.bloodstains.contains(&idx) {
                fg = RGB::from_hex("#cc3f0c").expect("hardcoded");
            }
            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale()
            }
            ctx.set(
                x,
                y,
                fg,
                RGB::from_hex("#000000").expect("hardcoded"),
                // RGB::from_hex("#222211").expect("hardcoded"),
                glyph,
            );
        }
        x += 1;
        if x > map.width - 1 {
            x = 0;
            y += 1;
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - w) - 1, 1.45))
        };
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - w) + 1, 1.45))
        };
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + w) - 1, 1.45))
        };
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + w) + 1, 1.45))
        };
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

fn prime_pattern(i: usize) -> PatternMode {
    match (prime_pattern_a(i), prime_pattern_b(i)) {
        (true, true) => PatternMode::A,
        (true, false) => PatternMode::B,
        (false, true) => PatternMode::C,
        (false, false) => PatternMode::D,
    }
}
enum PatternMode {
    A,
    B,
    C,
    D,
}
fn prime_pattern_a(i: usize) -> bool {
    i % 13 == 0 || i % 17 == 0 || i & 43 == 0
}

fn prime_pattern_b(i: usize) -> bool {
    i % 31 == 0 || i % 11 == 0 || i % 19 == 0
}
