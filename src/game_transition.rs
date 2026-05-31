use super::game_state::GameState;
use crate::state::StateTransition;

pub(in crate::game) fn apply_transition(
    current_state: GameState,
    transition: StateTransition,
) -> GameState {
    match transition {
        StateTransition::EnterGameplay(gameplay) => GameState::from_gameplay(gameplay),
        StateTransition::ReturnToMenu => GameState::new_menu(),
        StateTransition::Pause => pause_game(current_state),
        StateTransition::ResumeGame => resume_game(current_state),
    }
}

fn pause_game(current_state: GameState) -> GameState {
    match current_state {
        GameState::Gameplay(gameplay) => GameState::pause(gameplay),
        other => other,
    }
}

fn resume_game(current_state: GameState) -> GameState {
    match current_state {
        GameState::Paused(paused) => GameState::resume(paused),
        other => other,
    }
}
