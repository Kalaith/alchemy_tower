use super::gameplay_overlay_window::visible_window_start;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::journal::{
    JournalMilestoneStatusView, JournalNoteRowView, JournalNotesTabView,
};

/// Titles are one line each, with the selected beat's paragraph beneath them.
/// Six leaves room for the longest beat in the game underneath — see
/// `the_longest_recorded_beat_fits_the_panel`, which does that arithmetic
/// against the renderer's own numbers.
pub(crate) const VISIBLE_NOTE_ROWS: usize = 6;

impl GameplayState {
    pub(super) fn journal_notes_tab_view(&self, data: &GameData) -> JournalNotesTabView {
        // Newest first: the thing that just happened is what a player opening
        // the journal is usually looking for.
        let notes = self
            .progression
            .journal_milestones
            .iter()
            .rev()
            .collect::<Vec<_>>();
        let total = notes.len();
        let selected = self.ui.journal_index.min(total.saturating_sub(1));
        let start = visible_window_start(selected, total, VISIBLE_NOTE_ROWS);

        JournalNotesTabView {
            title: ui_copy("overlay_tower_notes"),
            active_title: ui_copy("overlay_progress_active"),
            milestones_title: ui_copy("overlay_progress_milestones"),
            notes_title: ui_copy("overlay_recorded_notes"),
            active_summary: self
                .active_quest_summary(data)
                .unwrap_or_else(|| self.next_goal_summary(data)),
            milestone_rows: self
                .milestone_status_lines(data)
                .into_iter()
                .map(|(label, detail, ready)| JournalMilestoneStatusView {
                    title: ui_format(
                        "journal_milestone_status_title",
                        &[
                            ("label", label),
                            (
                                "status",
                                if ready {
                                    ui_copy("overlay_progress_ready")
                                } else {
                                    ui_copy("overlay_progress_locked")
                                },
                            ),
                        ],
                    ),
                    detail,
                })
                .collect(),
            note_rows: notes
                .iter()
                .enumerate()
                .skip(start)
                .take(VISIBLE_NOTE_ROWS)
                .map(|(index, milestone)| JournalNoteRowView {
                    title: milestone.title.clone(),
                    selected: index == selected,
                })
                .collect(),
            note_range_text: (total > VISIBLE_NOTE_ROWS).then(|| {
                ui_format(
                    "journal_showing_range",
                    &[
                        ("first", &(start + 1).to_string()),
                        ("last", &(start + VISIBLE_NOTE_ROWS).min(total).to_string()),
                        ("total", &total.to_string()),
                    ],
                )
            }),
            note_detail: notes.get(selected).map(|milestone| milestone.text.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GameplayState, VISIBLE_NOTE_ROWS};

    /// Seed a state with every beat the game can record, which is what a
    /// finished campaign's journal holds.
    fn full_record(data: &crate::data::GameData) -> GameplayState {
        let mut state = GameplayState::new(data);
        for quest in &data.quests {
            for milestone in &quest.completion_milestones {
                state.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            }
        }
        for recipe in &data.recipes {
            for milestone in &recipe.discovery_milestones {
                state.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            }
        }
        for area in &data.areas {
            for target in &area.apply_targets {
                for milestone in &target.completion_milestones {
                    state.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
                }
            }
        }
        state
    }

    /// The finding this pass exists for. The notes tab drew the **last five**
    /// recorded beats and nothing else, and the archive's timeline the last
    /// seven — so of the fifty-odd beats the game authors, averaging about 240
    /// characters each, everything but the tail was written into the player's
    /// own journal and then permanently out of reach.
    #[test]
    fn every_recorded_note_can_be_read_again() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = full_record(&data);
        let total = state.progression.journal_milestones.len();
        assert!(
            total > VISIBLE_NOTE_ROWS * 3,
            "this only proves anything with a record several pages long, got {total}"
        );

        let mut seen = std::collections::HashSet::new();
        for index in 0..total {
            state.ui.journal_index = index;
            let view = state.journal_notes_tab_view(&data);
            assert!(
                view.note_rows.len() <= VISIBLE_NOTE_ROWS,
                "{} rows drawn at index {index}",
                view.note_rows.len()
            );
            assert!(
                view.note_rows.iter().any(|row| row.selected),
                "nothing selected at index {index}"
            );
            assert!(
                view.note_detail.is_some_and(|text| !text.is_empty()),
                "the selected note at {index} has no words behind it"
            );
            let selected_title = view
                .note_rows
                .into_iter()
                .find(|row| row.selected)
                .map(|row| row.title)
                .expect("something is selected");
            seen.insert(selected_title);
        }

        let titles = state
            .progression
            .journal_milestones
            .iter()
            .map(|milestone| milestone.title.clone())
            .collect::<std::collections::HashSet<_>>();
        let unreachable = titles.difference(&seen).cloned().collect::<Vec<_>>();
        assert!(
            unreachable.is_empty(),
            "recorded beats the journal can never show again: {unreachable:#?}"
        );
    }

    /// Measured off `screenshots/hud/journal_notes.png`: the record column is
    /// about 530px wide and wraps at roughly 66 characters per line at font 18.
    /// Deliberately pessimistic at 58, because a long word wraps early and
    /// costs a line, so a real beat takes at least as many lines as this says.
    const CHARS_PER_LINE: usize = 58;
    /// The shortest window the game is laid out for.
    const REFERENCE_SCREEN_HEIGHT: f32 = 720.0;

    /// The other half of the same defect. Beat prose runs to 413 characters and
    /// the section it was drawn into had about eighty pixels, with the bounds
    /// check asking only whether the *first* line fitted — which is how the herb
    /// entry came to be running its last line through the panel below it. This
    /// walks every authored beat against the real layout numbers.
    #[test]
    fn the_longest_recorded_beat_fits_the_panel() {
        use crate::ui::{note_detail_top, NOTES_BOTTOM_MARGIN, NOTE_DETAIL_LINE_HEIGHT};

        let data = crate::data::load_embedded().expect("embedded game data should load");
        let panel_y = 72.0;
        let panel_h = REFERENCE_SCREEN_HEIGHT - 144.0;
        let bottom_limit = panel_y + panel_h - NOTES_BOTTOM_MARGIN;
        let detail_top = note_detail_top(panel_y, VISIBLE_NOTE_ROWS);

        let mut clipped = Vec::new();
        let mut checked = 0usize;
        let mut walk = |id: &str, text: &str| {
            checked += 1;
            let lines = text.len().div_ceil(CHARS_PER_LINE).max(1) as f32;
            let needed = detail_top + lines * NOTE_DETAIL_LINE_HEIGHT;
            if needed > bottom_limit {
                clipped.push(format!(
                    "{id}: {} chars needs {needed:.0} against a {bottom_limit:.0} floor",
                    text.len()
                ));
            }
        };
        for quest in &data.quests {
            for milestone in &quest.completion_milestones {
                walk(&milestone.id, &milestone.text);
            }
        }
        for recipe in &data.recipes {
            for milestone in &recipe.discovery_milestones {
                walk(&milestone.id, &milestone.text);
            }
        }
        for area in &data.areas {
            for target in &area.apply_targets {
                for milestone in &target.completion_milestones {
                    walk(&milestone.id, &milestone.text);
                }
            }
        }

        assert!(checked > 40, "only {checked} beats walked");
        clipped.sort();
        assert!(
            clipped.is_empty(),
            "recorded beats whose text falls out of the panel:
{clipped:#?}"
        );
    }

    /// The section only says how much is out of sight when something is.
    #[test]
    fn the_count_appears_only_when_the_record_is_longer_than_the_box() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let full = full_record(&data);
        assert!(
            full.journal_notes_tab_view(&data).note_range_text.is_some(),
            "a full record needs a count"
        );

        let fresh = GameplayState::new(&data);
        assert!(
            fresh.progression.journal_milestones.len() <= VISIBLE_NOTE_ROWS,
            "a new game starts with a short record, or this proves nothing"
        );
        assert!(
            fresh
                .journal_notes_tab_view(&data)
                .note_range_text
                .is_none(),
            "a short record should not claim anything is hidden"
        );
    }
}
