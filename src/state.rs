//! Top-level game screens and transitions.

mod gameplay;
mod menu;
mod pause;

pub use gameplay::GameplayState;
pub use menu::MenuState;
pub use pause::PauseState;

pub enum StateTransition {
    EnterGameplay(GameplayState),
    ReturnToMenu,
    Pause,
    ResumeGame,
}
