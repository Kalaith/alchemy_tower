//! Top-level game screens and transitions.

mod gameplay;
mod menu;
mod pause;

pub(crate) use gameplay::GameplayState;
pub(crate) use menu::MenuState;
pub(crate) use pause::PauseState;

pub(crate) enum StateTransition {
    EnterGameplay(GameplayState),
    ReturnToMenu,
    Pause,
    ResumeGame,
}
