//! High-level game loop orchestration.

use macroquad::prelude::{clear_background, Color};

#[path = "game_bootstrap.rs"]
mod game_bootstrap;
#[path = "game_state.rs"]
mod game_state;
#[path = "game_transition.rs"]
mod game_transition;

use crate::art::ArtAssets;
use crate::audio::AudioAssets;
use crate::data::GameData;
use crate::state::GameplayState;

use self::game_state::GameState;
use self::game_transition::apply_transition;

pub(crate) struct Game {
    data: GameData,
    art: ArtAssets,
    audio: AudioAssets,
    state: Option<GameState>,
}

impl Game {
    pub(crate) async fn new() -> Self {
        game_bootstrap::load_game().await
    }

    pub(crate) fn update(&mut self) {
        let Some(mut current_state) = self.state.take() else {
            return;
        };
        let transition = current_state.update(&self.data, &self.audio);

        self.state = Some(match transition {
            Some(next) => apply_transition(current_state, next),
            None => current_state,
        });
    }

    pub(crate) fn draw(&self) {
        clear_background(Color::from_rgba(22, 24, 30, 255));

        let Some(state) = self.state.as_ref() else {
            return;
        };
        state.draw(&self.data, &self.art);
    }

    /// Seed a specific scene for the screenshot harness.
    pub(crate) fn begin_capture_scene(&mut self, scene: &str) {
        // "<scene>+quiet" captures the same moment with the quiet HUD, which is
        // the only way to compare the two without a person holding a camera.
        let scene = match scene.strip_suffix("+quiet") {
            Some(base) => {
                crate::ui::set_quiet_hud(true);
                base
            }
            None => scene,
        };
        self.state = Some(match scene {
            "gameplay" => GameState::from_gameplay(GameplayState::new(&self.data)),
            "paused" => GameState::pause(GameplayState::new(&self.data)),
            // "afterword:<npc_id>" opens a conversation with the whole story
            // already behind it — the only way to look at what the valley says
            // once the epilogue has run.
            other if other.starts_with("afterword:") => {
                let npc_id = other.strip_prefix("afterword:").unwrap_or("crow_guide");
                let mut gameplay = GameplayState::new(&self.data);
                gameplay.open_dialogue_after_everything(&self.data, npc_id);
                GameState::from_gameplay(gameplay)
            }
            // "dialogue", "dialogue:<npc_id>", or "dialogue:<npc_id>:<beat>"
            // opens a conversation overlay; the beat index skips past that many
            // finished steps of the townsperson's arc.
            other if other.starts_with("dialogue") => {
                let target = other.strip_prefix("dialogue:").unwrap_or("mira_apothecary");
                let (npc_id, beat) = match target.split_once(':') {
                    Some((npc_id, beat)) => (npc_id, beat.parse::<usize>().unwrap_or(0)),
                    None => (target, 0),
                };
                let mut gameplay = GameplayState::new(&self.data);
                if beat > 0 {
                    gameplay.open_dialogue_at_arc_beat(&self.data, npc_id, beat);
                } else {
                    gameplay.open_dialogue_with(npc_id);
                }
                GameState::from_gameplay(gameplay)
            }
            // "area:<area_id>" or "area:<area_id>:<day>" stands in a room with
            // every gate satisfied. The day picks the season, weather and daily
            // roll, which is what decides who is actually growing there.
            other if other.starts_with("area:") => {
                let target = other.strip_prefix("area:").unwrap_or_default();
                let (area_id, rest) = match target.split_once(':') {
                    Some((area_id, rest)) => (area_id, rest),
                    None => (target, ""),
                };
                let (day, time_window) = match rest.split_once(':') {
                    Some((day, window)) => (day.parse::<u32>().unwrap_or(0), window),
                    None => (rest.parse::<u32>().unwrap_or(0), "morning"),
                };
                let mut gameplay = GameplayState::new(&self.data);
                gameplay.preview_area(&self.data, area_id, day, time_window);
                GameState::from_gameplay(gameplay)
            }
            // "archive:<tab>:<index>" opens the archive console on a tab with a
            // selection deliberately past the first page.
            other if other.starts_with("archive") => {
                let target = other.strip_prefix("archive:").unwrap_or("2:7");
                let (tab, index) = match target.split_once(':') {
                    Some((tab, index)) => (
                        tab.parse::<usize>().unwrap_or(2),
                        index.parse::<usize>().unwrap_or(7),
                    ),
                    None => (2, 7),
                };
                let mut gameplay = GameplayState::new(&self.data);
                gameplay.open_archive_sample(&self.data, tab, index);
                GameState::from_gameplay(gameplay)
            }
            // "ending" opens the epilogue with every beat it can react to.
            "ending" => {
                let mut gameplay = GameplayState::new(&self.data);
                gameplay.open_full_ending();
                GameState::from_gameplay(gameplay)
            }
            // "brew" opens the alchemy bench with a sample filled cauldron.
            "brew" => {
                let mut gameplay = GameplayState::new(&self.data);
                gameplay.open_alchemy_sample_brew(&self.data);
                GameState::from_gameplay(gameplay)
            }
            // "compound" opens a bench that takes bottles, holding graded ones.
            "compound" => {
                let mut gameplay = GameplayState::new(&self.data);
                gameplay.open_compound_brew_sample(&self.data);
                GameState::from_gameplay(gameplay)
            }
            // "toasts" raises the event banners so they can be looked at; they
            // last a couple of seconds, so there is no catching one by hand.
            "toasts" => {
                let mut gameplay = GameplayState::new(&self.data);
                gameplay.show_event_toasts_sample(&self.data);
                GameState::from_gameplay(gameplay)
            }
            // "rune" opens the rune workbench holding every reworkable potion.
            "rune" => {
                let mut gameplay = GameplayState::new(&self.data);
                gameplay.open_rune_bench_sample(&self.data);
                GameState::from_gameplay(gameplay)
            }
            // "board" opens the quest board with a ready-to-deliver request.
            "board" => {
                let mut gameplay = GameplayState::new(&self.data);
                gameplay.open_quest_board_sample(&self.data);
                GameState::from_gameplay(gameplay)
            }
            // "journal" opens the journal on the herb-memory tab.
            "journal" => {
                let mut gameplay = GameplayState::new(&self.data);
                gameplay.open_journal_sample(&self.data);
                GameState::from_gameplay(gameplay)
            }
            // Default ("menu" or anything else): the boot flow already lands
            // on the main menu, so this also covers unrecognized scene names.
            _ => GameState::new_menu(),
        });
    }
}
