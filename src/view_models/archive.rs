pub(crate) struct ArchiveChromeView {
    pub(crate) title: &'static str,
    pub(crate) subtitle: String,
    pub(crate) tabs: Vec<&'static str>,
    pub(crate) footer_text: String,
}

pub(crate) struct ArchiveTimelineSectionView {
    pub(crate) title: String,
    pub(crate) status_title: String,
    pub(crate) recent_milestones: Vec<ArchiveTimelineMilestoneView>,
    pub(crate) status_lines: Vec<String>,
    pub(crate) reconstruction_text: String,
    pub(crate) reconstruction_locked: bool,
}

pub(crate) struct ArchiveTimelineMilestoneView {
    pub(crate) title: String,
    pub(crate) text: String,
}

pub(crate) struct ArchiveDisassemblySectionView {
    pub(crate) title: String,
    pub(crate) selected_inputs_title: String,
    pub(crate) help_text: String,
    pub(crate) empty_text: String,
    pub(crate) entries: Vec<ArchiveDisassemblyRecipeEntry>,
    pub(crate) selected_inputs: Vec<String>,
}

pub(crate) struct ArchiveDisassemblyRecipeEntry {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) meta: String,
    pub(crate) selected: bool,
}

pub(crate) struct ArchiveDuplicationSectionView {
    pub(crate) title: String,
    pub(crate) cost_title: String,
    pub(crate) help_text: String,
    pub(crate) empty_text: String,
    pub(crate) entries: Vec<ArchiveDuplicationItemEntry>,
    pub(crate) detail: Option<ArchiveDuplicationDetailView>,
}

pub(crate) struct ArchiveDuplicationItemEntry {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) meta: String,
    pub(crate) enabled: bool,
    pub(crate) selected: bool,
}

pub(crate) struct ArchiveDuplicationDetailView {
    pub(crate) target_text: String,
    pub(crate) coin_text: String,
    pub(crate) catalyst_text: String,
}

pub(crate) struct ArchiveExperimentsSectionView {
    pub(crate) title: String,
    pub(crate) filter_text: String,
    pub(crate) page_text: Option<String>,
    pub(crate) empty_text: String,
    pub(crate) entries: Vec<ArchiveExperimentEntryView>,
    pub(crate) selected_record: Option<ArchiveExperimentRecordView>,
}

pub(crate) struct ArchiveExperimentEntryView {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) meta: String,
    pub(crate) selected: bool,
}

pub(crate) struct ArchiveExperimentRecordView {
    pub(crate) title: String,
    pub(crate) output_text: String,
    pub(crate) quality_text: String,
    pub(crate) result_text: String,
    pub(crate) catalyst_text: String,
    pub(crate) morph_text: String,
    pub(crate) recipe_memory: Option<ArchiveExperimentRecipeMemoryView>,
}

pub(crate) struct ArchiveExperimentRecipeMemoryView {
    pub(crate) mastery_text: String,
    pub(crate) memory_text: String,
    pub(crate) detail_text: String,
}

pub(crate) struct ArchiveMasterySectionView {
    pub(crate) title: String,
    pub(crate) detail_title: String,
    pub(crate) empty_text: String,
    pub(crate) entries: Vec<ArchiveMasteryRecipeEntry>,
    pub(crate) detail: Option<ArchiveMasteryDetailView>,
}

pub(crate) struct ArchiveMasteryRecipeEntry {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) meta: String,
    pub(crate) selected: bool,
}

pub(crate) struct ArchiveMasteryDetailView {
    pub(crate) title: String,
    pub(crate) stage_text: String,
    pub(crate) best_result_text: Option<String>,
    pub(crate) traits_text: Option<String>,
    pub(crate) last_attempt_text: Option<String>,
    pub(crate) lore_note: String,
}

pub(crate) struct ArchiveMorphsSectionView {
    pub(crate) title: String,
    pub(crate) detail_title: String,
    pub(crate) empty_text: String,
    pub(crate) entries: Vec<ArchiveMorphRecipeEntry>,
    pub(crate) detail: Option<ArchiveMorphDetailView>,
}

pub(crate) struct ArchiveMorphRecipeEntry {
    pub(crate) title: String,
    pub(crate) detail: String,
    pub(crate) meta: String,
    pub(crate) selected: bool,
}

pub(crate) struct ArchiveMorphDetailView {
    pub(crate) last_morph_text: Option<String>,
    pub(crate) targets: Vec<ArchiveMorphTargetView>,
}

pub(crate) struct ArchiveMorphTargetView {
    pub(crate) title: String,
    pub(crate) conditions: String,
}
