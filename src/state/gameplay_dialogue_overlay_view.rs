use super::GameplayState;
use crate::content::ui_format;
use crate::data::GameData;
use crate::view_models::dialogue::DialogueOverlayView;

impl GameplayState {
    pub(super) fn dialogue_overlay_view(&self, data: &GameData) -> Option<DialogueOverlayView> {
        let npc_id = self.dialogue_npc_id()?;
        let npc = data.npc(npc_id)?;

        Some(DialogueOverlayView {
            title: npc.name.clone(),
            now_text: ui_format("overlay_now", &[("text", &self.npc_now_hint(data, npc))]),
            later_text: ui_format(
                "overlay_later",
                &[("text", &self.npc_later_hint(data, npc))],
            ),
            usually_text: ui_format(
                "overlay_usually",
                &[("text", &self.npc_usual_hint(data, npc))],
            ),
            body: self.current_dialogue_text(data, npc),
            footer: self.current_dialogue_footer(data, npc),
        })
    }
}
