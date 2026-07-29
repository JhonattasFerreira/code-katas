use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use macroquad::prelude::Conf;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Pongzin".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: true,
        fullscreen: false,
        ..Default::default()
    }
}
