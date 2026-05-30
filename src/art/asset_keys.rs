pub(super) const BACKGROUND_CATEGORY: &str = "background";
pub(super) const CHARACTER_CATEGORY: &str = "character";
pub(super) const EFFECT_CATEGORY: &str = "effect";
pub(super) const ITEM_ICON_CATEGORY: &str = "item_icon";
pub(super) const JOURNAL_TAB_CATEGORY: &str = "journal_tab";
pub(super) const STATION_CATEGORY: &str = "station";
pub(super) const TITLE_SCREEN_CATEGORY: &str = "title_screen";
pub(super) const WORLD_NODE_CATEGORY: &str = "world_node";

pub(super) const PLAYER_ID: &str = "player_tower_alchemist";
pub(super) const GENERATED_ASSET_PACK: &str = "assets/generated.zip";

pub(super) fn asset_key(category: &str, id: &str) -> String {
    format!("{category}:{id}")
}
