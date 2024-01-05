use rltk::{Rltk, VirtualKeyCode, RGB};

use crate::{
    gui::{MainMenuResult, MainMenuSelection},
    rex_assets::RexAssets,
    RunState, State,
};

pub fn main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    let save_exists = super::saveload_system::does_save_exist();
    let runstate = gs.ecs.fetch::<RunState>();
    let assets = gs.ecs.fetch::<RexAssets>();
    ctx.render_xp_sprite(&assets.menu, 0, 0);

    ctx.draw_box(
        20,
        14,
        39,
        2,
        RGB::from_hex("#808030").expect("hardcoded"),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color_centered(
        15,
        RGB::named(rltk::BURLYWOOD),
        RGB::named(rltk::BLACK),
        "And We Had a Wild Thyme",
    );

    ctx.draw_box(
        30,
        23,
        19,
        4,
        RGB::from_hex("#808030").expect("hardcoded"),
        RGB::named(rltk::BLACK),
    );
    if let RunState::MainMenu {
        menu_selection: selection,
    } = *runstate
    {
        if selection == MainMenuSelection::NewGame {
            ctx.print_color_centered(
                25,
                RGB::from_hex("#70e0a0").expect("hardcoded"),
                RGB::named(rltk::BLACK),
                "new game",
            );
        } else {
            ctx.print_color_centered(
                25,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "new game",
            );
        }

        if save_exists {
            if selection == MainMenuSelection::LoadGame {
                ctx.print_color_centered(
                    24,
                    RGB::named(rltk::GREEN_YELLOW),
                    RGB::named(rltk::BLACK),
                    "load game",
                );
            } else {
                ctx.print_color_centered(
                    24,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::BLACK),
                    "load game",
                );
            }
        }

        // if selection == MainMenuSelection::Quit {
        //     ctx.print_color_centered(
        //         26,
        //         RGB::named(rltk::GREEN_YELLOW),
        //         RGB::named(rltk::BLACK),
        //         "quit",
        //     );
        // } else {
        //     ctx.print_color_centered(26, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "quit");
        // }

        match ctx.key {
            None => {
                return MainMenuResult::NoSelection {
                    selected: selection,
                }
            }
            Some(key) => match key {
                // VirtualKeyCode::Escape => {
                //     return MainMenuResult::NoSelection {
                //         selected: MainMenuSelection::Quit,
                //     }
                // }
                // VirtualKeyCode::Up | VirtualKeyCode::K => {
                //     let newselection;
                //     match selection {
                //         MainMenuSelection::NewGame => newselection = MainMenuSelection::Quit,
                //         MainMenuSelection::LoadGame => newselection = MainMenuSelection::NewGame,
                //         MainMenuSelection::Quit => {
                //             if save_exists {
                //                 newselection = MainMenuSelection::LoadGame;
                //             } else {
                //                 newselection = MainMenuSelection::NewGame;
                //             }
                //         }
                //     }
                //     return MainMenuResult::NoSelection {
                //         selected: newselection,
                //     };
                // }
                // VirtualKeyCode::Down | VirtualKeyCode::J => {
                //     let newselection;
                //     match selection {
                //         MainMenuSelection::NewGame => {
                //             if save_exists {
                //                 newselection = MainMenuSelection::LoadGame;
                //             } else {
                //                 newselection = MainMenuSelection::Quit;
                //             }
                //         }
                //         MainMenuSelection::LoadGame => newselection = MainMenuSelection::Quit,
                //         MainMenuSelection::Quit => newselection = MainMenuSelection::NewGame,
                //     }
                //     return MainMenuResult::NoSelection {
                //         selected: newselection,
                //     };
                // }
                VirtualKeyCode::Return | VirtualKeyCode::Space => {
                    return MainMenuResult::Selected {
                        selected: selection,
                    }
                }
                _ => {
                    return MainMenuResult::NoSelection {
                        selected: selection,
                    }
                }
            },
        }
    }

    MainMenuResult::NoSelection {
        selected: crate::gui::MainMenuSelection::NewGame,
    }
}
