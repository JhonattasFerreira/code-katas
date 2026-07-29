use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppPhase {
    Menu,
    Playing,
}

pub fn is_point_in_rect(point: Vec2, rect_pos: Vec2, rect_size: Vec2) -> bool {
    point.x >= rect_pos.x
        && point.x <= rect_pos.x + rect_size.x
        && point.y >= rect_pos.y
        && point.y <= rect_pos.y + rect_size.y
}

pub fn start_button_clicked(button_pos: Vec2, button_size: Vec2) -> bool {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let normalized = vec2(mouse_x / screen_width(), mouse_y / screen_height());
    is_point_in_rect(normalized, button_pos, button_size)
}

pub fn is_start_key_pressed(enter_pressed: bool, space_pressed: bool) -> bool {
    enter_pressed || space_pressed
}

pub fn start_requested(button_pos: Vec2, button_size: Vec2) -> bool {
    start_button_clicked(button_pos, button_size)
        || is_start_key_pressed(is_key_pressed(KeyCode::Enter), is_key_pressed(KeyCode::Space))
}

/// Position of a marker traveling clockwise along a rectangle's border, starting
/// at the top-left corner. `t` is the lap progress in [0.0, 1.0], wrapping past 1.0.
pub fn border_marker_position(t: f32, rect_pos: Vec2, rect_size: Vec2) -> Vec2 {
    let perimeter = 2.0 * (rect_size.x + rect_size.y);
    let distance = t.rem_euclid(1.0) * perimeter;

    if distance < rect_size.x {
        vec2(rect_pos.x + distance, rect_pos.y)
    } else if distance < rect_size.x + rect_size.y {
        vec2(rect_pos.x + rect_size.x, rect_pos.y + (distance - rect_size.x))
    } else if distance < 2.0 * rect_size.x + rect_size.y {
        let along_bottom = distance - rect_size.x - rect_size.y;
        vec2(rect_pos.x + rect_size.x - along_bottom, rect_pos.y + rect_size.y)
    } else {
        let along_left = distance - 2.0 * rect_size.x - rect_size.y;
        vec2(rect_pos.x, rect_pos.y + rect_size.y - along_left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_inside_rect_is_contained() {
        assert!(is_point_in_rect(
            vec2(0.5, 0.5),
            vec2(0.4, 0.4),
            vec2(0.2, 0.2)
        ));
    }

    #[test]
    fn point_outside_rect_is_not_contained() {
        assert!(!is_point_in_rect(
            vec2(0.1, 0.1),
            vec2(0.4, 0.4),
            vec2(0.2, 0.2)
        ));
    }

    #[test]
    fn point_on_top_left_edge_is_contained() {
        assert!(is_point_in_rect(
            vec2(0.4, 0.4),
            vec2(0.4, 0.4),
            vec2(0.2, 0.2)
        ));
    }

    #[test]
    fn point_on_bottom_right_edge_is_contained() {
        assert!(is_point_in_rect(
            vec2(0.6, 0.6),
            vec2(0.4, 0.4),
            vec2(0.2, 0.2)
        ));
    }

    #[test]
    fn point_just_past_bottom_right_edge_is_not_contained() {
        assert!(!is_point_in_rect(
            vec2(0.6001, 0.6001),
            vec2(0.4, 0.4),
            vec2(0.2, 0.2)
        ));
    }

    #[test]
    fn only_enter_pressed_requests_start() {
        assert!(is_start_key_pressed(true, false));
    }

    #[test]
    fn only_space_pressed_requests_start() {
        assert!(is_start_key_pressed(false, true));
    }

    #[test]
    fn neither_key_pressed_does_not_request_start() {
        assert!(!is_start_key_pressed(false, false));
    }

    #[test]
    fn marker_at_start_is_top_left_corner() {
        assert_eq!(
            border_marker_position(0.0, vec2(0.4, 0.4), vec2(0.2, 0.1)),
            vec2(0.4, 0.4)
        );
    }

    #[test]
    fn marker_mid_top_edge_moves_right() {
        assert_eq!(
            border_marker_position(1.0 / 6.0, vec2(0.4, 0.4), vec2(0.2, 0.1)),
            vec2(0.5, 0.4)
        );
    }

    #[test]
    fn marker_at_top_right_corner() {
        let marker = border_marker_position(1.0 / 3.0, vec2(0.4, 0.4), vec2(0.2, 0.1));
        assert!((marker - vec2(0.6, 0.4)).length() < 0.0001);
    }

    #[test]
    fn marker_at_bottom_right_corner() {
        assert_eq!(
            border_marker_position(0.5, vec2(0.4, 0.4), vec2(0.2, 0.1)),
            vec2(0.6, 0.5)
        );
    }

    #[test]
    fn marker_at_bottom_left_corner() {
        assert_eq!(
            border_marker_position(5.0 / 6.0, vec2(0.4, 0.4), vec2(0.2, 0.1)),
            vec2(0.4, 0.5)
        );
    }

    #[test]
    fn marker_wraps_back_to_top_left_after_full_lap() {
        assert_eq!(
            border_marker_position(1.0, vec2(0.4, 0.4), vec2(0.2, 0.1)),
            vec2(0.4, 0.4)
        );
    }
}
