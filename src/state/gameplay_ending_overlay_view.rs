use super::GameplayState;
use crate::content::{narrative_text, ui_copy, NarrativeEpilogueBeat};
use crate::view_models::ending::EndingOverlayView;

/// The panel holds roughly fourteen lines at the design size and the fixed
/// paragraph already uses five, so the opening page has room for three beats.
pub(crate) const MAX_EPILOGUE_BEATS: usize = 3;

/// Later pages carry no fixed paragraph, so they have room for one more beat
/// than the opener. Five overran the box by 134 characters — the budget test
/// below is what says so.
const LATER_PAGE_BEATS: usize = 4;

impl GameplayState {
    /// The epilogue beats this run earned, heaviest first.
    ///
    /// All of them. The panel used to show the top three and stop, and since
    /// reaching the ending at all earns two of the highest beats outright, only
    /// one slot was ever really contested — nine of the twelve were invisible
    /// even to a player who had done everything. The box cannot grow, so the
    /// ending is read a few beats at a time instead.
    fn earned_epilogue_beats(&self) -> Vec<&'static NarrativeEpilogueBeat> {
        let mut earned = narrative_text()
            .epilogue_beats
            .iter()
            .filter(|beat| self.epilogue_beat_earned(beat))
            .collect::<Vec<_>>();
        earned.sort_by_key(|beat| std::cmp::Reverse(beat.order));
        earned
    }

    /// Beats shown on `page`, and how many pages there are in total.
    fn epilogue_page(&self, page: usize) -> (Vec<&'static NarrativeEpilogueBeat>, usize) {
        let earned = self.earned_epilogue_beats();
        let after_first = earned.len().saturating_sub(MAX_EPILOGUE_BEATS);
        let pages = 1 + after_first.div_ceil(LATER_PAGE_BEATS);
        let page = page.min(pages - 1);

        let shown = if page == 0 {
            earned.into_iter().take(MAX_EPILOGUE_BEATS).collect()
        } else {
            earned
                .into_iter()
                .skip(MAX_EPILOGUE_BEATS + (page - 1) * LATER_PAGE_BEATS)
                .take(LATER_PAGE_BEATS)
                .collect()
        };
        (shown, pages)
    }

    /// How many pages the epilogue runs to, so the input handler knows when
    /// confirming should turn a page and when it should close the game out.
    pub(super) fn epilogue_page_count(&self) -> usize {
        self.epilogue_page(0).1
    }

    pub(super) fn ending_overlay_view(&self) -> EndingOverlayView {
        let narrative = narrative_text();
        let page = self.ui.ending_page;
        let (shown, pages) = self.epilogue_page(page);

        // The opening paragraph sets the scene once; turning the page continues
        // the list rather than starting it again.
        let mut body = if page == 0 {
            narrative.overlays.observatory_epilogue.clone()
        } else {
            String::new()
        };
        for beat in shown {
            if !body.is_empty() {
                body.push_str("\n\n");
            }
            body.push_str(&beat.line);
        }

        let footer = if page + 1 < pages {
            ui_copy("overlay_ending_more").to_owned()
        } else {
            narrative.overlays.observatory_footer.clone()
        };

        EndingOverlayView {
            title: ui_copy("overlay_ending_title").to_owned(),
            body,
            footer,
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

    /// Earn everything the game can give.
    fn a_completionist_run(state: &mut GameplayState) {
        for beat in &narrative_text().epilogue_beats {
            for milestone_id in &beat.after_milestones {
                state.push_journal_milestone(milestone_id, "", "");
            }
        }
    }

    #[test]
    fn the_fullest_possible_epilogue_still_fits_its_panel() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        a_completionist_run(&mut state);

        // Every page, not just the first — a later page carries more beats
        // because it has no fixed paragraph above them, which is exactly where
        // the box would overrun if the prose grew.
        for page in 0..state.epilogue_page_count() {
            state.ui.ending_page = page;
            let body = state.ending_overlay_view().body;
            assert!(
                body.chars().count() <= EPILOGUE_CHAR_BUDGET,
                "epilogue page {page} is {} characters, over the {EPILOGUE_CHAR_BUDGET} the panel \
                 can show:\n{body}",
                body.chars().count()
            );
        }
    }

    /// The ending showed the three highest-order beats and stopped. Reaching it
    /// at all earns two of those outright, so one slot was ever really
    /// contested and nine of the twelve were invisible to a player who had done
    /// everything — the game's last words about their run, withheld.
    #[test]
    fn a_completionist_hears_every_beat_they_earned() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        a_completionist_run(&mut state);

        let mut read = String::new();
        for page in 0..state.epilogue_page_count() {
            state.ui.ending_page = page;
            read.push_str(&state.ending_overlay_view().body);
            read.push('\n');
        }

        let missing = narrative_text()
            .epilogue_beats
            .iter()
            .filter(|beat| !read.contains(beat.line.as_str()))
            .map(|beat| beat.order)
            .collect::<Vec<_>>();
        assert!(
            missing.is_empty(),
            "beats earned but never shown, by order: {missing:?}"
        );
    }

    /// Turning the last page closes the overlay rather than sticking on it.
    #[test]
    fn the_epilogue_ends_after_its_last_page() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        a_completionist_run(&mut state);
        let pages = state.epilogue_page_count();
        assert!(pages > 1, "a full run should take more than one page");

        // Asking for a page past the end shows the last one rather than an
        // empty panel.
        state.ui.ending_page = pages + 5;
        assert!(!state.ending_overlay_view().body.is_empty());
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
