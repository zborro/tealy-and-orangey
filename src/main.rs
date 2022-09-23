use macroquad::prelude::*;

mod game;
mod menu;

use crate::game::Game;
use crate::menu::{Menu, MenuSelection};

#[macroquad::main("TealyAndOrangey")]
async fn main() {
    loop {
        let mut selection;
        let menu_handle = scene::add_node(Menu::new());

        let camera = Camera2D::from_display_rect(Rect::new(0., 0., 1280., 960.));
        scene::set_camera(0, Some(camera));

        loop {
            if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
                return;
            }

            selection = scene::get_node(menu_handle).selection;
            if selection.is_some() {
                break;
            }

            next_frame().await
        }

        let start_level = {
            let selection = selection.unwrap();

            if selection == MenuSelection::ContinueGame {
                panic!("Continue Game not implemented yet!");
            }

            info!("Starting game");

            0
        };

        scene::clear();
        let _game_handle = scene::add_node(Game::new(start_level));

        loop {
            if is_key_pressed(KeyCode::Q) {
                return;
            }

            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            next_frame().await;
        }

        scene::clear();
    }
}
