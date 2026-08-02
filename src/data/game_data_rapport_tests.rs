//! Checks over the relationship layer: that every request knows whose work it
//! serves, and that a request waiting on standing can actually have that
//! standing earned.
//!
//! Split out of `game_data_progression_tests.rs`, which was at 740 lines when
//! board orders started paying rapport.

#[cfg(test)]
mod tests {
    use crate::data::load_embedded;
    use crate::data::GameData;

    /// Rapport a townsperson can be given without already having rapport with
    /// them. Arc beats pay +1 to accept and +2 to finish; an ungated board
    /// order pays +1 and is repeatable, so one of those is an unbounded supply.
    fn ungated_rapport_available(data: &GameData, npc_id: &str) -> Option<i32> {
        let mut total = 0;
        for quest in &data.quests {
            if quest.giver_npc_id == npc_id && quest.giver_npc_id != "quest_board" {
                total += 3;
            }
            if quest.rapport_npc_id != npc_id || !quest.required_rapport_npc_id.is_empty() {
                continue;
            }
            if quest.repeatable {
                // Repeatable and ungated: this alone reaches any threshold.
                return None;
            }
            total += 1;
        }
        Some(total)
    }

    /// A board order is the only part of the game the player can run forever,
    /// and for a long time it was the only part that earned no standing with
    /// anybody — the prose named the infirmary or the lamplighters and nothing
    /// downstream knew. Every order names its beneficiary now, and this keeps
    /// the next one from being posted without one.
    #[test]
    fn every_board_order_knows_whose_work_it_serves() {
        let data = load_embedded().expect("embedded game data should load");
        let mut orders = 0usize;
        let mut anonymous = Vec::new();

        for quest in &data.quests {
            if quest.giver_npc_id != "quest_board" {
                continue;
            }
            orders += 1;
            if quest.rapport_npc_id.is_empty() {
                anonymous.push(format!("{} serves nobody", quest.id));
            } else if data.npc(&quest.rapport_npc_id).is_none() {
                anonymous.push(format!(
                    "{} serves {}, who does not exist",
                    quest.id, quest.rapport_npc_id
                ));
            } else if quest.rapport_npc_id == "quest_board" {
                anonymous.push(format!("{} credits the board itself", quest.id));
            }
        }

        assert!(orders > 0, "there are no board orders at all");
        assert!(
            anonymous.is_empty(),
            "board orders that earn standing with nobody:\n{anonymous:#?}"
        );
    }

    /// The upper rapport tiers exist to gate requests a townsperson would not
    /// mention to a stranger. That only works if the standing can be reached
    /// without already having it — an order whose own rapport source is behind
    /// the same gate would never post, and nothing else would say so.
    #[test]
    fn a_standing_gate_can_have_its_standing_earned() {
        let data = load_embedded().expect("embedded game data should load");
        let mut gates = 0usize;
        let mut unreachable = Vec::new();

        for quest in &data.quests {
            if quest.required_rapport_npc_id.is_empty() {
                continue;
            }
            gates += 1;
            if data.npc(&quest.required_rapport_npc_id).is_none() {
                unreachable.push(format!(
                    "{} waits on {}, who does not exist",
                    quest.id, quest.required_rapport_npc_id
                ));
                continue;
            }
            if let Some(available) =
                ungated_rapport_available(&data, &quest.required_rapport_npc_id)
            {
                if available < quest.required_rapport {
                    unreachable.push(format!(
                        "{} wants {} rapport with {}, who can only be given {}",
                        quest.id, quest.required_rapport, quest.required_rapport_npc_id, available
                    ));
                }
            }
        }

        assert!(gates > 0, "nothing in the game waits on standing");
        assert!(
            unreachable.is_empty(),
            "standing gates that can never open:\n{unreachable:#?}"
        );
    }

    /// Standing should be worth having for everybody it can be built with. A
    /// townsperson the player can supply indefinitely but who never asks for
    /// anything in return is the old complaint in a smaller shape.
    #[test]
    fn the_upper_tiers_are_worth_reaching() {
        let data = load_embedded().expect("embedded game data should load");
        let gated = data
            .quests
            .iter()
            .filter(|quest| !quest.required_rapport_npc_id.is_empty())
            .count();
        assert!(
            gated >= 2,
            "only {gated} request(s) wait on standing; the upper tiers are labels again"
        );
    }
}
