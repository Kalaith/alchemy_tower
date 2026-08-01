//! Shared row-windowing for overlay lists.
//!
//! Several overlays draw fixed-height section boxes with no scrolling. Every one
//! of them was written when its list held two or three entries and quietly
//! overran its box once the content grew — the rune drafts list did, and the
//! quest board did the moment the board carried more than a handful of requests.

/// First row of the window that keeps `selected` visible. Clamping to the first
/// `window` entries instead would silently make every later entry unreachable,
/// which is worse than the overflow it fixes.
pub(super) fn visible_window_start(selected: usize, total: usize, window: usize) -> usize {
    if total <= window || window == 0 {
        return 0;
    }
    selected.saturating_sub(window - 1).min(total - window)
}

#[cfg(test)]
mod tests {
    use super::visible_window_start;

    #[test]
    fn the_selection_is_always_inside_the_window() {
        for window in 1..6usize {
            for total in 1..40usize {
                for selected in 0..total {
                    let start = visible_window_start(selected, total, window);
                    assert!(
                        selected >= start && selected < start + window,
                        "selection {selected} of {total} fell outside a {window}-row window at {start}"
                    );
                    assert!(
                        start + window <= total.max(window),
                        "window ran past the list"
                    );
                }
            }
        }
    }

    #[test]
    fn a_short_list_never_scrolls() {
        assert_eq!(visible_window_start(0, 3, 5), 0);
        assert_eq!(visible_window_start(2, 3, 5), 0);
    }
}

/// Rows shown at once by the archive's section lists and the brew journal.
pub(super) const ARCHIVE_PAGE_ROWS: usize = 6;

/// Where a paged list starts, and the line telling the player what fraction of
/// it they are looking at.
///
/// Four of the archive's five lists used to take the first six rows while the
/// selection index ranged over the whole list — so a player could select, and
/// act on, an entry that was neither drawn nor highlighted. Only the experiments
/// list paged correctly. This is that list's arithmetic, extracted so the other
/// four cannot drift from it again.
pub(super) fn paged_window(selected: usize, total: usize, rows: usize) -> (usize, Option<String>) {
    if total == 0 || rows == 0 {
        return (0, None);
    }
    let page = selected / rows;
    let start = page * rows;
    let page_count = total.div_ceil(rows);
    let text = (page_count > 1).then(|| {
        crate::content::ui_format(
            "overlay_page_of",
            &[
                ("page", &(page + 1).to_string()),
                ("count", &page_count.to_string()),
            ],
        )
    });
    (start, text)
}

#[cfg(test)]
mod paged_tests {
    use super::{paged_window, ARCHIVE_PAGE_ROWS};

    /// The failure this replaces: a selection past the first page was drawn on
    /// no page at all, so nothing appeared highlighted and the detail panel
    /// described a row the player could not see.
    #[test]
    fn every_selection_lands_on_the_page_that_is_drawn() {
        for total in 1..60usize {
            for selected in 0..total {
                let (start, _) = paged_window(selected, total, ARCHIVE_PAGE_ROWS);
                assert!(
                    selected >= start && selected < start + ARCHIVE_PAGE_ROWS,
                    "selection {selected} of {total} is not on the page starting at {start}"
                );
            }
        }
    }

    #[test]
    fn a_single_page_is_not_announced() {
        assert_eq!(
            paged_window(0, ARCHIVE_PAGE_ROWS, ARCHIVE_PAGE_ROWS).1,
            None
        );
        assert!(paged_window(0, ARCHIVE_PAGE_ROWS + 1, ARCHIVE_PAGE_ROWS)
            .1
            .is_some());
    }
}
