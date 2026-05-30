use macroquad::prelude::*;

pub(super) fn fill_slate() -> Color {
    Color::from_rgba(34, 34, 45, 228)
}

pub(super) fn slot_glow(index: usize) -> Color {
    match index % 5 {
        0 => Color::from_rgba(223, 77, 70, 255),
        1 => Color::from_rgba(75, 158, 232, 255),
        2 => Color::from_rgba(112, 203, 115, 255),
        3 => Color::from_rgba(232, 184, 76, 255),
        _ => Color::from_rgba(180, 92, 224, 255),
    }
}

pub(super) fn bright_ink() -> Color {
    Color::from_rgba(246, 238, 213, 255)
}

pub(super) fn muted_ink() -> Color {
    Color::from_rgba(186, 174, 145, 255)
}

pub(super) fn parchment() -> Color {
    Color::from_rgba(226, 204, 162, 255)
}

pub(super) fn brass() -> Color {
    Color::from_rgba(189, 140, 69, 255)
}

pub(super) fn brass_light() -> Color {
    Color::from_rgba(242, 205, 126, 255)
}

pub(super) fn shadow() -> Color {
    Color::from_rgba(0, 0, 0, 108)
}
