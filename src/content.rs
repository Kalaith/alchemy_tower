mod embedded_json;
mod input_bindings;
mod narrative_text;
mod ui_text;

pub(crate) use input_bindings::input_bindings;
pub(crate) use narrative_text::narrative_text;
pub(crate) use ui_text::{ui_copy, ui_copy_optional, ui_format, ui_text};
