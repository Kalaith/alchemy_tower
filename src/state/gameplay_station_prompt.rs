use super::GameplayState;
use crate::content::ui_text;
use crate::data::{GameData, StationDefinition, StationKind};

pub(super) fn station_prompt_text(
    state: &GameplayState,
    data: &GameData,
    station: &StationDefinition,
) -> String {
    match station.kind {
        StationKind::Alchemy => alchemy_prompt_text(state),
        StationKind::RestBed => interact_prompt(state, ui_text().prompts.sleep_in_bed.as_str()),
        StationKind::Shop => interact_prompt(state, ui_text().prompts.browse_shop.as_str()),
        StationKind::RuneWorkshop => {
            interact_prompt(state, ui_text().prompts.open_rune_workshop.as_str())
        }
        StationKind::ArchiveConsole => {
            interact_prompt(state, ui_text().prompts.reconstruct_archives.as_str())
        }
        StationKind::EndingFocus => interact_prompt(state, ui_text().prompts.focus_observatory.as_str()),
        StationKind::QuestBoard => quest_board_prompt(state, data),
        StationKind::Planter => state.planter_prompt_text(station),
        StationKind::Habitat => state.habitat_prompt_text(station),
        _ => String::new(),
    }
}

pub(super) fn alchemy_prompt_text(state: &GameplayState) -> String {
    state.alchemy_prompt_copy(ui_text().prompts.open_alchemy.as_str())
}

fn interact_prompt(state: &GameplayState, label: &str) -> String {
    state.interact_prompt_copy("world_prompt_interact", &[("label", label)])
}

fn quest_board_prompt(state: &GameplayState, data: &GameData) -> String {
    if state.available_board_quests(data).is_empty() {
        interact_prompt(state, ui_text().prompts.read_request_board.as_str())
    } else {
        state.interact_prompt_copy("world_prompt_board_new", &[])
    }
}
