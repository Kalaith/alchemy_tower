pub(crate) struct RuneOverlayView {
    pub(crate) station_name: String,
    pub(crate) subtitle: String,
    pub(crate) drafts_title: String,
    pub(crate) empty_text: String,
    pub(crate) footer_text: String,
    pub(crate) entries: Vec<RuneOverlayEntry>,
}

pub(crate) struct RuneOverlayEntry {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) meta: String,
    pub(crate) selected: bool,
}
