use crate::game::GameColor;
use crate::{COLOR_ORANGEY_LIGHT, COLOR_TEALY_LIGHT};
use macroquad::prelude::*;
use macroquad_platformer::*;

const BALL_RADIUS: u32 = 16;
const GRAVITY: f32 = 250.;
const SPEED_X: f32 = 250.;
const SPEED_Y: f32 = 300.;

struct CollisionInfo {
    is_colliding_bottom: bool,
}

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

    fn update(&mut self, pos: Vec2, collision_info: CollisionInfo) {
        self.pos = pos;

        if is_key_down(KeyCode::Left) {
            self.speed.x = -SPEED_X;
        }
        else if is_key_down(KeyCode::Right) {
            self.speed.x = SPEED_X;
        }
        else {
            self.speed.x = 0.;
        }

        if !collision_info.is_colliding_bottom {
            self.speed.y += GRAVITY * get_frame_time();
        }

        if is_key_pressed(KeyCode::Space) && collision_info.is_colliding_bottom {
            self.speed.y = -SPEED_Y;
        }
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

    fn get_collision_info(&self, physics: &World, collider: Actor, pos: Vec2) -> CollisionInfo {
        let is_colliding_bottom = physics.collide_check(collider, pos + vec2(0., 1.));

        CollisionInfo {
            is_colliding_bottom,
        }
    }

    pub fn update(&mut self, physics: &World) {
        let tealy_pos = physics.actor_pos(self.tealy.collider);
        let orangey_pos = physics.actor_pos(self.orangey.collider);

        let collision_info = self.get_collision_info(physics, self.tealy.collider, tealy_pos);
        self.tealy.update(tealy_pos, collision_info);

        let collision_info = self.get_collision_info(physics, self.orangey.collider, orangey_pos);
        self.orangey.update(orangey_pos, collision_info);
    }
}
