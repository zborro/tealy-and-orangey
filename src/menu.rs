use macroquad::experimental::scene::{Node, RefMut};
use macroquad::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MenuSelection {
    StartGame,
    ContinueGame,
}

pub struct Menu {
    pub selection: Option<MenuSelection>,
    index: u8,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            selection: None,
            index: 0,
        }
    }
}

impl Node for Menu {
    fn ready(_node: RefMut<Self>) {}

    fn draw(node: RefMut<Self>) {
        draw_text_ex(
            format!("menu selection: {}", node.index).as_str(),
            32.,
            32.,
            TextParams {
                color: WHITE,
                font_size: 32,
                ..Default::default()
            },
        );
    }

    fn update(mut node: RefMut<Self>) {
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
            node.index = (node.index + 1) % 2;
        } else if is_key_pressed(KeyCode::Enter) {
            node.selection = Some(if node.index == 0 {
                MenuSelection::StartGame
            } else {
                MenuSelection::ContinueGame
            });
        }
    }
}
