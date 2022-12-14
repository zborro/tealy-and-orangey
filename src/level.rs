use macroquad::experimental::collections::storage;
use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

use crate::balls::Balls;
use crate::game::GameColor;
use crate::resources::Resources;
use crate::{MAP_LAYER_PHYSICS, MAP_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE_PX};

const TILE_TEALY_SPAWN_POINT: u32 = 49;
const TILE_ORANGEY_SPAWN_POINT: u32 = 99;

pub struct Level {
    tiled_map: tiled::Map,
    physics: World,
    balls: Balls,
}

impl Level {
    fn _get_static_colliders(tiled_map: &tiled::Map) -> Vec<Tile> {
        let mut static_colliders = vec![Tile::Solid; MAP_WIDTH * 2];
        for (_x, _y, tile) in tiled_map.tiles("physics", None) {
            let tile = match tile {
                Some(_) => Tile::Solid,
                _ => Tile::Empty,
            };
            static_colliders.push(tile);
        }
        // TODO (optional): why extra solid tiles are required at the bottom?
        static_colliders.append(&mut vec![Tile::Solid; MAP_WIDTH]);
        static_colliders
    }

    pub fn new() -> Result<Level, FileError> {
        let resources = storage::get::<Resources>();

        let tiled_map = tiled::load_map(
            &resources.level_json,
            &[("tileset.png", resources.tileset)],
            &[],
        )
        .unwrap();

        let y_offset = 2;
        let mut tealy_spawn_pos = vec2(0., 0.);
        let mut orangey_spawn_pos = vec2(0., 0.);
        for (x, y, tile) in tiled_map.tiles("spawn", None) {
            if let Some(tile) = tile {
                let pos = vec2(
                    (x * TILE_SIZE_PX) as f32,
                    ((y + y_offset) * TILE_SIZE_PX) as f32,
                );
                match tile.id {
                    TILE_TEALY_SPAWN_POINT => tealy_spawn_pos = pos,
                    TILE_ORANGEY_SPAWN_POINT => orangey_spawn_pos = pos,
                    _ => {}
                }
            }
        }

        let mut physics = World::new();
        physics.add_static_tiled_layer(
            Self::_get_static_colliders(&tiled_map),
            TILE_SIZE_PX as f32,
            TILE_SIZE_PX as f32,
            MAP_WIDTH,
            MAP_LAYER_PHYSICS,
        );

        let balls = Balls::new(
            tealy_spawn_pos,
            physics.add_actor(tealy_spawn_pos, TILE_SIZE_PX as i32, TILE_SIZE_PX as i32),
            orangey_spawn_pos,
            physics.add_actor(orangey_spawn_pos, TILE_SIZE_PX as i32, TILE_SIZE_PX as i32),
        );

        Ok(Level {
            tiled_map,
            physics,
            balls,
        })
    }

    pub fn draw(&self) {
        let y_offset = 2. * TILE_SIZE_PX as f32;
        let dest = Rect::new(
            0.,
            y_offset,
            SCREEN_WIDTH as f32,
            SCREEN_HEIGHT as f32 - y_offset,
        );

        self.tiled_map.draw_tiles("background", dest, None);
        self.tiled_map.draw_tiles("physics", dest, None);
        self.tiled_map.draw_tiles("meta", dest, None);
        self.balls.draw();
    }

    pub fn update(&mut self) {
        self.balls.update(&self.physics);

        self.physics.move_h(
            self.balls.tealy.collider,
            self.balls.tealy.speed.x * get_frame_time(),
        );
        self.physics.move_v(
            self.balls.tealy.collider,
            self.balls.tealy.speed.y * get_frame_time(),
        );

        self.physics.move_h(
            self.balls.orangey.collider,
            self.balls.orangey.speed.x * get_frame_time(),
        );
        self.physics.move_v(
            self.balls.orangey.collider,
            self.balls.orangey.speed.y * get_frame_time(),
        );
    }
}
