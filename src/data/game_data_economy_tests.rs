//! The coin economy: whether money is ever a decision.
//!
//! Split out of `game_data_progression_tests.rs`, which was back over 800 lines
//! when commissions landed. Those checks ask whether the game can be finished;
//! these ask whether its currency means anything by the time it is.

#[cfg(test)]
mod tests {
    use crate::data::load_embedded;

    /// Coins stopped being a decision once the content grew: quests pay out
    /// thousands, the four floor gates cost 250 between them, and for a long
    /// while the dearest thing any counter sold was 38. This is a floor on
    /// ambition, not a balance model — it fails if nothing in the game is worth
    /// deliberately saving for.
    #[test]
    fn there_is_something_worth_saving_for() {
        const WORTH_SAVING_FOR: u32 = 150;

        let data = load_embedded().expect("embedded game data should load");
        let dearest = data
            .stations
            .iter()
            .flat_map(|station| station.stock.iter())
            .map(|stocked| stocked.price)
            .max()
            .unwrap_or(0);
        let one_off_income = data
            .quests
            .iter()
            .filter(|quest| !quest.repeatable)
            .map(|quest| quest.reward_coins)
            .sum::<u32>();

        assert!(
            dearest >= WORTH_SAVING_FOR,
            "the dearest purchasable item costs {dearest} against {one_off_income} coins of              one-off quest income; nothing in the shops is worth saving for"
        );
    }

    /// The measured problem: a finished campaign earns roughly five thousand
    /// coins in one-off quest income alone, plus repeatable board cycles and
    /// unbounded sales, against a few hundred coins of shop stock and 250 in
    /// one-time warp tolls. Coins stopped being a decision somewhere in the
    /// middle of act two and never became one again.
    ///
    /// Commissions are the answer: requests the player *funds* rather than is
    /// paid for. This asserts they exist and are priced against the problem
    /// rather than against a shop shelf.
    #[test]
    fn there_is_somewhere_for_a_finished_campaign_to_put_its_money() {
        let data = load_embedded().expect("embedded game data should load");

        let one_off_income = data
            .quests
            .iter()
            .filter(|quest| !quest.repeatable)
            .map(|quest| quest.reward_coins)
            .sum::<u32>();
        let commissions = data
            .quests
            .iter()
            .filter(|quest| quest.coin_cost > 0)
            .collect::<Vec<_>>();
        let sink = commissions.iter().map(|quest| quest.coin_cost).sum::<u32>();

        assert!(
            commissions.len() >= 3,
            "only {} commission(s); the last third has nothing to spend on",
            commissions.len()
        );
        assert!(
            sink * 2 >= one_off_income,
            "commissions absorb {sink} against {one_off_income} of one-off income alone,              which leaves coins meaningless in the endgame"
        );
        // A commission that pays out is a quest wearing a costume.
        let paid = commissions
            .iter()
            .filter(|quest| quest.reward_coins > 0)
            .map(|quest| quest.id.clone())
            .collect::<Vec<_>>();
        assert!(
            paid.is_empty(),
            "commissions that also pay the player: {paid:?}"
        );
    }

    /// Commissions escalate. Three unrelated things to buy is a shop; a chain
    /// where each one is dearer and needs better work than the last is what the
    /// end of a game is for.
    #[test]
    fn commissions_get_dearer_as_they_go() {
        let data = load_embedded().expect("embedded game data should load");
        let mut chained = 0usize;

        for quest in data.quests.iter().filter(|quest| quest.coin_cost > 0) {
            for prerequisite in &quest.prerequisite_quests {
                let Some(earlier) = data.quest(prerequisite) else {
                    continue;
                };
                if earlier.coin_cost == 0 {
                    continue;
                }
                chained += 1;
                assert!(
                    quest.coin_cost > earlier.coin_cost,
                    "{} costs {} and follows {}, which costs {}",
                    quest.id,
                    quest.coin_cost,
                    earlier.id,
                    earlier.coin_cost
                );
            }
        }

        assert!(
            chained > 0,
            "no commission follows another; nothing escalates"
        );
    }
}
