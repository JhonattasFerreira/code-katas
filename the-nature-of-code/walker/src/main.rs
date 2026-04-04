use macroquad::prelude::*;

mod walker;
use walker::Walker;

fn window_conf() -> Conf {
    Conf {
        window_title: "Walker".to_string(),
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut walker = Walker::new();

    loop {
        clear_background(Color::from_rgba(0, 80, 0, 255));
        walker.step();
        walker.show();

        next_frame().await;
    }
}
