//! Platform-selected save/load API.

#[cfg(not(target_arch = "wasm32"))]
#[path = "save_native.rs"]
mod platform;
#[cfg(target_arch = "wasm32")]
#[path = "save_wasm.rs"]
mod platform;
#[path = "save_codec.rs"]
mod save_codec;
#[path = "save_errors.rs"]
mod save_errors;
#[cfg(not(target_arch = "wasm32"))]
#[path = "save_native_path.rs"]
mod save_native_path;

pub(crate) use self::platform::{exists, load, save};
pub(crate) use self::save_errors::SAVE_ERROR_USER_DATA_DIR_MISSING;
