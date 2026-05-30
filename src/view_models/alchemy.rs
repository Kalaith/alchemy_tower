pub(crate) struct AlchemyFormulaePanelView {
    pub(crate) empty_text: String,
    pub(crate) rows: Vec<AlchemyFormulaRowView>,
}

pub(crate) struct AlchemyFormulaRowView {
    pub(crate) title: String,
    pub(crate) meta: String,
    pub(crate) detail: String,
    pub(crate) lore_note: String,
}

pub(crate) struct AlchemyMaterialsPanelView {
    pub(crate) sort_label: String,
    pub(crate) empty_text: String,
    pub(crate) rows: Vec<AlchemyMaterialRowView>,
}

pub(crate) struct AlchemyMaterialRowView {
    pub(crate) title: String,
    pub(crate) detail: String,
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
    pub(crate) state: AlchemyPreviewPanelState,
}

pub(crate) struct AlchemyResolvedPreviewView {
    pub(crate) title: String,
    pub(crate) output_line: String,
    pub(crate) quality_line: String,
    pub(crate) traits_line: String,
    pub(crate) read_line: String,
    pub(crate) requirements_line: Option<String>,
    pub(crate) process_flags_line: Option<String>,
    pub(crate) failure_reasons: Vec<String>,
    pub(crate) detail: String,
    pub(crate) has_recipe: bool,
}

pub(crate) struct AlchemySlotsPanelView {
    pub(crate) process_text: String,
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
