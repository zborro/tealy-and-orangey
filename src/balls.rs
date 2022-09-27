use crate::game::GameColor;
use crate::{COLOR_ORANGEY_LIGHT, COLOR_TEALY_LIGHT};
use macroquad::prelude::*;
use macroquad_platformer::*;

const BALL_RADIUS: u32 = 16;
const GRAVITY: f32 = 250.;

pub struct Ball {
    pub collider: Actor,
    pub speed: Vec2,
    game_color: GameColor,
}

impl Ball {
    fn draw(&self, pos: Vec2) {
        let color = if self.game_color == GameColor::Tealy {
            COLOR_TEALY_LIGHT
        } else {
            COLOR_ORANGEY_LIGHT
        };
        draw_circle(
            pos.x + BALL_RADIUS as f32,
            pos.y + BALL_RADIUS as f32,
            BALL_RADIUS as f32,
            color,
        );
    }

    fn update(&mut self) {
        self.speed.y = GRAVITY;
    }
}

pub struct Balls {
    pub tealy: Ball,
    pub orangey: Ball,
}

impl Balls {
    pub fn new(tealy_collider: Actor, orangey_collider: Actor) -> Balls {
        Balls {
            tealy: Ball {
                collider: tealy_collider,
                game_color: GameColor::Tealy,
                speed: vec2(0., 0.),
            },
            orangey: Ball {
                collider: orangey_collider,
                game_color: GameColor::Orangey,
                speed: vec2(0., 0.),
            },
        }
    }

    pub fn draw(&self, tealy_pos: Vec2, orangey_pos: Vec2) {
        self.tealy.draw(tealy_pos);
        self.orangey.draw(orangey_pos);
    }

    pub fn update(&mut self) {
        self.tealy.update();
        self.orangey.update();
    }
}
