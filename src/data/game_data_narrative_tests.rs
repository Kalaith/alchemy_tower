//! Journal and town-voice checks: that the flat milestone id space has no
//! collisions, that every recorded moment is remarked on by somebody, and that
//! no authored line waits on a beat the game never records.
//!
//! Split out of `game_data_progression_tests.rs` at 692 lines. The playability
//! checks there ask whether the game can be finished; these ask whether it says
//! anything about it. `recordable_milestone_ids` lives here because it is a
//! statement about the journal id space rather than about progression.

#[cfg(test)]
pub(crate) mod tests {
    use crate::data::load_embedded;
    use crate::data::GameData;

    pub(crate) fn recordable_milestone_ids(data: &GameData) -> std::collections::HashSet<String> {
        use crate::content::narrative_text;

        let mut recordable = std::collections::HashSet::new();
        for milestone in narrative_text().milestones.all() {
            recordable.insert(milestone.id.clone());
        }
        for quest in &data.quests {
            recordable.extend(quest.completion_milestones.iter().map(|m| m.id.clone()));
        }
        // Recipes became a third writer into the journal when discovery beats
        // landed, and three near-identical copies of this set had grown up in
        // this file — only one of which had been taught about them. One set.
        for recipe in &data.recipes {
            recordable.extend(recipe.discovery_milestones.iter().map(|m| m.id.clone()));
        }
        for area in &data.areas {
            for warp in &area.warps {
                recordable.extend(warp.unlock_milestones.iter().map(|m| m.id.clone()));
            }
        }
        recordable
    }

    /// A town reaction is prose gated on a condition. A quest or milestone id
    /// that does not exist gives a line that is written, shipped, and never
    /// spoken — the most expensive kind of typo in a content file.
    /// The reverse of `town_reactions_are_gated_on_real_beats`: that check asks
    /// whether every line has a beat, this one asks whether every beat has a
    /// line. Five recorded moments had nobody in the valley remark on them,
    /// including the discovery that the wizard removed eleven months of records
    /// deliberately — the game's largest revelation, landing in silence.
    ///
    /// Also the split guard. `NarrativeText::reactions` is `#[serde(default)]`
    /// so it can be filled from `narrative_reactions.json` after parsing, which
    /// means a broken include or a renamed key would leave the valley mute and
    /// still deserialize cleanly. Every assertion below reads zero as failure.
    /// Journal milestones live in one flat id space, and `push_journal_milestone`
    /// silently does nothing if the id is already recorded. Three different
    /// kinds of content now write into it — quests on completion, recipes on
    /// discovery, and the fixed narrative beats — so a duplicated id does not
    /// error anywhere: the second beat simply never appears, and its title and
    /// text are quietly replaced by the first one's.
    #[test]
    fn no_two_journal_beats_share_an_id() {
        use crate::content::narrative_text;

        let data = load_embedded().expect("embedded game data should load");
        let mut seen = std::collections::HashMap::<String, String>::new();
        let mut clashes = Vec::new();

        let mut claim = |id: &str, owner: String, clashes: &mut Vec<String>| {
            if let Some(first) = seen.insert(id.to_owned(), owner.clone()) {
                clashes.push(format!("{id}: {first} and {owner}"));
            }
        };

        for milestone in narrative_text().milestones.all() {
            claim(&milestone.id, "narrative".to_owned(), &mut clashes);
        }
        for quest in &data.quests {
            for milestone in &quest.completion_milestones {
                claim(&milestone.id, format!("quest {}", quest.id), &mut clashes);
            }
        }
        for recipe in &data.recipes {
            for milestone in &recipe.discovery_milestones {
                claim(&milestone.id, format!("recipe {}", recipe.id), &mut clashes);
            }
        }
        for area in &data.areas {
            for warp in &area.warps {
                for milestone in &warp.unlock_milestones {
                    claim(&milestone.id, format!("warp {}", warp.id), &mut clashes);
                }
            }
        }
        clashes.sort();

        assert!(
            clashes.is_empty(),
            "journal beats sharing an id, so the later one never appears:
{clashes:#?}"
        );
    }

    #[test]
    fn every_recorded_moment_gets_remarked_on_by_somebody() {
        use crate::content::narrative_text;

        let data = load_embedded().expect("embedded game data should load");
        let narrative = narrative_text();

        assert!(
            !narrative.reactions.is_empty(),
            "no reactions loaded at all — the split reactions files are not reaching the game"
        );

        // Reactions are one file per speaker, so a table entry left out of
        // `REACTION_SOURCES` does not fail to compile — it just makes that
        // townsperson mute. Forgetting exactly this kind of registration has
        // broken four tests at once before; here it would break none.
        let mute = data
            .npcs
            .iter()
            .filter(|npc| {
                !narrative
                    .reactions
                    .iter()
                    .any(|reaction| reaction.npc_id == npc.id)
            })
            .map(|npc| npc.id.clone())
            .collect::<Vec<_>>();
        assert!(
            mute.is_empty(),
            "townsfolk with no reactions loaded — a speaker file is likely unregistered: {mute:?}"
        );

        let spoken_for = narrative
            .reactions
            .iter()
            .map(|reaction| reaction.after_milestone.clone())
            .collect::<std::collections::HashSet<_>>();

        let silent = data
            .quests
            .iter()
            .flat_map(|quest| quest.completion_milestones.iter())
            .map(|milestone| milestone.id.clone())
            .filter(|id| !spoken_for.contains(id))
            .collect::<std::collections::BTreeSet<_>>();

        assert!(
            silent.is_empty(),
            "moments the journal records that nobody in town says a word about: {silent:?}"
        );
    }

    #[test]
    fn town_reactions_are_gated_on_real_beats() {
        use crate::content::narrative_text;

        let data = load_embedded().expect("embedded game data should load");
        let narrative = narrative_text();

        let known_milestones = recordable_milestone_ids(&data);

        let mut unreachable = Vec::new();
        for reaction in &narrative.reactions {
            if data.npc(&reaction.npc_id).is_none() {
                unreachable.push(format!("{}: no such townsperson", reaction.npc_id));
            }
            if !reaction.after_quest.is_empty() && data.quest(&reaction.after_quest).is_none() {
                unreachable.push(format!(
                    "{} waits on quest {}, which does not exist",
                    reaction.npc_id, reaction.after_quest
                ));
            }
            if !reaction.after_milestone.is_empty()
                && !known_milestones.contains(&reaction.after_milestone)
            {
                unreachable.push(format!(
                    "{} waits on milestone {}, which nothing records",
                    reaction.npc_id, reaction.after_milestone
                ));
            }
        }

        // The epilogue is gated the same way, and a beat that can never be
        // earned is the one piece of prose nobody would ever notice missing.
        for beat in &narrative.epilogue_beats {
            for milestone_id in &beat.after_milestones {
                if !known_milestones.contains(milestone_id) {
                    unreachable.push(format!(
                        "epilogue beat waits on milestone {milestone_id}, which nothing records"
                    ));
                }
            }
        }

        assert!(
            unreachable.is_empty(),
            "narrative gated on beats that never happen:\n{unreachable:#?}"
        );
    }
}
