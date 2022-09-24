use macroquad::prelude::*;

mod game;
mod menu;

use crate::game::Game;
use crate::menu::{Menu, MenuSelection};

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 960;
const COLOR_TEALY_DARK: Color = Color::new(0.17, 0.40, 0.45, 1.00);
const COLOR_TEALY: Color = Color::new(0.19, 0.59, 0.69, 1.00);
const COLOR_TEALY_LIGHT: Color = Color::new(0.41, 0.78, 0.86, 1.00);

const COLOR_ORANGEY_DARK: Color = Color::new(0.43, 0.26, 0.08, 1.00);
const COLOR_ORANGEY: Color = Color::new(0.67, 0.39, 0.09, 1.00);
const COLOR_ORANGEY_LIGHT: Color = Color::new(0.86, 0.56, 0.26, 1.00);

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
