use macroquad::experimental::scene::{Node, RefMut};
use macroquad::prelude::*;

use crate::balls::Balls;
use crate::level::Level;

#[derive(PartialEq, Eq)]
pub enum GameColor {
    Tealy,
    Orangey,
}

pub struct Game {
    level: Level,
}

impl Game {
    pub fn new(_start_level: u32) -> Game {
        let level = Level::new().unwrap();

        Game { level }
    }
}

impl Node for Game {
    fn draw(node: RefMut<Self>) {
        clear_background(BLACK);
        node.level.draw();
    }

    fn update(mut node: RefMut<Self>) {
        node.level.update();
    }
}
