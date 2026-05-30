use super::gameplay_overlay_types::{OverlayScreen, ARCHIVE_TABS};
use super::gameplay_types::GameplayState;

impl GameplayState {
    pub(super) fn overlay(&self) -> Option<&OverlayScreen> {
        self.ui.current.as_ref()
    }

    pub(super) fn set_overlay(&mut self, overlay: OverlayScreen) {
        self.ui.current = Some(overlay);
    }

    pub(super) fn clear_overlay(&mut self) {
        self.ui.current = None;
    }

    pub(super) fn dialogue_npc_id(&self) -> Option<&str> {
        match self.overlay() {
            Some(OverlayScreen::Dialogue(npc_id)) => Some(npc_id.as_str()),
            _ => None,
        }
    }

    pub(super) fn archive_tab_id(&self) -> &'static str {
        ARCHIVE_TABS[self.ui.archive_tab]
    }

    pub(super) fn archive_tabs(&self) -> &'static [&'static str] {
        &ARCHIVE_TABS
    }

    pub(super) fn archive_tab_selected(&self, index: usize) -> bool {
        index == self.ui.archive_tab
    }

    pub(super) fn archive_selected_index(&self, item_count: usize) -> usize {
        self.ui.archive_index.min(item_count.saturating_sub(1))
    }

    pub(super) fn shop_buy_tab_active(&self) -> bool {
        self.ui.shop_buy_tab
    }

    pub(super) fn shop_sell_tab_active(&self) -> bool {
        !self.ui.shop_buy_tab
    }

    pub(super) fn shop_item_selected(&self, index: usize) -> bool {
        index == self.ui.shop_index
    }

    pub(super) fn selected_shop_entry<'a, T>(&self, entries: &'a [T]) -> Option<&'a T> {
        entries.get(self.ui.shop_index)
    }

    pub(super) fn rune_recipe_selected(&self, index: usize) -> bool {
        index == self.ui.rune_index
    }

    pub(super) fn quest_board_entry_selected(&self, index: usize) -> bool {
        index == self.ui.shop_index
    }

    pub(super) fn journal_tab_selected(&self, index: usize) -> bool {
        index == self.ui.journal_tab
    }

    pub(super) fn journal_tab_index(&self) -> usize {
        self.ui.journal_tab
    }

    pub(super) fn greenhouse_journal_unlocked(&self) -> bool {
        self.progression
            .completed_quests
            .contains("entry_to_greenhouse")
    }
}
