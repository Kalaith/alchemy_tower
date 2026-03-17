//! Alchemy-specific helpers for recipe matching and brewing outcomes.

mod brewing;
mod fallback;
mod matching;
mod morphs;
mod quality;
mod traits;

pub use brewing::{resolve_brew, BrewResolution};
pub use matching::match_recipe;
pub use quality::{mastery_stage, quality_band};
