use crate::content::ui_format;

pub(super) fn new_journal_note(title: &str) -> String {
    ui_format("gameplay_new_journal_note", &[("title", title)])
}
