use super::gameplay_save_restore::apply_save_snapshot;
use super::gameplay_save_snapshot::build_save_snapshot;
use super::GameplayState;
use crate::data::GameData;
use crate::save::{exists as save_exists, load as load_save, save as write_save};

pub(super) fn save_slot_exists() -> bool {
    save_exists()
}

pub(super) fn save_slot(state: &GameplayState, data: &GameData) -> Result<(), String> {
    write_save(&build_save_snapshot(state, data))
}

pub(super) fn load_slot(state: &mut GameplayState, data: &GameData) -> Result<(), String> {
    let save = load_save()?;
    apply_save_snapshot(state, data, save)
}
