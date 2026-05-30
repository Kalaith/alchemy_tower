use super::game_state::GameState;
use crate::state::{MenuState, PauseState, StateTransition};

pub(in crate::game) fn apply_transition(
    current_state: GameState,
    transition: StateTransition,
) -> GameState {
    match transition {
        StateTransition::EnterGameplay(gameplay) => GameState::Gameplay(gameplay),
        StateTransition::ReturnToMenu => GameState::Menu(MenuState::new()),
        StateTransition::Pause => pause_game(current_state),
        StateTransition::ResumeGame => resume_game(current_state),
    }
}

fn pause_game(current_state: GameState) -> GameState {
    match current_state {
        GameState::Gameplay(gameplay) => GameState::Paused(PauseState::new(gameplay)),
        other => other,
    }
}

fn resume_game(current_state: GameState) -> GameState {
    match current_state {
        GameState::Paused(paused) => GameState::Gameplay(paused.into_gameplay()),
        other => other,
    }
}
