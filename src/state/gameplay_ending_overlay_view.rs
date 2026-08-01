use super::GameplayState;
use crate::content::{narrative_text, ui_copy, NarrativeEpilogueBeat};
use crate::view_models::ending::EndingOverlayView;

/// The panel holds roughly fourteen lines at the design size and the fixed
/// paragraph already uses five. An epilogue that listed everything would be a
/// checklist and would overrun the box; the heaviest few things the player
/// actually did read better than all of them.
pub(crate) const MAX_EPILOGUE_BEATS: usize = 3;

impl GameplayState {
    pub(super) fn ending_overlay_view(&self) -> EndingOverlayView {
        let narrative = narrative_text();
        let mut earned = narrative
            .epilogue_beats
            .iter()
            .filter(|beat| self.epilogue_beat_earned(beat))
            .collect::<Vec<_>>();
        earned.sort_by_key(|beat| std::cmp::Reverse(beat.order));

        let mut body = narrative.overlays.observatory_epilogue.clone();
        for beat in earned.into_iter().take(MAX_EPILOGUE_BEATS) {
            body.push_str("\n\n");
            body.push_str(&beat.line);
        }

        EndingOverlayView {
            title: ui_copy("overlay_ending_title").to_owned(),
            body,
            footer: narrative.overlays.observatory_footer.clone(),
        }
    }

    fn epilogue_beat_earned(&self, beat: &NarrativeEpilogueBeat) -> bool {
        beat.after_milestones
            .iter()
            .all(|milestone_id| self.has_journal_milestone(milestone_id))
    }
}

#[cfg(test)]
mod tests {
    use super::{GameplayState, MAX_EPILOGUE_BEATS};
    use crate::content::narrative_text;

    /// The ending panel is a fixed box with no scroll, and this number was
    /// calibrated against a capture rather than guessed: an epilogue of 1047
    /// characters rendered fourteen lines and ran the last of them through the
    /// footer. Body width is 852px at the design size, which turns out to be
    /// about 78 characters, and the box holds thirteen lines above the footer.
    /// Approximate on purpose — it will not catch one overlong word, but it does
    /// catch the epilogue quietly growing past its box.
    const EPILOGUE_CHAR_BUDGET: usize = 1000;

    #[test]
    fn the_fullest_possible_epilogue_still_fits_its_panel() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        for beat in &narrative_text().epilogue_beats {
            for milestone_id in &beat.after_milestones {
                state.push_journal_milestone(milestone_id, "", "");
            }
        }

        let body = state.ending_overlay_view().body;
        assert!(
            body.chars().count() <= EPILOGUE_CHAR_BUDGET,
            "a fully earned epilogue is {} characters, over the {EPILOGUE_CHAR_BUDGET} the panel \
             can show:\n{body}",
            body.chars().count()
        );
    }

    #[test]
    fn an_untouched_valley_gets_only_the_fixed_paragraph() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let state = GameplayState::new(&data);
        assert_eq!(
            state.ending_overlay_view().body,
            narrative_text().overlays.observatory_epilogue
        );
    }

    /// Earning more should never show less, and should never show more than the
    /// panel was measured for. Beats share milestones, so recording one beat's
    /// requirements can earn several — count what is genuinely earned rather
    /// than assuming one per step.
    #[test]
    fn the_epilogue_grows_with_the_work_and_stops_at_the_cap() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let beats = &narrative_text().epilogue_beats;
        let mut previous = state.ending_overlay_view().body.len();

        for beat in beats {
            for milestone_id in &beat.after_milestones {
                state.push_journal_milestone(milestone_id, "", "");
            }
            let earned = beats
                .iter()
                .filter(|candidate| {
                    candidate
                        .after_milestones
                        .iter()
                        .all(|id| state.has_journal_milestone(id))
                })
                .count();

            let body = state.ending_overlay_view().body;
            assert!(
                body.len() >= previous,
                "the epilogue lost a beat it had already earned"
            );
            previous = body.len();
            assert_eq!(
                body.matches("\n\n").count(),
                earned.min(MAX_EPILOGUE_BEATS),
                "showed the wrong number of the {earned} earned beats"
            );
        }

        assert!(
            beats.len() > MAX_EPILOGUE_BEATS,
            "author more epilogue beats than the cap, or the cap means nothing"
        );
    }
}
