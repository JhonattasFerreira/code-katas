mod ai;
mod ball;
mod collision;
mod constants;
mod game_state;
mod input;
mod menu;
mod paddle;
mod render;
#[cfg(test)]
mod test_helpers;
mod window;

use ball::Ball;
use constants::BUTTON_SIZE;
use game_state::{GameState, RoundOutcome};
use macroquad::prelude::*;
use menu::AppPhase;
use paddle::Paddle;
use window::window_conf;

fn reset_match(player_paddle: &mut Paddle, cpu_paddle: &mut Paddle, ball: &mut Ball) {
    *player_paddle = Paddle::new_player();
    *cpu_paddle = Paddle::new_cpu();
    *ball = Ball::new_centered();
}

fn update_playing(
    game_state: &mut GameState,
    player_paddle: &mut Paddle,
    cpu_paddle: &mut Paddle,
    ball: &mut Ball,
    dt: f32,
) -> Option<AppPhase> {
    if game_state.is_round_over() {
        if game_state.update(dt) == RoundOutcome::ReturnToMenu {
            reset_match(player_paddle, cpu_paddle, ball);
            return Some(AppPhase::Menu);
        }
        return None;
    }

    let intent = input::read_move_intent();
    player_paddle.move_by(intent, dt);

    let cpu_intent = ai::chase_intent(cpu_paddle.position.y, cpu_paddle.size.y, ball.center().y);
    cpu_paddle.move_by(cpu_intent, dt);

    if game_state.is_serving() {
        game_state.tick_serve(dt);
    } else {
        ball.apply_movement(dt);
        ball.bounce_off_walls();
        ball.resolve_collision_with(player_paddle);
        ball.resolve_collision_with(cpu_paddle);

        if let Some(side) = ball::check_scoring_exit(ball.position.x, ball.size) {
            game_state.register_point(side);
            *ball = Ball::new_centered_towards(side);
        }
    }

    None
}

fn draw_playing(game_state: &GameState, player_paddle: &Paddle, cpu_paddle: &Paddle, ball: &Ball) {
    render::draw_center_line();
    render::draw_paddle(player_paddle);
    render::draw_paddle(cpu_paddle);
    render::draw_ball(ball);
    render::draw_score(game_state);
    if let Some(winner) = game_state.winner {
        render::draw_winner_message(winner);
    }
    if let Some(remaining) = game_state.serve_countdown() {
        render::draw_serve_countdown(remaining);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player_paddle = Paddle::new_player();
    let mut cpu_paddle = Paddle::new_cpu();
    let mut ball = Ball::new_centered();
    let mut game_state = GameState::new();
    let mut app_phase = AppPhase::Menu;
    let button_pos = vec2(0.5, 0.5) - BUTTON_SIZE / 2.0;

    loop {
        let dt = get_frame_time();

        match app_phase {
            AppPhase::Menu => {
                if menu::start_requested(button_pos, BUTTON_SIZE) {
                    app_phase = AppPhase::Playing;
                }
            }
            AppPhase::Playing => {
                if let Some(next_phase) =
                    update_playing(&mut game_state, &mut player_paddle, &mut cpu_paddle, &mut ball, dt)
                {
                    app_phase = next_phase;
                }
            }
        }

        clear_background(BLACK);
        match app_phase {
            AppPhase::Menu => {
                render::draw_start_button(button_pos, BUTTON_SIZE);
                render::draw_controls_hint(button_pos, BUTTON_SIZE);
                render::draw_credit();
            }
            AppPhase::Playing => {
                draw_playing(&game_state, &player_paddle, &cpu_paddle, &ball);
            }
        }
        next_frame().await
    }
}
