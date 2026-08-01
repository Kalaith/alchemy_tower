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
