use crate::content::{narrative_text, ui_copy, ui_text};

pub(super) fn closed() -> String {
    ui_text().statuses.closed_archive.clone()
}

pub(super) fn timeline_restored_toast() -> &'static str {
    ui_copy("archive_timeline_restored_toast")
}

pub(super) fn timeline_complete() -> String {
    narrative_text().statuses.archive_timeline_complete.clone()
}

pub(super) fn timeline_incomplete() -> String {
    narrative_text()
        .statuses
        .archive_timeline_incomplete
        .clone()
}
