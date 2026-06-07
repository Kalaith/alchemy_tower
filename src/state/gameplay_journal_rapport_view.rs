use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::journal::{JournalRapportRowView, JournalRapportTabView};

impl GameplayState {
    pub(super) fn journal_rapport_tab_view(&self, data: &GameData) -> JournalRapportTabView {
        JournalRapportTabView {
            title: ui_copy("overlay_town_rapport"),
            rows: data
                .npcs
                .iter()
                .map(|npc| {
                    let rapport = self
                        .progression
                        .relationships
                        .get(&npc.id)
                        .copied()
                        .unwrap_or_default();
                    let role = if npc.role.is_empty() {
                        ui_copy("overlay_rapport_empty")
                    } else {
                        npc.role.as_str()
                    };
                    JournalRapportRowView {
                        title: ui_format(
                            "overlay_rapport_line",
                            &[
                                ("name", &npc.name),
                                ("role", role),
                                ("value", &rapport.to_string()),
                            ],
                        ),
                        now_text: ui_format(
                            "overlay_now",
                            &[("text", &self.npc_now_hint(data, npc))],
                        ),
                        later_text: ui_format(
                            "overlay_later",
                            &[("text", &self.npc_later_hint(data, npc))],
                        ),
                        usually_text: ui_format(
                            "overlay_usually",
                            &[("text", &self.npc_usual_hint(data, npc))],
                        ),
                    }
                })
                .collect(),
        }
    }
}
