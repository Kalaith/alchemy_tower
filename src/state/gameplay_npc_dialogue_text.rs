use crate::content::ui_format;

pub(super) fn with_followup(base: &str, extra: &str) -> String {
    ui_format("npc_story_with_followup", &[("base", base), ("extra", extra)])
}
