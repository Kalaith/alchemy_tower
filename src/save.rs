//! Save/load helpers.

mod save_data;

pub(crate) use save_data::{
    exists, load, save, SAVE_ERROR_USER_DATA_DIR_MISSING, SAVE_ERROR_WASM_LOAD_UNAVAILABLE,
    SAVE_ERROR_WASM_SAVE_UNAVAILABLE,
};
