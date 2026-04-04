use macroquad::prelude::*;
use noise::{Fbm, NoiseFn, Value};

const RADIUS: f32 = 30.0;
const BORDER_THICKNESS: f32 = 2.0;

pub struct Walker {
    x: f32,
    y: f32,
    tx: f64,
    ty: f64,
    noise: Fbm<Value>,
}

impl Walker {
    pub fn new() -> Self {
        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            tx: 0.0,
            ty: 10000.0,
            noise: Fbm::<Value>::new(1),
        }
    }

    pub fn step(&mut self) {
        let nx = self.noise.get([self.tx, 0.0]) as f32;
        let ny = self.noise.get([self.ty, 0.0]) as f32;

        self.x = (nx + 1.0) / 2.0 * screen_width();
        self.y = (ny + 1.0) / 2.0 * screen_height();

        let delta = get_frame_time() as f64 * 60.0;
        self.tx += 0.007 * delta;
        self.ty += 0.007 * delta;
    }

    pub fn show(&self) {
        draw_poly(self.x, self.y, 64, RADIUS, 0.0, YELLOW);
        draw_poly_lines(self.x, self.y, 64, RADIUS, 0.0, BORDER_THICKNESS, BLACK);
    }
}
