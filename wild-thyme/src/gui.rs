use super::{gamelog::GameLog, Map, Name, Player, Position};
use rltk::{Point, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

use crate::{
    components::{
        Backpack, CombatStats, Equipped, Hidden, HungerClock, HungerState, InBackpack, Viewshed,
    },
    stats::Stats,
    State,
};

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame,
    Quit,
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    // bg box
    ctx.draw_box(
        0,
        43,
        79,
        6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    // depth level
    let map = ecs.fetch::<Map>();
    let depth = format!("depth: {}", map.depth);
    ctx.print_color(
        2,
        43,
        RGB::named(rltk::BURLYWOOD),
        RGB::named(rltk::BLACK),
        &depth,
    );

    // hp bar
    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    let hunger = ecs.read_storage::<HungerClock>();
    for (_player, stats, hc) in (&players, &combat_stats, &hunger).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(
            12,
            43,
            RGB::named(rltk::GREEN_YELLOW),
            RGB::named(rltk::BLACK),
            &health,
        );
        ctx.draw_bar_horizontal(
            28,
            43,
            51,
            stats.hp,
            stats.max_hp,
            RGB::named(rltk::DEEPPINK),
            RGB::named(rltk::BLACK),
        );

        match hc.state {
            HungerState::Full => ctx.print_color(
                70,
                42,
                RGB::named(rltk::LIMEGREEN),
                RGB::named(rltk::BLACK),
                "TUMMY FULL",
            ),
            HungerState::Normal => {}
            HungerState::Hungry => ctx.print_color(
                70,
                42,
                RGB::named(rltk::ORANGE),
                RGB::named(rltk::BLACK),
                "HUNGRY....",
            ),
            HungerState::Starving => ctx.print_color(
                70,
                42,
                RGB::named(rltk::DARK_RED),
                RGB::named(rltk::BLACK),
                "STARVING!!",
            ),
        }
    }

    // game log
    let log = ecs.fetch::<GameLog>();
    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, s);
        }
        y += 1;
    }

    // mouse tooltips
    draw_tooltips(ecs, ctx);
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();
    let hidden = ecs.read_storage::<Hidden>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height {
        return;
    }
    let mut tooltip: Vec<String> = Vec::new();
    for (name, position, _hidden) in (&names, &positions, !&hidden).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            tooltip.push(name.name.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 {
                width = s.len() as i32;
            }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(
                    left_x,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::BLACK),
                    s,
                );
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x - i,
                        y,
                        RGB::named(rltk::WHITE),
                        rltk::BLACK,
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                rltk::BLACK,
                &"->".to_string(),
            );
        } else {
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x + 1, y, RGB::named(rltk::WHITE), rltk::BLACK, s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x + 1 + i,
                        y,
                        RGB::named(rltk::WHITE),
                        rltk::BLACK,
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                rltk::BLACK,
                &"<-".to_string(),
            );
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn show_inventory(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack_items = gs.ecs.read_storage::<InBackpack>();
    let backpacks = gs.ecs.read_storage::<Backpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack_items, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();
    let backpack = backpacks
        .get(*player_entity)
        .expect("player should always have backpack");

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    let padding;
    if backpack.items < 10 && backpack.capacity >= 10 {
        padding = "0";
    } else {
        padding = "";
    }
    let right_offset;
    if backpack.capacity >= 10 {
        right_offset = 6;
    } else {
        right_offset = 4;
    }
    ctx.print_color(
        15 + 31 - right_offset,
        y - 2,
        RGB::named(rltk::BURLYWOOD),
        RGB::named(rltk::BLACK),
        format!("{}{}/{}", padding, backpack.items, backpack.capacity),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::BURLYWOOD),
        RGB::named(rltk::BLACK),
        "INVENTORY",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::BURLYWOOD),
        RGB::named(rltk::BLACK),
        "[ESCAPE] to cancel",
    );

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack_items, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
    {
        ctx.set(
            17,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            y,
            RGB::named(rltk::BURLYWOOD),
            RGB::named(rltk::BLACK),
            97 + j as rltk::FontCharType,
        );
        ctx.set(
            19,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        j += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}

pub fn show_drop_item(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::BURLYWOOD),
        RGB::named(rltk::BLACK),
        "DROP ITEM?",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::BURLYWOOD),
        RGB::named(rltk::BLACK),
        "[ESCAPE] to cancel",
    );

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
    {
        ctx.set(
            17,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            y,
            RGB::named(rltk::BURLYWOOD),
            RGB::named(rltk::BLACK),
            97 + j as rltk::FontCharType,
        );
        ctx.set(
            19,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        j += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}

pub fn show_remove_item(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<Equipped>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::BURLYWOOD),
        RGB::named(rltk::BLACK),
        "UNEQUIP ITEM?",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::BURLYWOOD),
        RGB::named(rltk::BLACK),
        "[ESCAPE] to cancel",
    );

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
    {
        ctx.set(
            17,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            y,
            RGB::named(rltk::BURLYWOOD),
            RGB::named(rltk::BLACK),
            97 + j as rltk::FontCharType,
        );
        ctx.set(
            19,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        j += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}

pub fn ranged_target(
    gs: &mut State,
    ctx: &mut Rltk,
    range: i32,
) -> (ItemMenuResult, Option<Point>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let player_pos = gs.ecs.fetch::<Point>();
    let viewsheds = gs.ecs.read_storage::<Viewshed>();

    ctx.print_color(
        5,
        0,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "[CLICK] on target (or an empty space to cancel):",
    );

    let mut available_cells = Vec::new();
    let visible = viewsheds.get(*player_entity);
    if let Some(visible) = visible {
        for idx in visible.visible_tiles.iter() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
            if distance <= range as f32 {
                ctx.set_bg(idx.x, idx.y, RGB::named(rltk::DARK_CYAN));
                available_cells.push(idx);
            }
        }
    } else {
        return (ItemMenuResult::Cancel, None);
    }

    let mouse_pos = ctx.mouse_pos();
    let mut valid_target = false;
    for idx in available_cells.iter() {
        if idx.x == mouse_pos.0 && idx.y == mouse_pos.1 {
            valid_target = true;
        }
    }
    if valid_target {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::CYAN));
        if ctx.left_click {
            return (
                ItemMenuResult::Selected,
                Some(Point::new(mouse_pos.0, mouse_pos.1)),
            );
        }
    } else {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::RED));
        if ctx.left_click {
            return (ItemMenuResult::Cancel, None);
        }
    }

    (ItemMenuResult::NoResponse, None)
}

