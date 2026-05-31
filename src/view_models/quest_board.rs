pub(crate) struct QuestBoardOverlayView {
    pub(crate) title: String,
    pub(crate) subtitle: String,
    pub(crate) available_title: String,
    pub(crate) empty_text: String,
    pub(crate) entries: Vec<QuestBoardOverlayEntry>,
    pub(crate) locked_title: String,
    pub(crate) locked_text: String,
    pub(crate) active_title: String,
    pub(crate) active_text: String,
    pub(crate) footer_text: String,
}

pub(crate) struct QuestBoardOverlayEntry {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) meta: String,
    pub(crate) selected: bool,
}
