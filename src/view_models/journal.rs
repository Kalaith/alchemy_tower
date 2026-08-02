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
    /// Which page of the shelf is on screen, when there is more than one.
    pub(crate) page_text: Option<String>,
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

/// A herb reduced to one line for the list column. The full block is shown for
/// the selected herb only — at full detail the column had room for about one of
/// twenty-nine, and cut the rest without saying so.
pub(crate) struct JournalHerbRowView {
    pub(crate) title: String,
    pub(crate) state_line: String,
    pub(crate) selected: bool,
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
    pub(crate) notes_title: &'static str,
    pub(crate) active_summary: String,
    pub(crate) milestone_rows: Vec<JournalMilestoneStatusView>,
    /// Titles of the recorded beats, newest first, windowed to what the box
    /// holds. The whole record is walkable — it used to be the last five and
    /// nothing else, with fifty-odd authored beats behind them.
    pub(crate) note_rows: Vec<JournalNoteRowView>,
    pub(crate) note_range_text: Option<String>,
    /// The selected beat's paragraph. The title is not repeated here — the
    /// row above it is already highlighted and carries it.
    pub(crate) note_detail: Option<String>,
}

pub(crate) struct JournalMilestoneStatusView {
    pub(crate) title: String,
    pub(crate) detail: String,
}

pub(crate) struct JournalNoteRowView {
    pub(crate) title: String,
    pub(crate) selected: bool,
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
    /// Set only when there are more routes than the column shows.
    pub(crate) route_range_text: Option<String>,
    /// The description of the selected route. Route prose is a paragraph and
    /// the column is 380px wide, so showing every one at once left room for two.
    pub(crate) route_detail: Option<String>,
    pub(crate) herb_memories: JournalHerbMemoriesView,
    pub(crate) route_progress: JournalRouteProgressView,
}

pub(crate) struct JournalRouteRowView {
    pub(crate) title: String,
    pub(crate) selected: bool,
}

pub(crate) struct JournalHerbMemoriesView {
    pub(crate) title: &'static str,
    pub(crate) empty_text: String,
    /// Set only when there are more herbs than the column shows.
    pub(crate) range_text: Option<String>,
    pub(crate) rows: Vec<JournalHerbRowView>,
    /// The full block, for the selected herb only.
    pub(crate) detail: Option<JournalHerbMemoryView>,
}

pub(crate) struct JournalHerbMemoryView {
    pub(crate) title: String,
    pub(crate) state_line: String,
    pub(crate) route_line: String,
    pub(crate) summary: String,
    pub(crate) conditions: String,
    /// Which brews this ingredient feeds — names the recipes the player has
    /// discovered and counts the ones still waiting to be found.
    pub(crate) used_in_text: Option<String>,
    pub(crate) best_specimen_text: Option<String>,
    pub(crate) variant_text: Option<String>,
    pub(crate) note_text: Option<String>,
}

pub(crate) struct JournalRouteProgressView {
    pub(crate) all_restored_text: Option<String>,
    pub(crate) locked_lines: Vec<String>,
}
