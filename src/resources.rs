use macroquad::prelude::*;

pub struct Resources {
    pub tileset: Texture2D,
    pub level_json: String,
}

impl Resources {
    pub async fn new() -> Result<Resources, FileError> {
        let tileset = load_texture("assets/tileset.png").await?;
        tileset.set_filter(FilterMode::Nearest);

        let level_json = load_string("assets/level-00.tmj").await.unwrap();

        Ok(Resources {
            tileset,
            level_json,
        })
    }
}
