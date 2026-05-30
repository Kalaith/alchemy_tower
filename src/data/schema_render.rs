use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BlockerVisualStyle {
    Shelf,
    House,
    #[default]
    Panel,
    Grass,
    Quarry,
    Forest,
    Reeds,
    Dunes,
    Rainforest,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct AreaRenderDefinition {
    #[serde(default)]
    pub(crate) blocker_style: BlockerVisualStyle,
    #[serde(default)]
    pub(crate) blocker_primary: Option<[u8; 4]>,
    #[serde(default)]
    pub(crate) blocker_secondary: Option<[u8; 4]>,
    #[serde(default)]
    pub(crate) blocker_detail: Option<[u8; 4]>,
    #[serde(default)]
    pub(crate) blocker_alt: Option<[u8; 4]>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct StationRenderDefinition {
    #[serde(default = "default_station_sprite_size")]
    pub(crate) sprite_size: [f32; 2],
    #[serde(default = "default_station_highlight_size_bonus")]
    pub(crate) highlight_size_bonus: [f32; 2],
    #[serde(default)]
    pub(crate) overlay_effect_id: String,
    #[serde(default = "default_zero_vec2")]
    pub(crate) overlay_effect_offset: [f32; 2],
    #[serde(default = "default_zero_vec2")]
    pub(crate) overlay_effect_size: [f32; 2],
}

impl Default for StationRenderDefinition {
    fn default() -> Self {
        Self {
            sprite_size: default_station_sprite_size(),
            highlight_size_bonus: default_station_highlight_size_bonus(),
            overlay_effect_id: String::new(),
            overlay_effect_offset: default_zero_vec2(),
            overlay_effect_size: default_zero_vec2(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GatherNodeRenderDefinition {
    #[serde(default = "default_gather_node_sprite_size")]
    pub(crate) sprite_size: [f32; 2],
    #[serde(default)]
    pub(crate) sprite_id: String,
}

impl Default for GatherNodeRenderDefinition {
    fn default() -> Self {
        Self {
            sprite_size: default_gather_node_sprite_size(),
            sprite_id: String::new(),
        }
    }
}

fn default_station_sprite_size() -> [f32; 2] {
    [96.0, 96.0]
}

fn default_station_highlight_size_bonus() -> [f32; 2] {
    [8.0, 8.0]
}

fn default_gather_node_sprite_size() -> [f32; 2] {
    [64.0, 64.0]
}

fn default_zero_vec2() -> [f32; 2] {
    [0.0, 0.0]
}
