use crate::content::ui_format;

pub(super) fn sort_status(sort_label: &str) -> String {
    ui_format("inventory_sort_status", &[("mode", sort_label)])
}
