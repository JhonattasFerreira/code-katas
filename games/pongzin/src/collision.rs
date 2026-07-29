use crate::ball::Ball;
use crate::constants::MAX_BOUNCE_ANGLE;
use crate::paddle::Paddle;
use macroquad::math::{vec2, Vec2};

pub fn resolve_ball_paddle_collision(ball: &Ball, paddle: &Paddle) -> Option<(f32, Vec2)> {
    let overlapping = aabb_overlap(
        ball.position,
        Vec2::splat(ball.size),
        paddle.position,
        paddle.size,
    );

    if !overlapping {
        return None;
    }

    let paddle_center_x = paddle.position.x + paddle.size.x / 2.0;
    let moving_right = ball.center().x >= paddle_center_x;

    let relative_hit = relative_impact_position(ball.center().y, paddle.position.y, paddle.size.y);
    let direction = bounce_direction_from_impact(relative_hit, moving_right);

    let x = if moving_right {
        paddle.position.x + paddle.size.x
    } else {
        paddle.position.x - ball.size
    };

    Some((x, direction))
}

fn aabb_overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    a_pos.x < b_pos.x + b_size.x
        && a_pos.x + a_size.x > b_pos.x
        && a_pos.y < b_pos.y + b_size.y
        && a_pos.y + a_size.y > b_pos.y
}

// Impact position relative to the paddle's center, normalized to
// [-1.0, 1.0] (-1 = top, 0 = center, 1 = bottom). The clamp covers
// overlaps that exceed the paddle's bounds (large ball / high dt).
fn relative_impact_position(ball_center_y: f32, paddle_y: f32, paddle_height: f32) -> f32 {
    let half_height = paddle_height / 2.0;
    let paddle_center_y = paddle_y + half_height;
    ((ball_center_y - paddle_center_y) / half_height).clamp(-1.0, 1.0)
}

