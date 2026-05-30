use super::GameplayState;
use crate::alchemy::BrewResolution;
use crate::content::ui_format;

impl GameplayState {
    pub(super) fn brew_is_stable(&self, resolution: &BrewResolution<'_>) -> bool {
        resolution.process_match
            && resolution.minimum_quality_met
            && resolution.minimum_elements_met
    }

    pub(super) fn inventory_sort_label(&self) -> &'static str {
        self.ui.inventory_sort_mode.label()
    }

    pub(super) fn cycle_inventory_sort_mode(&mut self) {
        self.ui.inventory_sort_mode = self.ui.inventory_sort_mode.next();
        self.runtime.status_text = ui_format(
            "inventory_sort_status",
            &[("mode", self.inventory_sort_label())],
        );
    }
}
