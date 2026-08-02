use crate::data::{AreaDefinition, FlourishShape};
use macroquad::prelude::*;

/// Draw the flourishes an area has earned.
///
/// `earned` decides which; this only knows how to put shapes on the ground.
/// It used to be a `match` on area id with the coordinates inlined beside the
/// conditions, which is why the world acknowledged four story chains out of
/// twelve — every new one was a code change in two files.
pub(crate) fn draw_phase1_story_flourishes_view(
    area: &AreaDefinition,
    offset: Vec2,
    earned: &dyn Fn(&str) -> bool,
) {
    for flourish in &area.flourishes {
        if !earned(&flourish.id) {
            continue;
        }
        for shape in &flourish.shapes {
            draw_flourish_shape(shape, offset);
        }
    }
}

fn flourish_color(rgba: [u8; 4]) -> Color {
    Color::from_rgba(rgba[0], rgba[1], rgba[2], rgba[3])
}

fn draw_flourish_shape(shape: &FlourishShape, offset: Vec2) {
    match shape {
        FlourishShape::Rect { x, y, w, h, color } => {
            draw_rectangle(offset.x + x, offset.y + y, *w, *h, flourish_color(*color));
        }
        FlourishShape::Circle {
            x,
            y,
            radius,
            color,
            pulse,
        } => {
            let radius = if *pulse {
                // Offset by position so a row of lamps breathes out of step
                // rather than blinking in unison.
                let swell = ((get_time() as f32 * 2.2) + x * 0.01).sin() * 0.5 + 0.5;
                radius + swell * 2.5
            } else {
                *radius
            };
            draw_circle(offset.x + x, offset.y + y, radius, flourish_color(*color));
        }
        FlourishShape::Line {
            x,
            y,
            to_x,
            to_y,
            thickness,
            color,
        } => {
            draw_line(
                offset.x + x,
                offset.y + y,
                offset.x + to_x,
                offset.y + to_y,
                *thickness,
                flourish_color(*color),
            );
        }
    }
}
