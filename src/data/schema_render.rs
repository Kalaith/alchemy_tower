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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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

/// A bit of world that appears once the town has changed.
///
/// These were four `match` arms on area id in the renderer, with their
/// conditions in one Rust file and their coordinates in another, covering two
/// areas between them. Twelve story chains finish in this game and the world
/// acknowledged four of them. Data-driven, a reopened stall or a lit street is
/// an entry in an area file rather than a pull request.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlourishDefinition {
    pub(crate) id: String,
    /// Shown once *any* of these is finished. Both lists empty means always.
    /// A list rather than a single id because the first one authored already
    /// needed an "or": the square blooms for the greenhouse milestone or for
    /// Brin's cultivation request, whichever the player reaches first.
    #[serde(default)]
    pub(crate) after_any_completed_quest: Vec<String>,
    #[serde(default)]
    pub(crate) after_any_journal_milestone: Vec<String>,
    pub(crate) shapes: Vec<FlourishShape>,
}

/// The small vocabulary the four hand-written flourishes actually used, and
/// enough for the ones the TODO asks for next.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum FlourishShape {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [u8; 4],
    },
    Circle {
        x: f32,
        y: f32,
        radius: f32,
        color: [u8; 4],
        /// A slow breathing swell, for lamplight and anything else alive.
        #[serde(default)]
        pulse: bool,
    },
    Line {
        x: f32,
        y: f32,
        to_x: f32,
        to_y: f32,
        thickness: f32,
        color: [u8; 4],
    },
}
