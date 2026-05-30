pub(crate) struct QuestBoardOverlayView {
    pub(crate) empty_text: String,
    pub(crate) entries: Vec<QuestBoardOverlayEntry>,
    pub(crate) locked_text: String,
    pub(crate) active_text: String,
}

pub(crate) struct QuestBoardOverlayEntry {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) meta: String,
    pub(crate) selected: bool,
}
