use crate::constants::AI_DEADZONE;
use crate::input::MoveIntent;

pub fn chase_intent(paddle_y: f32, paddle_height: f32, ball_center_y: f32) -> MoveIntent {
    let paddle_center_y = paddle_y + paddle_height / 2.0;
    let diff = ball_center_y - paddle_center_y;

    if diff < -AI_DEADZONE {
        MoveIntent::Up
    } else if diff > AI_DEADZONE {
        MoveIntent::Down
    } else {
        MoveIntent::Idle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ball_above_paddle_center_moves_up() {
        assert_eq!(chase_intent(0.5, 0.15, 0.5), MoveIntent::Up);
    }

    #[test]
    fn ball_below_paddle_center_moves_down() {
        assert_eq!(chase_intent(0.5, 0.15, 0.7), MoveIntent::Down);
    }

    #[test]
    fn ball_exactly_at_paddle_center_is_idle() {
        assert_eq!(chase_intent(0.5, 0.15, 0.575), MoveIntent::Idle);
    }

    #[test]
    fn works_with_different_paddle_heights() {
        // center at 0.2 + 0.4 / 2.0 = 0.4
        assert_eq!(chase_intent(0.2, 0.4, 0.35), MoveIntent::Up);
        assert_eq!(chase_intent(0.2, 0.4, 0.4), MoveIntent::Idle);
        assert_eq!(chase_intent(0.2, 0.4, 0.45), MoveIntent::Down);
    }

    #[test]
    fn small_difference_inside_deadzone_is_idle() {
        assert_eq!(chase_intent(0.5, 0.15, 0.58), MoveIntent::Idle);
        assert_eq!(chase_intent(0.5, 0.15, 0.57), MoveIntent::Idle);
    }

    #[test]
    fn difference_exactly_at_deadzone_edge_is_idle() {
        assert_eq!(chase_intent(0.5, 0.15, 0.575 + AI_DEADZONE), MoveIntent::Idle);
        assert_eq!(chase_intent(0.5, 0.15, 0.575 - AI_DEADZONE), MoveIntent::Idle);
    }

    #[test]
    fn difference_just_past_deadzone_edge_moves() {
        assert_eq!(chase_intent(0.5, 0.15, 0.575 + AI_DEADZONE + 0.001), MoveIntent::Down);
        assert_eq!(chase_intent(0.5, 0.15, 0.575 - AI_DEADZONE - 0.001), MoveIntent::Up);
    }
}
