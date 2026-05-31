use crate::content::ui_format;

pub(super) struct QuestUnlockRequirements {
    pub(super) missing_prereqs: Vec<String>,
    pub(super) missing_warp: bool,
    pub(super) missing_total_brews: bool,
    pub(super) minimum_total_brews: u32,
}

pub(super) fn summary(requirements: QuestUnlockRequirements) -> String {
    let mut reasons = Vec::new();
    if !requirements.missing_prereqs.is_empty() {
        reasons.push(ui_format(
            "quests_unlock_finish",
            &[("quests", &requirements.missing_prereqs.join(", "))],
        ));
    }
    if requirements.missing_warp {
        reasons.push(ui_format("quests_unlock_greenhouse", &[]));
    }
    if requirements.missing_total_brews {
        reasons.push(ui_format(
            "quests_unlock_brews",
            &[("brews", &requirements.minimum_total_brews.to_string())],
        ));
    }
    if reasons.is_empty() {
        ui_format("quests_unlock_closed", &[])
    } else {
        ui_format(
            "quests_unlock_after",
            &[("reasons", &reasons.join(" and "))],
        )
    }
}
