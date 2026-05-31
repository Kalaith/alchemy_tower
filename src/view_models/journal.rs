pub(crate) struct JournalChromeView {
    pub(crate) title: &'static str,
    pub(crate) close_label: &'static str,
    pub(crate) current_conditions_text: String,
    pub(crate) tabs: Vec<&'static str>,
    pub(crate) footer_text: String,
}

pub(crate) struct JournalBrewsTabView {
    pub(crate) title: &'static str,
    pub(crate) empty_text: String,
    pub(crate) entries: Vec<JournalBrewMemoryView>,
}

pub(crate) struct JournalBrewMemoryView {
    pub(crate) title: String,
    pub(crate) state_line: String,
    pub(crate) recap: String,
    pub(crate) effects_text: Option<String>,
    pub(crate) traits_text: Option<String>,
    pub(crate) best_brew_text: Option<String>,
    pub(crate) formula_text: Option<String>,
    pub(crate) successful_brews_text: Option<String>,
}

pub(crate) struct JournalGreenhouseTabView {
    pub(crate) title: &'static str,
    pub(crate) empty_text: String,
    pub(crate) beds: Vec<JournalGreenhouseBedView>,
}

pub(crate) struct JournalGreenhouseBedView {
    pub(crate) title: String,
    pub(crate) summary: String,
}

pub(crate) struct JournalNotesTabView {
    pub(crate) title: &'static str,
    pub(crate) active_title: &'static str,
    pub(crate) milestones_title: &'static str,
    pub(crate) active_summary: String,
    pub(crate) milestone_rows: Vec<JournalMilestoneStatusView>,
    pub(crate) recent_milestones: Vec<JournalRecentMilestoneView>,
}

pub(crate) struct JournalMilestoneStatusView {
    pub(crate) title: String,
    pub(crate) detail: String,
}

pub(crate) struct JournalRecentMilestoneView {
    pub(crate) title: String,
    pub(crate) text: String,
}

pub(crate) struct JournalRapportTabView {
    pub(crate) title: &'static str,
    pub(crate) rows: Vec<JournalRapportRowView>,
}

pub(crate) struct JournalRapportRowView {
    pub(crate) title: String,
    pub(crate) now_text: String,
    pub(crate) later_text: String,
    pub(crate) usually_text: String,
}

pub(crate) struct JournalRoutesTabView {
    pub(crate) title: &'static str,
    pub(crate) progress_title: &'static str,
    pub(crate) route_rows: Vec<JournalRouteRowView>,
    pub(crate) herb_memories: JournalHerbMemoriesView,
    pub(crate) route_progress: JournalRouteProgressView,
}

pub(crate) struct JournalRouteRowView {
    pub(crate) title: String,
    pub(crate) detail: String,
}

pub(crate) struct JournalHerbMemoriesView {
    pub(crate) title: &'static str,
    pub(crate) empty_text: String,
    pub(crate) entries: Vec<JournalHerbMemoryView>,
}

pub(crate) struct JournalHerbMemoryView {
    pub(crate) title: String,
    pub(crate) state_line: String,
    pub(crate) route_line: String,
    pub(crate) summary: String,
    pub(crate) conditions: String,
    pub(crate) best_specimen_text: Option<String>,
    pub(crate) variant_text: Option<String>,
    pub(crate) note_text: Option<String>,
}

pub(crate) struct JournalRouteProgressView {
    pub(crate) all_restored_text: Option<String>,
    pub(crate) locked_lines: Vec<String>,
}
