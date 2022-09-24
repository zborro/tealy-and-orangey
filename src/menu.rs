use macroquad::experimental::scene::{Node, RefMut};
use macroquad::prelude::*;

const BORDER: u32 = 5;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MenuSelection {
    StartGame,
    ContinueGame,
}

pub struct Menu {
    pub selection: Option<MenuSelection>,
    index: u8,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            selection: None,
            index: 0,
        }
    }
}

use crate::{
    COLOR_ORANGEY, COLOR_ORANGEY_LIGHT, COLOR_TEALY, COLOR_TEALY_LIGHT, SCREEN_HEIGHT, SCREEN_WIDTH,
};

impl Node for Menu {
    fn ready(_node: RefMut<Self>) {}

    fn draw(node: RefMut<Self>) {
        draw_rectangle(
            0.,
            0.,
            SCREEN_WIDTH as f32,
            SCREEN_HEIGHT as f32 / 2.,
            COLOR_TEALY_LIGHT,
        );
        draw_rectangle(
            BORDER as f32,
            BORDER as f32,
            (SCREEN_WIDTH - 2 * BORDER) as f32,
            (SCREEN_HEIGHT - 2 * BORDER) as f32 / 2.,
            COLOR_TEALY,
        );

        draw_rectangle(
            0.,
            SCREEN_HEIGHT as f32 / 2.,
            SCREEN_WIDTH as f32,
            SCREEN_HEIGHT as f32 / 2.,
            COLOR_ORANGEY_LIGHT,
        );

        draw_rectangle(
            BORDER as f32,
            (SCREEN_HEIGHT / 2) as f32,
            (SCREEN_WIDTH - 2 * BORDER) as f32,
            (SCREEN_HEIGHT / 2 - BORDER) as f32,
            COLOR_ORANGEY,
        );

        draw_text_ex(
            format!("menu selection: {}", node.index).as_str(),
            32.,
            32.,
            TextParams {
                color: WHITE,
                font_size: 32,
                ..Default::default()
            },
        );
    }

    fn update(mut node: RefMut<Self>) {
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
            node.index = (node.index + 1) % 2;
        } else if is_key_pressed(KeyCode::Enter) {
            node.selection = Some(if node.index == 0 {
                MenuSelection::StartGame
            } else {
                MenuSelection::ContinueGame
            });
        }
    }
}
