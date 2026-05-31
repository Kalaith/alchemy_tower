use crate::content::ui_format;

pub(super) fn archive_filter_status(filter_label: &str) -> String {
    ui_format("archive_filter_status", &[("mode", filter_label)])
}
