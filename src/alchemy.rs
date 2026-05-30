//! Alchemy-specific helpers for recipe matching and brewing outcomes.

mod brewing;
mod fallback;
mod matching;
mod morphs;
mod quality;
mod traits;

pub(crate) use brewing::{resolve_brew, BrewResolution};
pub(crate) use matching::match_recipe;
pub(crate) use quality::{mastery_stage, quality_band};
