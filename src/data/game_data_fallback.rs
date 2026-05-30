use super::game_data::GameData;
use super::embedded_json::expect_labeled_json;

impl GameData {
    #[cfg(test)]
    pub(crate) fn fallback() -> Self {
        crate::data::load_embedded()
            .expect("embedded fallback game data must remain valid")
    }

    pub(crate) fn runtime_fallback() -> Self {
        let mut data: GameData = expect_labeled_json(
            "game_data_fallback",
            include_str!("../../assets/data/game_data_fallback.json"),
        );
        data.build_indexes()
            .expect("embedded runtime fallback indexes must remain valid");
        data
    }
}
