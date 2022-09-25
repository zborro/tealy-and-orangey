use crate::game::GameColor;
use crate::{COLOR_ORANGEY_LIGHT, COLOR_TEALY_LIGHT};
use macroquad::prelude::*;

const BALL_RADIUS: u32 = 16;

struct Ball {
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
}

pub struct Balls {
    tealy: Ball,
    orangey: Ball,
}

impl Balls {
    pub fn new(tealy_pos: Vec2, orangey_pos: Vec2) -> Balls {
        Balls {
            tealy: Ball {
                pos: tealy_pos,
                game_color: GameColor::Tealy,
            },
            orangey: Ball {
                pos: orangey_pos,
                game_color: GameColor::Orangey,
            },
        }
    }

    pub fn draw(&self) {
        self.tealy.draw();
        self.orangey.draw();
    }
}
