use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveIntent {
    Up,
    Down,
    Idle,
}

pub fn resolve_intent(up_pressed: bool, down_pressed: bool) -> MoveIntent {
    match (up_pressed, down_pressed) {
        (true, false) => MoveIntent::Up,
        (false, true) => MoveIntent::Down,
        _ => MoveIntent::Idle,
    }
}

pub fn read_move_intent() -> MoveIntent {
    resolve_intent(is_key_down(KeyCode::Up), is_key_down(KeyCode::Down))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_up_pressed_resolves_to_up() {
        assert_eq!(resolve_intent(true, false), MoveIntent::Up);
    }

    #[test]
    fn only_down_pressed_resolves_to_down() {
        assert_eq!(resolve_intent(false, true), MoveIntent::Down);
    }

    #[test]
    fn neither_pressed_resolves_to_idle() {
        assert_eq!(resolve_intent(false, false), MoveIntent::Idle);
    }

    #[test]
    fn both_pressed_resolves_to_idle() {
        assert_eq!(resolve_intent(true, true), MoveIntent::Idle);
    }
}
