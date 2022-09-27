use crate::game::GameColor;
use crate::{COLOR_ORANGEY_LIGHT, COLOR_TEALY_LIGHT};
use macroquad::prelude::*;
use macroquad_platformer::*;

const BALL_RADIUS: u32 = 16;
const GRAVITY: f32 = 250.;

pub struct Ball {
    pub collider: Actor,
    pub speed: Vec2,
    pos: Vec2,
    game_color: GameColor,
}

impl Ball {
    fn draw(&self) {
        let color = if self.game_color == GameColor::Tealy {
            COLOR_TEALY_LIGHT
        } else {
            COLOR_ORANGEY_LIGHT
        };
        draw_circle(
            self.pos.x + BALL_RADIUS as f32,
            self.pos.y + BALL_RADIUS as f32,
            BALL_RADIUS as f32,
            color,
        );
    }

    fn update(&mut self, pos: Vec2) {
        self.pos = pos;
        self.speed.y = GRAVITY;
    }
}

pub struct Balls {
    pub tealy: Ball,
    pub orangey: Ball,
}

impl Balls {
    pub fn new(
        tealy_pos: Vec2,
        tealy_collider: Actor,
        orangey_pos: Vec2,
        orangey_collider: Actor,
    ) -> Balls {
        Balls {
            tealy: Ball {
                collider: tealy_collider,
                game_color: GameColor::Tealy,
                pos: tealy_pos,
                speed: vec2(0., 0.),
            },
            orangey: Ball {
                collider: orangey_collider,
                pos: orangey_pos,
                game_color: GameColor::Orangey,
                speed: vec2(0., 0.),
            },
        }
    }

    pub fn draw(&self) {
        self.tealy.draw();
        self.orangey.draw();
    }

    pub fn update(&mut self, tealy_pos: Vec2, orangey_pos: Vec2) {
        self.tealy.update(tealy_pos);
        self.orangey.update(orangey_pos);
    }
}