#[derive(PartialEq, Copy, Clone)]
pub enum GameOverResult {
    NoSelection,
    QuitToMenu,
}

fn print_stat(ctx: &mut Rltk, line: i32, stat: &str, stat_value: i32) {
    ctx.print_color_centered(
        line,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        format!("{}: {}", stat, stat_value),
    );
}

pub fn game_over(ctx: &mut Rltk, stats: &Stats) -> GameOverResult {
    ctx.print_color_centered(
        15,
        RGB::named(rltk::RED),
        RGB::named(rltk::BLACK),
        "OH NO YOU DIED!!",
    );

    print_stat(ctx, 17, "deepest level", stats.deepest_level);
    print_stat(ctx, 18, "THYME eaten", stats.thyme_eaten);
    print_stat(ctx, 19, "things killed", stats.mobs_killed);
    print_stat(ctx, 20, "portals taken", stats.portals_taken);
    print_stat(ctx, 21, "traps triggered", stats.traps_triggered);
    print_stat(ctx, 22, "steps taken", stats.steps_taken);

    ctx.print_color_centered(
        25,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "PRESS [ENTER]",
    );

    match ctx.key {
        None => GameOverResult::NoSelection,
        Some(rltk::VirtualKeyCode::Return) => GameOverResult::QuitToMenu,
        Some(_) => GameOverResult::NoSelection,
    }
}

pub fn cake_judge(ctx: &mut Rltk, stats: &Stats) -> GameOverResult {
    ctx.print_color_centered(
        7,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "you baked...",
    );
    ctx.print_color_centered(
        9,
        RGB::named(rltk::PALE_GREEN),
        RGB::named(rltk::BLACK),
        stats.cake.description.to_string(),
    );

    print_stat(ctx, 12, "overall", stats.cake.overall_points);
    print_stat(ctx, 13, "moistness", stats.cake.moist_points);
    print_stat(ctx, 14, "sweetness", stats.cake.sweet_points);
    print_stat(ctx, 15, "style", stats.cake.style_points);
    print_stat(ctx, 16, "spiciness", stats.cake.hot_points);
    print_stat(ctx, 17, "moldiness", stats.cake.mold_points);
    print_stat(ctx, 18, "edible?", stats.cake.edible_points);

    print_stat(ctx, 20, "deepest level", stats.deepest_level);
    print_stat(ctx, 21, "THYME eaten", stats.thyme_eaten);
    print_stat(ctx, 22, "minimum hp", stats.min_hp);
    print_stat(ctx, 23, "things killed", stats.mobs_killed);
    print_stat(ctx, 24, "portals taken", stats.portals_taken);
    print_stat(ctx, 25, "traps triggered", stats.traps_triggered);
    print_stat(ctx, 26, "steps taken", stats.steps_taken);

    ctx.print_color_centered(
        36,
        RGB::named(rltk::PALE_GREEN),
        RGB::named(rltk::BLACK),
        "YUMMMM THANKS FOR PLAYING!",
    );
    ctx.print_color_centered(
        32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "baking can be quite the adventure,",
    );
    ctx.print_color_centered(
        33,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "and we had a wild thyme.",
    );

    ctx.draw_box(
        0,
        43,
        79,
        6,
        RGB::named(rltk::BLACK),
        RGB::named(rltk::BLACK),
    );

    match ctx.key {
        None => GameOverResult::NoSelection,
        Some(rltk::VirtualKeyCode::Return) => GameOverResult::QuitToMenu,
        Some(_) => GameOverResult::NoSelection,
    }
}
