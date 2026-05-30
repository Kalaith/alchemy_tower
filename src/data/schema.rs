//! Serializable game definitions shared across systems.

#[path = "schema_core.rs"]
mod schema_core;
#[path = "schema_alchemy.rs"]
mod schema_alchemy;
#[path = "schema_items.rs"]
mod schema_items;
#[path = "schema_progression.rs"]
mod schema_progression;
#[path = "schema_world.rs"]
mod schema_world;

pub use self::schema_alchemy::*;
pub use self::schema_core::*;
pub use self::schema_items::*;
pub use self::schema_progression::*;
pub use self::schema_world::*;
