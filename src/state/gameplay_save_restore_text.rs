use crate::content::{narrative_text, ui_format};

pub(super) fn incompatible_version(found: u32, expected: u32) -> String {
    ui_format(
        "gameplay_save_version_incompatible",
        &[
            ("found", &found.to_string()),
            ("expected", &expected.to_string()),
        ],
    )
}

pub(super) fn unknown_area() -> String {
    narrative_text().statuses.save_unknown_area.clone()
}
