use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemCategory {
    Ingredient,
    Catalyst,
    Potion,
    Rune,
    Creature,
}

impl ItemCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ingredient => "ingredient",
            Self::Catalyst => "catalyst",
            Self::Potion => "potion",
            Self::Rune => "rune",
            Self::Creature => "creature",
        }
    }
}

impl fmt::Display for ItemCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectKind {
    Glow,
    Speed,
    Misfire,
    Restore,
}

impl EffectKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Glow => "glow",
            Self::Speed => "speed",
            Self::Misfire => "misfire",
            Self::Restore => "restore",
        }
    }
}

impl fmt::Display for EffectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StationKind {
    Alchemy,
    RestBed,
    Shop,
    RuneWorkshop,
    ArchiveConsole,
    EndingFocus,
    QuestBoard,
    Planter,
    Habitat,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerVisualStyle {
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
pub struct AreaRenderDefinition {
    #[serde(default)]
    pub blocker_style: BlockerVisualStyle,
    #[serde(default)]
    pub blocker_primary: Option<[u8; 4]>,
    #[serde(default)]
    pub blocker_secondary: Option<[u8; 4]>,
    #[serde(default)]
    pub blocker_detail: Option<[u8; 4]>,
    #[serde(default)]
    pub blocker_alt: Option<[u8; 4]>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationRenderDefinition {
    #[serde(default = "default_station_sprite_size")]
    pub sprite_size: [f32; 2],
    #[serde(default = "default_station_highlight_size_bonus")]
    pub highlight_size_bonus: [f32; 2],
    #[serde(default)]
    pub overlay_effect_id: String,
    #[serde(default = "default_zero_vec2")]
    pub overlay_effect_offset: [f32; 2],
    #[serde(default = "default_zero_vec2")]
    pub overlay_effect_size: [f32; 2],
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
pub struct GatherNodeRenderDefinition {
    #[serde(default = "default_gather_node_sprite_size")]
    pub sprite_size: [f32; 2],
    #[serde(default)]
    pub sprite_id: String,
}

impl Default for GatherNodeRenderDefinition {
    fn default() -> Self {
        Self {
            sprite_size: default_gather_node_sprite_size(),
            sprite_id: String::new(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ElementProfile {
    #[serde(default)]
    pub vita: i32,
    #[serde(default)]
    pub ember: i32,
    #[serde(default)]
    pub mist: i32,
    #[serde(default)]
    pub lux: i32,
}

impl ElementProfile {
    pub fn total(&self) -> i32 {
        self.vita + self.ember + self.mist + self.lux
    }

    pub fn add_assign(&mut self, other: &Self) {
        self.vita += other.vita;
        self.ember += other.ember;
        self.mist += other.mist;
        self.lux += other.lux;
    }

    pub fn meets(&self, required: &Self) -> bool {
        self.vita >= required.vita
            && self.ember >= required.ember
            && self.mist >= required.mist
            && self.lux >= required.lux
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
