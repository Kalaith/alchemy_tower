use crate::content::{ui_copy, ui_format};

pub(super) fn active_quest_summary(title: &str, requirements: &str, location: &str) -> String {
    ui_format(
        "journal_active_summary",
        &[
            ("title", title),
            ("requirements", requirements),
            ("location", location),
        ],
    )
}

pub(super) fn greenhouse_status(restored: bool) -> (&'static str, String, bool) {
    (
        ui_copy("milestone_greenhouse_access"),
        if restored {
            ui_copy("milestone_greenhouse_restored").to_owned()
        } else {
            ui_copy("milestone_greenhouse_locked").to_owned()
        },
        restored,
    )
}

pub(super) fn archive_status(recovered: bool, ready: bool) -> (&'static str, String, bool) {
    (
        ui_copy("milestone_archive_reconstruction"),
        if recovered {
            ui_copy("milestone_archive_recovered").to_owned()
        } else if ready {
            ui_copy("milestone_archive_ready").to_owned()
        } else {
            ui_copy("milestone_archive_locked").to_owned()
        },
        recovered || ready,
    )
}

pub(super) fn observatory_status(archive_recovered: bool) -> (&'static str, String, bool) {
    (
        ui_copy("milestone_observatory_access"),
        if archive_recovered {
            ui_copy("milestone_observatory_ready").to_owned()
        } else {
            ui_copy("milestone_observatory_locked").to_owned()
        },
        archive_recovered,
    )
}

pub(super) fn tabs(greenhouse_unlocked: bool) -> Vec<&'static str> {
    let mut tabs = vec![
        ui_copy("journal_tab_routes"),
        ui_copy("journal_tab_notes"),
        ui_copy("journal_tab_brews"),
    ];
    if greenhouse_unlocked {
        tabs.push(ui_copy("journal_tab_greenhouse"));
    }
    tabs.push(ui_copy("journal_tab_rapport"));
    tabs
}
