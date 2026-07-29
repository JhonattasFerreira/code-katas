use crate::ball::BallExit;
use crate::constants::{POINTS_TO_WIN, SERVE_DELAY_DURATION, WINNER_MESSAGE_DURATION};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    Player,
    Cpu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundOutcome {
    Continue,
    ReturnToMenu,
}

pub struct GameState {
    pub player_score: u32,
    pub cpu_score: u32,
    pub winner: Option<Winner>,
    winner_display_timer: f32,
    serve_timer: Option<f32>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player_score: 0,
            cpu_score: 0,
            winner: None,
            winner_display_timer: 0.0,
            serve_timer: None,
        }
    }

    pub fn is_round_over(&self) -> bool {
        self.winner.is_some()
    }

    pub fn register_point(&mut self, exit: BallExit) {
        let (player_score, cpu_score) = apply_point(self.player_score, self.cpu_score, exit);
        self.player_score = player_score;
        self.cpu_score = cpu_score;

        if let Some(winner) = check_winner(self.player_score, self.cpu_score, POINTS_TO_WIN) {
            self.winner = Some(winner);
            self.winner_display_timer = WINNER_MESSAGE_DURATION;
            self.serve_timer = None;
        } else {
            self.serve_timer = Some(SERVE_DELAY_DURATION);
        }
    }

    pub fn is_serving(&self) -> bool {
        self.serve_timer.is_some()
    }

    pub fn serve_countdown(&self) -> Option<f32> {
        self.serve_timer
    }

    pub fn tick_serve(&mut self, dt: f32) {
        if let Some(timer) = self.serve_timer {
            self.serve_timer = tick_serve_timer(timer, dt);
        }
    }

    pub fn update(&mut self, dt: f32) -> RoundOutcome {
        if self.winner.is_none() {
            return RoundOutcome::Continue;
        }

        self.winner_display_timer = tick_winner_timer(self.winner_display_timer, dt);

        if self.winner_display_timer <= 0.0 {
            self.player_score = 0;
            self.cpu_score = 0;
            self.winner = None;
            RoundOutcome::ReturnToMenu
        } else {
            RoundOutcome::Continue
        }
    }
}

fn apply_point(player_score: u32, cpu_score: u32, exit: BallExit) -> (u32, u32) {
    match exit {
        BallExit::Left => (player_score, cpu_score + 1),
        BallExit::Right => (player_score + 1, cpu_score),
    }
}

fn check_winner(player_score: u32, cpu_score: u32, points_to_win: u32) -> Option<Winner> {
    if player_score >= points_to_win {
        Some(Winner::Player)
    } else if cpu_score >= points_to_win {
        Some(Winner::Cpu)
    } else {
        None
    }
}

fn tick_winner_timer(timer: f32, dt: f32) -> f32 {
    timer - dt
}

