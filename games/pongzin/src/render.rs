use crate::ball::Ball;
use crate::constants::{
    CENTER_LINE_DASH_COUNT, CENTER_LINE_WIDTH, CONTROLS_FONT_SIZE, CONTROLS_MARGIN_BELOW_BUTTON,
    CREDIT_FONT_SIZE, CREDIT_MARGIN, SCORE_FONT_SIZE, SCORE_Y_FRACTION,
    SERVE_COUNTDOWN_FONT_SIZE, SERVE_COUNTDOWN_Y_FRACTION, START_BUTTON_MARKER_LAP_DURATION,
    START_BUTTON_MARKER_SIZE, START_BUTTON_TEXT_FONT_SIZE, WINNER_MESSAGE_FONT_SIZE,
    WINNER_MESSAGE_Y_FRACTION,
};
use crate::game_state::{GameState, Winner};
use crate::menu::border_marker_position;
use crate::paddle::Paddle;
use macroquad::prelude::*;

fn rect_pixels(position: Vec2, size: Vec2) -> (f32, f32, f32, f32) {
    (
        position.x * screen_width(),
        position.y * screen_height(),
        size.x * screen_width(),
        size.y * screen_height(),
    )
}

fn draw_scaled_rect(position: Vec2, size: Vec2, color: Color) {
    let (x, y, w, h) = rect_pixels(position, size);
    draw_rectangle(x, y, w, h, color);
}

fn text_dimensions(text: &str, font_size_fraction: f32) -> (f32, TextDimensions) {
    let font_size = font_size_fraction * screen_height();
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    (font_size, dimensions)
}

fn draw_centered_text(text: &str, y_fraction: f32, font_size_fraction: f32) {
    let (font_size, dimensions) = text_dimensions(text, font_size_fraction);
    draw_text(
        text,
        (screen_width() - dimensions.width) / 2.0,
        y_fraction * screen_height(),
        font_size,
        WHITE,
    );
}

fn draw_text_centered_in_rect(text: &str, rect_pos: Vec2, rect_size: Vec2, font_size_fraction: f32, color: Color) {
    let (font_size, dimensions) = text_dimensions(text, font_size_fraction);
    let rect_center_x = (rect_pos.x + rect_size.x / 2.0) * screen_width();
    let rect_center_y = (rect_pos.y + rect_size.y / 2.0) * screen_height();
    draw_text(
        text,
        rect_center_x - dimensions.width / 2.0,
        rect_center_y + dimensions.height / 2.0,
        font_size,
        color,
    );
}

pub fn draw_paddle(paddle: &Paddle) {
    draw_scaled_rect(paddle.position, paddle.size, WHITE);
}

pub fn draw_ball(ball: &Ball) {
    draw_scaled_rect(ball.position, Vec2::splat(ball.size), WHITE);
}

pub fn draw_center_line() {
    let dash_height = 1.0 / CENTER_LINE_DASH_COUNT as f32;
    let x = 0.5 - CENTER_LINE_WIDTH / 2.0;

    for i in 0..CENTER_LINE_DASH_COUNT {
        if i % 2 == 0 {
            draw_scaled_rect(
                vec2(x, i as f32 * dash_height),
                vec2(CENTER_LINE_WIDTH, dash_height),
                WHITE,
            );
        }
    }
}

pub fn draw_score(game_state: &GameState) {
    let text = format!("{}   {}", game_state.player_score, game_state.cpu_score);
    draw_centered_text(&text, SCORE_Y_FRACTION, SCORE_FONT_SIZE);
}

pub fn draw_start_button(position: Vec2, size: Vec2) {
    let (x, y, w, h) = rect_pixels(position, size);
    draw_rectangle_lines(x, y, w, h, 2.0, WHITE);
    draw_text_centered_in_rect("START", position, size, START_BUTTON_TEXT_FONT_SIZE, WHITE);
    draw_start_button_marker(position, size);
}

fn draw_start_button_marker(position: Vec2, size: Vec2) {
    let lap_progress = (get_time() as f32 / START_BUTTON_MARKER_LAP_DURATION).rem_euclid(1.0);
    let marker_center = border_marker_position(lap_progress, position, size);
    draw_scaled_rect(
        marker_center - Vec2::splat(START_BUTTON_MARKER_SIZE / 2.0),
        Vec2::splat(START_BUTTON_MARKER_SIZE),
        BLACK,
    );
}

pub fn draw_controls_hint(button_pos: Vec2, button_size: Vec2) {
    draw_centered_text(
        "Use UP / DOWN arrow keys to move your paddle",
        button_pos.y + button_size.y + CONTROLS_MARGIN_BELOW_BUTTON,
        CONTROLS_FONT_SIZE,
    );
}

pub fn draw_credit() {
    let font_size = CREDIT_FONT_SIZE * screen_height();
    draw_text(
        "Made with love by Jhocore",
        CREDIT_MARGIN * screen_width(),
        (1.0 - CREDIT_MARGIN) * screen_height(),
        font_size,
        WHITE,
    );
}

pub fn draw_winner_message(winner: Winner) {
    let text = match winner {
        Winner::Player => "Player wins!",
        Winner::Cpu => "CPU wins!",
    };
    draw_centered_text(text, WINNER_MESSAGE_Y_FRACTION, WINNER_MESSAGE_FONT_SIZE);
}

pub fn draw_serve_countdown(remaining: f32) {
    let text = (remaining.ceil() as i32).to_string();
    draw_centered_text(&text, SERVE_COUNTDOWN_Y_FRACTION, SERVE_COUNTDOWN_FONT_SIZE);
}
