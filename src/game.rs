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
    balls: Balls,
}

impl Game {
    pub fn new(_start_level: u32) -> Game {
        let level = Level::new().unwrap();

        let tealy_pos = level.get_spawn_position(GameColor::Tealy);
        let orangey_pos = level.get_spawn_position(GameColor::Orangey);

        Game {
            level,
            balls: Balls::new(tealy_pos, orangey_pos),
        }
    }
}

impl Node for Game {
    fn draw(node: RefMut<Self>) {
        clear_background(BLACK);
        node.level.draw();
        node.balls.draw();
    }

    fn update(mut _node: RefMut<Self>) {}
}
