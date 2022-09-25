use macroquad::experimental::collections::storage;
use macroquad_platformer::*;
use macroquad::prelude::*;
use macroquad_tiled as tiled;

use crate::game::GameColor;
use crate::resources::Resources;
use crate::{MAP_WIDTH, TILE_SIZE_PX, MAP_LAYER_PHYSICS, SCREEN_WIDTH, SCREEN_HEIGHT};

pub struct Level {
    tiled_map: tiled::Map,
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

        Ok(Level {
            tiled_map,
        })
    }

    pub fn draw(&self) {
        let y_offset = 64.;
        let dest = Rect::new(0., y_offset, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32 - y_offset);

        self.tiled_map.draw_tiles("background", dest, None);
        self.tiled_map.draw_tiles("physics", dest, None);
        self.tiled_map.draw_tiles("meta", dest, None);
    }
}
