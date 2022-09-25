use macroquad::experimental::collections::storage;
use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

use crate::game::GameColor;
use crate::resources::Resources;
use crate::{MAP_LAYER_PHYSICS, MAP_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE_PX};

const TILE_TEALY_SPAWN_POINT: u32 = 49;
const TILE_ORANGEY_SPAWN_POINT: u32 = 99;

pub struct Level {
    tiled_map: tiled::Map,
    tealy_spawn_pos: Vec2,
    orangey_spawn_pos: Vec2,
}

impl Level {
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

        Ok(Level {
            tiled_map,
            tealy_spawn_pos,
            orangey_spawn_pos,
        })
    }

    pub fn draw(&self) {
        let y_offset = 64.;
        let dest = Rect::new(
            0.,
            y_offset,
            SCREEN_WIDTH as f32,
            SCREEN_HEIGHT as f32 - y_offset,
        );

        self.tiled_map.draw_tiles("background", dest, None);
        self.tiled_map.draw_tiles("physics", dest, None);
        self.tiled_map.draw_tiles("meta", dest, None);
    }

    pub fn get_spawn_position(&self, game_color: GameColor) -> Vec2 {
        match game_color {
            GameColor::Tealy => self.tealy_spawn_pos,
            GameColor::Orangey => self.orangey_spawn_pos,
        }
    }
}