fn tick_serve_timer(timer: f32, dt: f32) -> Option<f32> {
    let remaining = timer - dt;
    if remaining <= 0.0 {
        None
    } else {
        Some(remaining)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- apply_point ---

    #[test]
    fn right_exit_from_zero_scores_player() {
        assert_eq!(apply_point(0, 0, BallExit::Right), (1, 0));
    }

    #[test]
    fn left_exit_from_zero_scores_cpu() {
        assert_eq!(apply_point(0, 0, BallExit::Left), (0, 1));
    }

    #[test]
    fn apply_point_increments_only_the_correct_side_from_nonzero_score() {
        assert_eq!(apply_point(3, 2, BallExit::Right), (4, 2));
        assert_eq!(apply_point(3, 2, BallExit::Left), (3, 3));
    }

    // --- check_winner ---

    #[test]
    fn no_winner_when_both_scores_below_threshold() {
        assert_eq!(check_winner(2, 3, 5), None);
    }

    #[test]
    fn no_winner_when_one_point_short() {
        assert_eq!(check_winner(4, 0, 5), None);
    }

    #[test]
    fn player_wins_at_threshold() {
        assert_eq!(check_winner(5, 3, 5), Some(Winner::Player));
    }

    #[test]
    fn cpu_wins_at_threshold() {
        assert_eq!(check_winner(3, 5, 5), Some(Winner::Cpu));
    }

    #[test]
    fn player_wins_when_above_threshold() {
        assert_eq!(check_winner(6, 3, 5), Some(Winner::Player));
    }

    // --- tick_winner_timer ---

    #[test]
    fn tick_winner_timer_counts_down() {
        assert_eq!(tick_winner_timer(2.0, 0.5), 1.5);
    }

    #[test]
    fn tick_winner_timer_can_go_negative_when_dt_exceeds_remaining() {
        let result = tick_winner_timer(0.3, 0.5);
        assert!((result - (-0.2)).abs() < 1e-4);
    }

    #[test]
    fn tick_winner_timer_zero_dt_does_not_change_timer() {
        assert_eq!(tick_winner_timer(2.0, 0.0), 2.0);
    }

    // --- GameState ---

    #[test]
    fn new_game_state_starts_zeroed() {
        let state = GameState::new();
        assert_eq!(state.player_score, 0);
        assert_eq!(state.cpu_score, 0);
        assert_eq!(state.winner, None);
    }

    #[test]
    fn register_point_below_threshold_increments_score_without_winner() {
        let mut state = GameState::new();
        state.register_point(BallExit::Right);
        assert_eq!(state.player_score, 1);
        assert_eq!(state.cpu_score, 0);
        assert_eq!(state.winner, None);
    }

    #[test]
    fn register_point_reaching_threshold_sets_winner_and_timer() {
        let mut state = GameState::new();
        for _ in 0..POINTS_TO_WIN {
            state.register_point(BallExit::Right);
        }
        assert_eq!(state.player_score, POINTS_TO_WIN);
        assert_eq!(state.winner, Some(Winner::Player));
    }

    #[test]
    fn update_does_nothing_without_a_winner() {
        let mut state = GameState::new();
        let outcome = state.update(0.5);
        assert_eq!(outcome, RoundOutcome::Continue);
        assert_eq!(state.winner, None);
    }

    #[test]
    fn update_keeps_winner_while_timer_has_not_elapsed() {
        let mut state = GameState::new();
        for _ in 0..POINTS_TO_WIN {
            state.register_point(BallExit::Right);
        }
        let outcome = state.update(WINNER_MESSAGE_DURATION - 0.5);
        assert_eq!(outcome, RoundOutcome::Continue);
        assert_eq!(state.winner, Some(Winner::Player));
        assert_eq!(state.player_score, POINTS_TO_WIN);
    }

    #[test]
    fn update_resets_match_when_timer_elapses() {
        let mut state = GameState::new();
        for _ in 0..POINTS_TO_WIN {
            state.register_point(BallExit::Right);
        }
        let outcome = state.update(WINNER_MESSAGE_DURATION + 0.1);
        assert_eq!(outcome, RoundOutcome::ReturnToMenu);
        assert_eq!(state.winner, None);
        assert_eq!(state.player_score, 0);
        assert_eq!(state.cpu_score, 0);
    }

    #[test]
    fn is_round_over_reflects_winner_presence() {
        let mut state = GameState::new();
        assert!(!state.is_round_over());
        for _ in 0..POINTS_TO_WIN {
            state.register_point(BallExit::Right);
        }
        assert!(state.is_round_over());
    }

    // --- tick_serve_timer ---

    #[test]
    fn tick_serve_timer_counts_down() {
        assert_eq!(tick_serve_timer(1.0, 0.3), Some(0.7));
    }

    #[test]
    fn tick_serve_timer_clears_when_time_runs_out() {
        assert_eq!(tick_serve_timer(0.3, 0.5), None);
    }

    #[test]
    fn tick_serve_timer_clears_exactly_at_zero() {
        assert_eq!(tick_serve_timer(0.5, 0.5), None);
    }

    // --- serve pause on GameState ---

    #[test]
    fn register_point_without_winner_starts_serve_timer() {
        let mut state = GameState::new();
        state.register_point(BallExit::Right);
        assert!(state.is_serving());
        assert_eq!(state.serve_countdown(), Some(SERVE_DELAY_DURATION));
    }

    #[test]
    fn register_point_reaching_threshold_does_not_start_serve_timer() {
        let mut state = GameState::new();
        for _ in 0..POINTS_TO_WIN {
            state.register_point(BallExit::Right);
        }
        assert!(!state.is_serving());
        assert_eq!(state.serve_countdown(), None);
    }

    #[test]
    fn tick_serve_decrements_timer_without_clearing_it_early() {
        let mut state = GameState::new();
        state.register_point(BallExit::Right);
        state.tick_serve(SERVE_DELAY_DURATION - 0.2);
        assert!(state.is_serving());
        assert!((state.serve_countdown().unwrap() - 0.2).abs() < 1e-4);
    }

    #[test]
    fn tick_serve_clears_timer_once_elapsed() {
        let mut state = GameState::new();
        state.register_point(BallExit::Right);
        state.tick_serve(SERVE_DELAY_DURATION + 0.1);
        assert!(!state.is_serving());
        assert_eq!(state.serve_countdown(), None);
    }
}
