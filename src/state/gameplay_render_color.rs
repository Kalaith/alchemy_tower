use macroquad::prelude::Color;

pub(super) fn render_color(values: [u8; 4]) -> Color {
    Color::from_rgba(values[0], values[1], values[2], values[3])
}
