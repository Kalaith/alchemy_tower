use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, WarpDefinition};

impl GameplayState {
    pub(super) fn warp_is_unlocked(&self, warp: &WarpDefinition) -> bool {
        self.progression.unlocked_warps.contains(&warp.id)
            || (warp.required_total_brews == 0
                && warp.required_coins == 0
                && warp.required_item_id.is_empty()
                && warp.required_journal_milestone.is_empty())
    }

    pub(super) fn can_unlock_warp(&self, warp: &WarpDefinition) -> bool {
        self.progression.total_brews >= warp.required_total_brews
            && self.coins >= warp.required_coins
            && (warp.required_item_id.is_empty()
                || self
                    .inventory
                    .get(&warp.required_item_id)
                    .copied()
                    .unwrap_or_default()
                    >= warp.required_item_amount)
            && (warp.required_journal_milestone.is_empty()
                || self.has_journal_milestone(&warp.required_journal_milestone))
    }

    pub(super) fn pay_warp_costs(&mut self, warp: &WarpDefinition) {
        self.coins = self.coins.saturating_sub(warp.required_coins);
        if !warp.required_item_id.is_empty() {
            if let Some(amount) = self.inventory.get_mut(&warp.required_item_id) {
                *amount = amount.saturating_sub(warp.required_item_amount);
            }
            self.inventory.retain(|_, amount| *amount > 0);
        }
    }

    pub(super) fn locked_warps<'a>(&self, data: &'a GameData) -> Vec<&'a WarpDefinition> {
        data.areas
            .iter()
            .flat_map(|area| area.warps.iter())
            .filter(|warp| !self.warp_is_unlocked(warp))
            .collect()
    }

    pub(super) fn next_locked_warp<'a>(&self, data: &'a GameData) -> Option<&'a WarpDefinition> {
        self.locked_warps(data)
            .into_iter()
            .min_by_key(|warp| self.warp_progress_score(data, warp))
    }

    pub(super) fn warp_progress_score(&self, _data: &GameData, warp: &WarpDefinition) -> u32 {
        let owned = self
            .inventory
            .get(&warp.required_item_id)
            .copied()
            .unwrap_or_default();
        let item_missing = warp.required_item_amount.saturating_sub(owned);
        let milestone_missing = u32::from(
            !warp.required_journal_milestone.is_empty()
                && !self.has_journal_milestone(&warp.required_journal_milestone),
        );

        warp.required_total_brews
            .saturating_sub(self.progression.total_brews)
            .saturating_mul(100)
            .saturating_add(warp.required_coins.saturating_sub(self.coins))
            .saturating_add(item_missing.saturating_mul(25))
            .saturating_add(milestone_missing.saturating_mul(150))
    }

    pub(super) fn warp_requirement_summary(&self, data: &GameData, warp: &WarpDefinition) -> String {
        let mut parts = Vec::new();
        if self.progression.total_brews < warp.required_total_brews {
            parts.push(ui_format(
                "gameplay_requirement_more_brews",
                &[(
                    "count",
                    &warp
                        .required_total_brews
                        .saturating_sub(self.progression.total_brews)
                        .to_string(),
                )],
            ));
        }
        if self.coins < warp.required_coins {
            parts.push(ui_format(
                "gameplay_requirement_more_coins",
                &[(
                    "count",
                    &warp.required_coins.saturating_sub(self.coins).to_string(),
                )],
            ));
        }
        if !warp.required_item_id.is_empty() {
            let owned = self
                .inventory
                .get(&warp.required_item_id)
                .copied()
                .unwrap_or_default();
            if owned < warp.required_item_amount {
                parts.push(ui_format(
                    "gameplay_requirement_item_amount",
                    &[
                        ("item", data.item_name(&warp.required_item_id)),
                        (
                            "amount",
                            &warp.required_item_amount.saturating_sub(owned).to_string(),
                        ),
                    ],
                ));
            }
        }
        if !warp.required_journal_milestone.is_empty()
            && !self.has_journal_milestone(&warp.required_journal_milestone)
        {
            parts.push(if warp.required_journal_hint.is_empty() {
                ui_copy("gameplay_requirement_archive_entry").to_owned()
            } else {
                warp.required_journal_hint.clone()
            });
        }
        if parts.is_empty() {
            warp.locked_note.clone()
        } else {
            parts.join(", ")
        }
    }

    pub(super) fn warp_lock_text(&self, data: &GameData, warp: &WarpDefinition) -> String {
        let requirement_summary = self.warp_requirement_summary(data, warp);
        if requirement_summary == warp.locked_note {
            requirement_summary
        } else {
            ui_format(
                "gameplay_warp_lock_text",
                &[
                    ("label", &warp.label),
                    ("requirements", &requirement_summary),
                ],
            )
        }
    }
}
