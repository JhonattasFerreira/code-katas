use crate::constants::{PADDLE_MARGIN, PADDLE_SIZE, PADDLE_SPEED};
use crate::input::MoveIntent;
use macroquad::math::{vec2, Vec2};

pub struct Paddle {
    pub position: Vec2,
    pub size: Vec2,
    pub speed: f32,
}

impl Paddle {
    pub fn new_player() -> Self {
        Self::new_at(PADDLE_MARGIN)
    }

    pub fn new_cpu() -> Self {
        Self::new_at(1.0 - PADDLE_MARGIN - PADDLE_SIZE.x)
    }

    fn new_at(x: f32) -> Self {
        Paddle {
            position: vec2(x, 0.5 - PADDLE_SIZE.y / 2.0),
            size: PADDLE_SIZE,
            speed: PADDLE_SPEED,
        }
    }

    pub fn move_by(&mut self, intent: MoveIntent, dt: f32) {
        self.position.y = update_paddle_y(self.position.y, intent, self.speed, self.size.y, dt);
    }
}

pub fn update_paddle_y(y: f32, intent: MoveIntent, speed: f32, height: f32, dt: f32) -> f32 {
    let delta = speed * dt;
    let new_y = match intent {
        MoveIntent::Up => y - delta,
        MoveIntent::Down => y + delta,
        MoveIntent::Idle => y,
    };

    new_y.clamp(0.0, 1.0 - height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_does_not_move() {
        assert_eq!(update_paddle_y(0.5, MoveIntent::Idle, 0.6, 0.15, 0.1), 0.5);
    }

    #[test]
    fn up_moves_paddle_up() {
        let y = update_paddle_y(0.5, MoveIntent::Up, 0.6, 0.15, 0.1);
        assert_eq!(y, 0.5 - 0.6 * 0.1);
    }

    #[test]
    fn down_moves_paddle_down() {
        let y = update_paddle_y(0.5, MoveIntent::Down, 0.6, 0.15, 0.1);
        assert_eq!(y, 0.5 + 0.6 * 0.1);
    }

    #[test]
    fn clamps_at_top_edge() {
        let y = update_paddle_y(0.0, MoveIntent::Up, 0.6, 0.15, 0.1);
        assert_eq!(y, 0.0);
    }

    #[test]
    fn clamps_at_bottom_edge() {
        let y = update_paddle_y(0.85, MoveIntent::Down, 0.6, 0.15, 0.1);
        assert_eq!(y, 0.85);
    }

    #[test]
    fn zero_dt_does_not_move() {
        assert_eq!(update_paddle_y(0.5, MoveIntent::Up, 0.6, 0.15, 0.0), 0.5);
        assert_eq!(update_paddle_y(0.5, MoveIntent::Down, 0.6, 0.15, 0.0), 0.5);
    }

    #[test]
    fn large_delta_still_clamps_exactly_at_edge() {
        let y = update_paddle_y(0.5, MoveIntent::Up, 0.6, 0.15, 10.0);
        assert_eq!(y, 0.0);

        let y = update_paddle_y(0.5, MoveIntent::Down, 0.6, 0.15, 10.0);
        assert_eq!(y, 0.85);
    }

    #[test]
    fn move_by_updates_position_in_place() {
        let mut paddle = Paddle {
            position: vec2(0.02, 0.5),
            size: vec2(0.02, 0.15),
            speed: 0.6,
        };
        paddle.move_by(MoveIntent::Down, 0.1);
        assert_eq!(paddle.position.y, 0.5 + 0.6 * 0.1);
    }

    #[test]
    fn new_cpu_is_positioned_on_the_right_edge() {
        let paddle = Paddle::new_cpu();
        assert_eq!(paddle.position.x, 1.0 - PADDLE_MARGIN - PADDLE_SIZE.x);
        assert_eq!(paddle.position.y, 0.5 - PADDLE_SIZE.y / 2.0);
    }

    #[test]
    fn move_by_clamps_at_edge() {
        let mut paddle = Paddle {
            position: vec2(0.02, 0.0),
            size: vec2(0.02, 0.15),
            speed: 0.6,
        };
        paddle.move_by(MoveIntent::Up, 10.0);
        assert_eq!(paddle.position.y, 0.0);
    }
}
