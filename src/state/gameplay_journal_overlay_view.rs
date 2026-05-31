use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::view_models::journal::JournalChromeView;

impl GameplayState {
    pub(super) fn journal_chrome_view(&self) -> JournalChromeView {
        JournalChromeView {
            title: ui_copy("overlay_journal_title"),
            close_label: ui_copy("overlay_close"),
            current_conditions_text: ui_format(
                "overlay_current_conditions",
                &[
                    ("season", self.current_season()),
                    ("weather", self.current_weather()),
                ],
            ),
            tabs: self.journal_tabs(),
            footer_text: ui_format(
                "overlay_journal_footer",
                &[
                    ("switch", &input_bindings().navigation.switch),
                    ("close", &input_bindings().global.cancel),
                    ("journal", &input_bindings().global.journal),
                ],
            ),
        }
    }
}
