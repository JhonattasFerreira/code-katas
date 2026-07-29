#![cfg(test)]

use crate::ball::Ball;
use crate::paddle::Paddle;
use macroquad::math::{vec2, Vec2};

pub fn ball_at(x: f32, y: f32, size: f32, direction: Vec2) -> Ball {
    Ball {
        position: vec2(x, y),
        size,
        speed: 0.5,
        direction,
    }
}

pub fn paddle_at(x: f32, y: f32, width: f32, height: f32) -> Paddle {
    Paddle {
        position: vec2(x, y),
        size: vec2(width, height),
        speed: 0.6,
    }
}
