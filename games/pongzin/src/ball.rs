use crate::collision::resolve_ball_paddle_collision;
use crate::constants::{BALL_MAX_SPEED, BALL_SIZE, BALL_SPEED, BALL_SPEED_INCREASE_FACTOR};
use crate::paddle::Paddle;
use macroquad::math::{vec2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallExit {
    Left,
    Right,
}

pub struct Ball {
    pub position: Vec2,
    pub size: f32,
    pub speed: f32,
    pub direction: Vec2,
}

impl Ball {
    pub fn new_centered() -> Self {
        Self::new_centered_towards(BallExit::Left)
    }

    pub fn new_centered_towards(side: BallExit) -> Self {
        let direction = match side {
            BallExit::Left => vec2(-0.3, 0.0).normalize(),
            BallExit::Right => vec2(0.3, 0.0).normalize(),
        };

        Ball {
            position: Vec2::splat(0.5 - BALL_SIZE / 2.0),
            size: BALL_SIZE,
            speed: BALL_SPEED,
            direction,
        }
    }

    pub fn center(&self) -> Vec2 {
        self.position + Vec2::splat(self.size / 2.0)
    }

    pub fn apply_movement(&mut self, dt: f32) {
        self.position = update_ball_position(self.position, self.direction, self.speed, dt);
    }

    pub fn bounce_off_walls(&mut self) {
        let (y, direction) = compute_wall_bounce(self.position.y, self.size, self.direction);
        self.position.y = y;
        self.direction = direction;
    }

    pub fn resolve_collision_with(&mut self, paddle: &Paddle) {
        if let Some((x, direction)) = resolve_ball_paddle_collision(self, paddle) {
            self.position.x = x;
            self.direction = direction;
            self.increase_speed();
        }
    }

    fn increase_speed(&mut self) {
        self.speed = compute_increased_speed(self.speed, BALL_SPEED_INCREASE_FACTOR, BALL_MAX_SPEED);
    }
}

pub fn update_ball_position(position: Vec2, direction: Vec2, speed: f32, dt: f32) -> Vec2 {
    position + direction * speed * dt
}

pub fn compute_increased_speed(speed: f32, factor: f32, max: f32) -> f32 {
    (speed * factor).min(max)
}

pub fn check_scoring_exit(x: f32, size: f32) -> Option<BallExit> {
    if x + size <= 0.0 {
        Some(BallExit::Left)
    } else if x >= 1.0 {
        Some(BallExit::Right)
    } else {
        None
    }
}

pub fn compute_wall_bounce(y: f32, size: f32, direction: Vec2) -> (f32, Vec2) {
    if y <= 0.0 {
        (0.0, vec2(direction.x, direction.y.abs()))
    } else if y + size >= 1.0 {
        (1.0 - size, vec2(direction.x, -direction.y.abs()))
    } else {
        (y, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{ball_at, paddle_at};

    fn vec2_approx_eq(a: Vec2, b: Vec2) -> bool {
        (a - b).length() < 1e-4
    }

    #[test]
    fn moves_in_a_straight_line() {
        let position = update_ball_position(vec2(0.5, 0.5), vec2(0.6, 0.8), 0.5, 0.1);
        assert_eq!(position.x, 0.5 + 0.6 * 0.5 * 0.1);
        assert_eq!(position.y, 0.5 + 0.8 * 0.5 * 0.1);
    }

    #[test]
    fn zero_dt_does_not_move() {
        let position = update_ball_position(vec2(0.5, 0.5), vec2(0.6, 0.8), 0.5, 0.0);
        assert_eq!(position, vec2(0.5, 0.5));
    }

    #[test]
    fn no_bounce_when_away_from_walls() {
        let (y, direction) = compute_wall_bounce(0.5, 0.02, vec2(0.6, 0.8));
        assert_eq!(y, 0.5);
        assert_eq!(direction, vec2(0.6, 0.8));
    }

    #[test]
    fn bounces_off_top_wall() {
        let (y, direction) = compute_wall_bounce(-0.01, 0.02, vec2(0.6, -0.8));
        assert_eq!(y, 0.0);
        assert_eq!(direction, vec2(0.6, 0.8));
    }

    #[test]
    fn bounces_off_bottom_wall() {
        let (y, direction) = compute_wall_bounce(0.99, 0.02, vec2(0.6, 0.8));
        assert_eq!(y, 1.0 - 0.02);
        assert_eq!(direction, vec2(0.6, -0.8));
    }

    #[test]
    fn touching_top_edge_exactly_bounces() {
        let (y, direction) = compute_wall_bounce(0.0, 0.02, vec2(0.6, -0.8));
        assert_eq!(y, 0.0);
        assert_eq!(direction, vec2(0.6, 0.8));
    }

    #[test]
    fn touching_bottom_edge_exactly_bounces() {
        let (y, direction) = compute_wall_bounce(1.0 - 0.02, 0.02, vec2(0.6, 0.8));
        assert_eq!(y, 1.0 - 0.02);
        assert_eq!(direction, vec2(0.6, -0.8));
    }

    #[test]
    fn apply_movement_updates_position_in_place() {
        let mut ball = ball_at(0.5, 0.5, 0.02, vec2(0.6, 0.8));
        ball.apply_movement(0.1);
        assert_eq!(ball.position.x, 0.5 + 0.6 * 0.5 * 0.1);
        assert_eq!(ball.position.y, 0.5 + 0.8 * 0.5 * 0.1);
    }

    #[test]
    fn bounce_off_walls_updates_position_and_direction() {
        let mut ball = ball_at(0.5, 0.99, 0.02, vec2(0.6, 0.8));
        ball.bounce_off_walls();
        assert_eq!(ball.position.y, 1.0 - 0.02);
        assert_eq!(ball.direction, vec2(0.6, -0.8));
    }

    #[test]
    fn bounce_off_walls_does_nothing_away_from_walls() {
        let mut ball = ball_at(0.5, 0.5, 0.02, vec2(0.6, 0.8));
        ball.bounce_off_walls();
        assert_eq!(ball.position.y, 0.5);
        assert_eq!(ball.direction, vec2(0.6, 0.8));
    }

    #[test]
    fn resolve_collision_with_updates_position_and_direction_on_hit() {
        let mut ball = ball_at(0.03, 0.55, 0.02, vec2(-0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        ball.resolve_collision_with(&paddle);
        assert_eq!(ball.position.x, 0.02 + 0.02);
        // relative_hit = -0.2 (same geometry tested in collision.rs), angle = -9°
        assert!(vec2_approx_eq(ball.direction, vec2(0.98768836, -0.15643448)));
    }

    #[test]
    fn resolve_collision_with_increases_speed_on_hit() {
        let mut ball = ball_at(0.03, 0.55, 0.02, vec2(-0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        ball.resolve_collision_with(&paddle);
        assert!((ball.speed - compute_increased_speed(0.5, BALL_SPEED_INCREASE_FACTOR, BALL_MAX_SPEED)).abs() < 1e-4);
    }

    // --- compute_increased_speed ---

    #[test]
    fn compute_increased_speed_multiplies_by_factor() {
        let speed = compute_increased_speed(0.5, 1.05, 1.0);
        assert!((speed - 0.525).abs() < 1e-4);
    }

    #[test]
    fn compute_increased_speed_clamps_at_max() {
        let speed = compute_increased_speed(0.98, 1.05, 1.0);
        assert_eq!(speed, 1.0);
    }

    #[test]
    fn compute_increased_speed_stays_at_max_when_already_capped() {
        let speed = compute_increased_speed(1.0, 1.05, 1.0);
        assert_eq!(speed, 1.0);
    }

    // --- check_scoring_exit ---

    #[test]
    fn no_exit_when_fully_inside_screen() {
        assert_eq!(check_scoring_exit(0.5, 0.02), None);
    }

    #[test]
    fn no_exit_when_touching_left_edge_without_leaving() {
        assert_eq!(check_scoring_exit(0.0, 0.02), None);
    }

    #[test]
    fn exits_left_when_fully_off_left_edge() {
        assert_eq!(check_scoring_exit(-0.02, 0.02), Some(BallExit::Left));
    }

    #[test]
    fn exits_left_when_far_past_left_edge() {
        assert_eq!(check_scoring_exit(-0.1, 0.02), Some(BallExit::Left));
    }

    #[test]
    fn no_exit_when_touching_right_edge_without_leaving() {
        assert_eq!(check_scoring_exit(0.99, 0.02), None);
    }

    #[test]
    fn exits_right_when_fully_off_right_edge() {
        assert_eq!(check_scoring_exit(1.0, 0.02), Some(BallExit::Right));
    }

    #[test]
    fn exits_right_when_far_past_right_edge() {
        assert_eq!(check_scoring_exit(1.2, 0.02), Some(BallExit::Right));
    }

    // --- new_centered_towards ---

    #[test]
    fn new_centered_towards_left_moves_left() {
        let ball = Ball::new_centered_towards(BallExit::Left);
        assert_eq!(ball.position, Vec2::splat(0.5 - BALL_SIZE / 2.0));
        assert!(ball.direction.x < 0.0);
    }

    #[test]
    fn new_centered_towards_right_moves_right() {
        let ball = Ball::new_centered_towards(BallExit::Right);
        assert_eq!(ball.position, Vec2::splat(0.5 - BALL_SIZE / 2.0));
        assert!(ball.direction.x > 0.0);
    }

    #[test]
    fn new_centered_towards_direction_is_unit_vector() {
        let left = Ball::new_centered_towards(BallExit::Left);
        let right = Ball::new_centered_towards(BallExit::Right);
        assert!((left.direction.length_squared() - 1.0).abs() < 1e-4);
        assert!((right.direction.length_squared() - 1.0).abs() < 1e-4);
    }

    #[test]
    fn resolve_collision_with_does_nothing_when_far_apart() {
        let mut ball = ball_at(0.5, 0.5, 0.02, vec2(0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        ball.resolve_collision_with(&paddle);
        assert_eq!(ball.position.x, 0.5);
        assert_eq!(ball.direction, vec2(0.6, 0.8));
        assert_eq!(ball.speed, 0.5);
    }
}
