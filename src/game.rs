use macroquad::experimental::scene::{Node, RefMut};
use macroquad::prelude::*;

pub struct Game;

impl Game {
    pub fn new(_start_level: u32) -> Game {
        Game
    }
}

impl Node for Game {
    fn draw(_node: RefMut<Self>) {
        draw_text_ex(
            "Game...",
            32.,
            32.,
            TextParams {
                color: WHITE,
                font_size: 32,
                ..Default::default()
            },
        );
    }

    fn update(mut _node: RefMut<Self>) {}
}
