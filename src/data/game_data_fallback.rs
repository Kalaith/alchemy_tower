#[cfg(debug_assertions)]
use super::embedded_json::expect_labeled_json;
use super::game_data::GameData;

impl GameData {
    #[cfg(test)]
    pub(crate) fn fallback() -> Self {
        super::loader::load_embedded().expect("embedded fallback game data must remain valid")
    }

    #[cfg(debug_assertions)]
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
