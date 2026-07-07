use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, WarpDefinition};

pub(super) struct WarpRequirementProgress {
    total_brews: u32,
    coins: u32,
    owned_required_item: u32,
    missing_journal_milestone: bool,
    mastered_recipe_brews: u32,
}

impl WarpRequirementProgress {
    pub(super) fn new(
        total_brews: u32,
        coins: u32,
        owned_required_item: u32,
        missing_journal_milestone: bool,
        mastered_recipe_brews: u32,
    ) -> Self {
        Self {
            total_brews,
            coins,
            owned_required_item,
            missing_journal_milestone,
            mastered_recipe_brews,
        }
    }
}

pub(super) fn warp_requirement_summary(
    data: &GameData,
    warp: &WarpDefinition,
    progress: WarpRequirementProgress,
) -> String {
    let mut parts = Vec::new();
    if progress.total_brews < warp.required_total_brews {
        parts.push(ui_format(
            "gameplay_requirement_more_brews",
            &[(
                "count",
                &warp
                    .required_total_brews
                    .saturating_sub(progress.total_brews)
                    .to_string(),
            )],
        ));
    }
    if progress.coins < warp.required_coins {
        parts.push(ui_format(
            "gameplay_requirement_more_coins",
            &[(
                "count",
                &warp
                    .required_coins
                    .saturating_sub(progress.coins)
                    .to_string(),
            )],
        ));
    }
    if !warp.required_item_id.is_empty() && progress.owned_required_item < warp.required_item_amount
    {
        parts.push(ui_format(
            "gameplay_requirement_item_amount",
            &[
                ("item", data.item_name(&warp.required_item_id)),
                (
                    "amount",
                    &warp
                        .required_item_amount
                        .saturating_sub(progress.owned_required_item)
                        .to_string(),
                ),
            ],
        ));
    }
    if !warp.required_mastered_recipe.is_empty()
        && progress.mastered_recipe_brews < crate::alchemy::MASTERED_BREW_COUNT
    {
        let recipe_name = data
            .recipes
            .iter()
            .find(|recipe| recipe.id == warp.required_mastered_recipe)
            .map(|recipe| recipe.name.as_str())
            .unwrap_or_default();
        parts.push(ui_format(
            "gameplay_requirement_master_recipe",
            &[
                ("recipe", recipe_name),
                ("have", &progress.mastered_recipe_brews.to_string()),
                ("need", &crate::alchemy::MASTERED_BREW_COUNT.to_string()),
            ],
        ));
    }
    if progress.missing_journal_milestone {
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

pub(super) fn warp_lock_text(warp: &WarpDefinition, requirement_summary: String) -> String {
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
