use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ItemCategory {
    Ingredient,
    Catalyst,
    Potion,
    Rune,
    Creature,
}

impl ItemCategory {
    pub(crate) fn as_str(self) -> &'static str {
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
pub(crate) enum EffectKind {
    Glow,
    Speed,
    Misfire,
    Restore,
}

impl EffectKind {
    pub(crate) fn as_str(self) -> &'static str {
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
pub(crate) enum StationKind {
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct ElementProfile {
    #[serde(default)]
    pub(crate) vita: i32,
    #[serde(default)]
    pub(crate) ember: i32,
    #[serde(default)]
    pub(crate) mist: i32,
    #[serde(default)]
    pub(crate) lux: i32,
}

impl ElementProfile {
    pub(crate) fn total(&self) -> i32 {
        self.vita + self.ember + self.mist + self.lux
    }

    pub(crate) fn add_assign(&mut self, other: &Self) {
        self.vita += other.vita;
        self.ember += other.ember;
        self.mist += other.mist;
        self.lux += other.lux;
    }

    pub(crate) fn meets(&self, required: &Self) -> bool {
        self.vita >= required.vita
            && self.ember >= required.ember
            && self.mist >= required.mist
            && self.lux >= required.lux
    }
}
