use macroquad::experimental::scene::{Node, RefMut};
use macroquad::prelude::*;

use crate::{
    COLOR_ORANGEY, COLOR_ORANGEY_DARK, COLOR_ORANGEY_LIGHT, COLOR_TEALY, COLOR_TEALY_DARK,
    COLOR_TEALY_LIGHT, SCREEN_HEIGHT, SCREEN_WIDTH,
};

const BORDER: u32 = 5;
const SHADOW_OFFSET: u32 = 3;

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

    fn draw_background(&self) {
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
    }

    fn draw_text(
        &self,
        text: &str,
        center: Vec2,
        font_size: u16,
        color: Color,
        shadow_color: Option<Color>,
    ) {
        let text_size = measure_text(text, None, font_size, 1.);

        if let Some(shadow_color) = shadow_color {
            draw_text_ex(
                text,
                center.x - text_size.width / 2. + SHADOW_OFFSET as f32,
                center.y - text_size.height / 2. + SHADOW_OFFSET as f32,
                TextParams {
                    font_size,
                    color: shadow_color,
                    ..Default::default()
                },
            );
        }

        draw_text_ex(
            text,
            center.x - text_size.width / 2.,
            center.y - text_size.height / 2.,
            TextParams {
                font_size,
                color,
                ..Default::default()
            },
        );
    }

    fn draw_logo(&self) {
        let size = vec2(SCREEN_WIDTH as f32 / 2., SCREEN_HEIGHT as f32 / 4.);
        let center = vec2(SCREEN_WIDTH as f32 / 2., SCREEN_HEIGHT as f32 / 4.);

        draw_rectangle(
            center.x - size.x / 2. - BORDER as f32,
            center.y - size.y / 2. - BORDER as f32,
            size.x + 2. * BORDER as f32,
            size.y + 2. * BORDER as f32,
            COLOR_TEALY_LIGHT,
        );

        draw_rectangle(
            center.x - size.x / 2.,
            center.y - size.y / 2.,
            size.x,
            size.y,
            BLACK,
        );

        let font_size = 60;

        self.draw_text(
            "Tealy",
            vec2(
                SCREEN_WIDTH as f32 / 2.,
                SCREEN_HEIGHT as f32 / 3.5 - font_size as f32,
            ),
            font_size,
            COLOR_TEALY,
            Some(COLOR_TEALY_DARK),
        );

        self.draw_text(
            "&",
            vec2(SCREEN_WIDTH as f32 / 2., SCREEN_HEIGHT as f32 / 3.5),
            font_size,
            WHITE,
            Some(GRAY),
        );

        self.draw_text(
            "Orangey",
            vec2(
                SCREEN_WIDTH as f32 / 2.,
                SCREEN_HEIGHT as f32 / 3.5 + font_size as f32,
            ),
            font_size,
            COLOR_ORANGEY,
            Some(COLOR_ORANGEY_DARK),
        );
    }
}

impl Node for Menu {
    fn ready(_node: RefMut<Self>) {}

    fn draw(node: RefMut<Self>) {
        node.draw_background();
        node.draw_logo();
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
