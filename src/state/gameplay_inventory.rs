use super::GameplayState;

#[path = "gameplay_inventory_status_text.rs"]
mod inventory_status_text;

impl GameplayState {
    pub(super) fn inventory_sort_label(&self) -> &'static str {
        self.ui.inventory_sort_mode.label()
    }

    pub(super) fn cycle_inventory_sort_mode(&mut self) {
        self.ui.inventory_sort_mode = self.ui.inventory_sort_mode.next();
        self.runtime.status_text = inventory_status_text::sort_status(self.inventory_sort_label());
    }
}
