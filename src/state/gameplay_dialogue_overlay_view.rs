use super::GameplayState;
use crate::data::GameData;
use crate::view_models::dialogue::DialogueOverlayView;

impl GameplayState {
    pub(super) fn dialogue_overlay_view(&self, data: &GameData) -> Option<DialogueOverlayView> {
        let npc_id = self.dialogue_npc_id()?;
        let npc = data.npc(npc_id)?;

        Some(DialogueOverlayView {
            title: npc.name.clone(),
            body: self.current_dialogue_text(data, npc),
            footer: self.current_dialogue_footer(data, npc),
        })
    }
}
