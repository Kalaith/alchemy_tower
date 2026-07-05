pub(crate) struct AlchemyChromeView {
    pub(crate) title: &'static str,
    pub(crate) subtitle: String,
    pub(crate) footer_text: String,
    pub(crate) close_label: String,
    pub(crate) action_buttons: AlchemyActionButtonsView,
}

pub(crate) struct AlchemyActionButtonsView {
    pub(crate) sort_label: &'static str,
    pub(crate) clear_label: &'static str,
    pub(crate) repeat_label: &'static str,
    pub(crate) brew_label: &'static str,
}

pub(crate) struct AlchemyFormulaePanelView {
    pub(crate) title: &'static str,
    pub(crate) empty_text: String,
    pub(crate) rows: Vec<AlchemyFormulaRowView>,
}

pub(crate) struct AlchemyFormulaRowView {
    pub(crate) title: String,
    pub(crate) detail: String,
}

pub(crate) struct AlchemyMaterialsPanelView {
    pub(crate) title: &'static str,
    pub(crate) sort_text: String,
    pub(crate) empty_text: String,
    pub(crate) rows: Vec<AlchemyMaterialRowView>,
}

pub(crate) struct AlchemyMaterialRowView {
    pub(crate) title: String,
    pub(crate) meta: String,
    pub(crate) selected: bool,
    pub(crate) enabled: bool,
}

pub(crate) enum AlchemyPreviewPanelState {
    EmptySelection,
    NoStation,
    Resolved(AlchemyResolvedPreviewView),
}

pub(crate) struct AlchemyPreviewPanelView {
    pub(crate) title: &'static str,
    pub(crate) empty_text: &'static str,
    pub(crate) state: AlchemyPreviewPanelState,
}

pub(crate) struct AlchemyResolvedPreviewView {
    pub(crate) title: String,
    /// Present when the projected output satisfies an open quest — connects the
    /// brew at the bench to the townsperson who needs it.
    pub(crate) quest_line: Option<String>,
    pub(crate) output_line: String,
    pub(crate) quality_line: String,
    pub(crate) traits_line: String,
    pub(crate) read_line: String,
    pub(crate) requirements_line: Option<String>,
    pub(crate) process_flags_line: Option<String>,
    pub(crate) failure_reasons_title: &'static str,
    pub(crate) failure_reason_lines: Vec<String>,
    pub(crate) detail: String,
    pub(crate) has_recipe: bool,
}

pub(crate) struct AlchemySlotsPanelView {
    pub(crate) title: &'static str,
    pub(crate) process_text: String,
    pub(crate) stir_label: &'static str,
    pub(crate) timing_label: &'static str,
    pub(crate) catalyst_label: &'static str,
    pub(crate) slots: Vec<AlchemySlotView>,
    pub(crate) catalyst: AlchemyCatalystSlotView,
}

pub(crate) struct AlchemySlotView {
    pub(crate) label: String,
    pub(crate) item_name: String,
    pub(crate) action_text: &'static str,
}

pub(crate) struct AlchemyCatalystSlotView {
    pub(crate) item_name: String,
    pub(crate) action_text: &'static str,
}
