use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use specs_derive::Component;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;

pub struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        player_input(self, ctx);
        self.run_systems();
        ctx.cls();
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

fn main() -> rltk::BError {
    // build context and game state
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("rusty roguelike tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };

    // add Specs resources
    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    // register components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    // create entities
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::FORESTGREEN),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    // start main loop
    rltk::main_loop(context, gs)
}
