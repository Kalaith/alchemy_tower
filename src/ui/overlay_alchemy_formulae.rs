use super::{draw_overlay_section_box, draw_overlay_section_title, draw_wrapped_text};
use crate::alchemy_layout::{AL_BOX_BOTTOM_MARGIN, AL_FORM_BOX_Y, AL_FORM_TITLE_Y, AL_LW, AL_LX};
use crate::view_models::alchemy::AlchemyFormulaePanelView;
use macroquad::prelude::Color;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub(crate) fn draw_alchemy_formulae_panel_view(
    view: &AlchemyFormulaePanelView,
    x: f32,
    y: f32,
    h: f32,
) {
    let box_top = y + AL_FORM_BOX_Y;
    let box_bottom = y + h - AL_BOX_BOTTOM_MARGIN;
    draw_overlay_section_title(x + AL_LX, y + AL_FORM_TITLE_Y, view.title, None);
    draw_overlay_section_box(x + AL_LX - 2.0, box_top, AL_LW, box_bottom - box_top);

    let text_x = x + AL_LX + 6.0;
    let avail_w = AL_LW - 24.0;

    if view.rows.is_empty() {
        draw_wrapped_text(
            &view.empty_text,
            text_x,
            box_top + 24.0,
            avail_w,
            18.0,
            18.0,
            dark::TEXT_DIM,
        );
        return;
    }

    let mut ky = box_top + 24.0;
    let mut shown = 0usize;
    for row in &view.rows {
        // Reserve room for a title line plus up to two detail lines; stop before
        // a recipe would spill past the box instead of overrunning it.
        if ky + 62.0 > box_bottom {
            break;
        }
        draw_ui_text(&row.title, text_x, ky, 18.0, dark::TEXT_BRIGHT);
        ky += 22.0;
        for line in wrap_lines(&row.detail, avail_w, 15, 2) {
            draw_ui_text(&line, text_x, ky, 15.0, dark::TEXT_DIM);
            ky += 18.0;
        }
        ky += 12.0;
        shown += 1;
    }

    if shown < view.rows.len() {
        draw_ui_text(
            &format!("+{} more (browse to see)", view.rows.len() - shown),
            text_x,
            (box_bottom - 8.0).min(ky),
            15.0,
            Color::from_rgba(186, 174, 145, 220),
        );
    }
}

/// Word-wrap `text` to at most `max_lines` lines that each fit `max_w`, adding
/// an ellipsis if content is dropped.
fn wrap_lines(text: &str, max_w: f32, font: u16, max_lines: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let trial = if current.is_empty() {
            word.to_string()
        } else {
            format!("{current} {word}")
        };
        if measure_ui_text(&trial, None, font, 1.0).width <= max_w {
            current = trial;
        } else if current.is_empty() {
            current = word.to_string();
        } else {
            lines.push(std::mem::take(&mut current));
            current = word.to_string();
            if lines.len() == max_lines {
                break;
            }
        }
    }
    if lines.len() < max_lines && !current.is_empty() {
        lines.push(current);
    }
    if lines.len() >= max_lines && text_has_more(text, &lines) {
        if let Some(last) = lines.last_mut() {
            last.push('…');
        }
    }
    lines.truncate(max_lines);
    lines
}

fn text_has_more(text: &str, lines: &[String]) -> bool {
    let shown: usize = lines
        .iter()
        .map(|line| line.split_whitespace().count())
        .sum();
    text.split_whitespace().count() > shown
}