// Maps the impact point to a unit direction vector: the closer to the
// paddle's edges, the wider the exit angle (up to MAX_BOUNCE_ANGLE).
// cos²+sin² = 1, so the vector is already normalized.
fn bounce_direction_from_impact(relative_hit: f32, moving_right: bool) -> Vec2 {
    let angle = relative_hit * MAX_BOUNCE_ANGLE;
    let x_magnitude = angle.cos();
    let y = angle.sin();
    let x = if moving_right { x_magnitude } else { -x_magnitude };
    vec2(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{ball_at, paddle_at};

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    fn vec2_approx_eq(a: Vec2, b: Vec2) -> bool {
        (a - b).length() < 1e-4
    }

    #[test]
    fn no_collision_when_far_apart() {
        let ball = ball_at(0.5, 0.5, 0.02, vec2(0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        let result = resolve_ball_paddle_collision(&ball, &paddle);
        assert_eq!(result, None);
    }

    #[test]
    fn no_collision_when_overlapping_only_in_x_not_y() {
        // paddle at y=0.5..0.65, ball aligned in x but at y=0.9 (out of range)
        let ball = ball_at(0.03, 0.9, 0.02, vec2(-0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        let result = resolve_ball_paddle_collision(&ball, &paddle);
        assert_eq!(result, None);
    }

    #[test]
    fn no_collision_when_overlapping_only_in_y_not_x() {
        // aligned in y, but ball far away in x
        let ball = ball_at(0.5, 0.55, 0.02, vec2(-0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        let result = resolve_ball_paddle_collision(&ball, &paddle);
        assert_eq!(result, None);
    }

    #[test]
    fn no_collision_when_edges_touch_exactly() {
        // ball.x + ball.size == paddle.x (touching from the right, no overlap)
        let ball = ball_at(0.0, 0.55, 0.02, vec2(-0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        let result = resolve_ball_paddle_collision(&ball, &paddle);
        assert_eq!(result, None);
    }

    // --- relative_impact_position ---

    #[test]
    fn relative_impact_at_paddle_center_is_zero() {
        assert_eq!(relative_impact_position(0.575, 0.5, 0.15), 0.0);
    }

    #[test]
    fn relative_impact_at_top_edge_is_negative_one() {
        assert!(approx_eq(relative_impact_position(0.5, 0.5, 0.15), -1.0));
    }

    #[test]
    fn relative_impact_at_bottom_edge_is_one() {
        assert!(approx_eq(relative_impact_position(0.65, 0.5, 0.15), 1.0));
    }

    #[test]
    fn relative_impact_at_intermediate_point_is_proportional() {
        // halfway between the center (0.575) and the top (0.5)
        assert!(approx_eq(
            relative_impact_position(0.575 - 0.0375, 0.5, 0.15),
            -0.5
        ));
    }

    #[test]
    fn relative_impact_beyond_top_edge_clamps_to_negative_one() {
        assert_eq!(relative_impact_position(0.4, 0.5, 0.15), -1.0);
    }

    #[test]
    fn relative_impact_beyond_bottom_edge_clamps_to_one() {
        assert_eq!(relative_impact_position(0.8, 0.5, 0.15), 1.0);
    }

    // --- bounce_direction_from_impact ---

    #[test]
    fn bounce_direction_at_center_moving_right_is_straight() {
        let direction = bounce_direction_from_impact(0.0, true);
        assert_eq!(direction, vec2(1.0, 0.0));
    }

    #[test]
    fn bounce_direction_at_center_moving_left_is_straight() {
        let direction = bounce_direction_from_impact(0.0, false);
        assert_eq!(direction, vec2(-1.0, 0.0));
    }

    #[test]
    fn bounce_direction_at_bottom_edge_moving_right_is_max_angle_downward() {
        let direction = bounce_direction_from_impact(1.0, true);
        assert!(vec2_approx_eq(
            direction,
            vec2(MAX_BOUNCE_ANGLE.cos(), MAX_BOUNCE_ANGLE.sin())
        ));
    }

    #[test]
    fn bounce_direction_at_top_edge_moving_right_is_max_angle_upward() {
        let direction = bounce_direction_from_impact(-1.0, true);
        assert!(vec2_approx_eq(
            direction,
            vec2(MAX_BOUNCE_ANGLE.cos(), -MAX_BOUNCE_ANGLE.sin())
        ));
    }

    #[test]
    fn bounce_direction_at_bottom_edge_moving_left_is_max_angle_downward() {
        let direction = bounce_direction_from_impact(1.0, false);
        assert!(vec2_approx_eq(
            direction,
            vec2(-MAX_BOUNCE_ANGLE.cos(), MAX_BOUNCE_ANGLE.sin())
        ));
    }

    #[test]
    fn bounce_direction_is_always_a_unit_vector() {
        let direction = bounce_direction_from_impact(0.5, true);
        assert!(approx_eq(direction.length_squared(), 1.0));
    }

    // --- resolve_ball_paddle_collision (integration) ---

    #[test]
    fn bounces_straight_when_hitting_paddle_center() {
        // ball_center_y (0.565 + 0.01 = 0.575) aligned with the paddle's center
        let ball = ball_at(0.03, 0.565, 0.02, vec2(-0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        let (x, direction) = resolve_ball_paddle_collision(&ball, &paddle).unwrap();
        assert_eq!(x, 0.04);
        assert!(vec2_approx_eq(direction, vec2(1.0, 0.0)));
    }

    #[test]
    fn bounces_upward_when_hitting_paddle_top_edge() {
        // ball_center_y (0.49 + 0.01 = 0.5) aligned with the paddle's top edge
        let ball = ball_at(0.03, 0.49, 0.02, vec2(-0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        let (x, direction) = resolve_ball_paddle_collision(&ball, &paddle).unwrap();
        assert_eq!(x, 0.04);
        assert!(vec2_approx_eq(
            direction,
            vec2(MAX_BOUNCE_ANGLE.cos(), -MAX_BOUNCE_ANGLE.sin())
        ));
    }

    #[test]
    fn bounces_downward_when_hitting_paddle_bottom_edge() {
        // ball_center_y (0.64 + 0.01 = 0.65) aligned with the paddle's bottom edge
        let ball = ball_at(0.03, 0.64, 0.02, vec2(-0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        let (x, direction) = resolve_ball_paddle_collision(&ball, &paddle).unwrap();
        assert_eq!(x, 0.04);
        assert!(vec2_approx_eq(
            direction,
            vec2(MAX_BOUNCE_ANGLE.cos(), MAX_BOUNCE_ANGLE.sin())
        ));
    }

    #[test]
    fn bounces_off_right_face_of_paddle() {
        // ball overlapping, coming from the paddle's right side (ball center >= paddle center)
        let ball = ball_at(0.03, 0.55, 0.02, vec2(-0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.02, 0.15);
        let (x, direction) = resolve_ball_paddle_collision(&ball, &paddle).unwrap();
        let expected = bounce_direction_from_impact(-0.2, true);
        assert_eq!(x, 0.02 + 0.02);
        assert!(vec2_approx_eq(direction, expected));
    }

    #[test]
    fn bounces_off_left_face_of_paddle() {
        // ball overlapping, coming from the paddle's left side (ball center < paddle center)
        let ball = ball_at(0.01, 0.55, 0.02, vec2(0.6, 0.8));
        let paddle = paddle_at(0.02, 0.5, 0.03, 0.15);
        let (x, direction) = resolve_ball_paddle_collision(&ball, &paddle).unwrap();
        let expected = bounce_direction_from_impact(-0.2, false);
        assert_eq!(x, 0.02 - 0.02);
        assert!(vec2_approx_eq(direction, expected));
    }
}
