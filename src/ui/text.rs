use macroquad::prelude::*;

pub fn wrapped_lines(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();

    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if measure_text(&candidate, None, font_size as u16, 1.0).width <= max_width {
            line = candidate;
        } else {
            if !line.is_empty() {
                lines.push(line);
            }
            line = word.to_owned();
        }
    }

    if !line.is_empty() {
        lines.push(line);
    }

    if lines.is_empty() && !text.is_empty() {
        lines.push(text.to_owned());
    }

    lines
}

pub fn truncate_text_to_width(text: &str, max_width: f32, font_size: f32) -> String {
    if measure_text(text, None, font_size as u16, 1.0).width <= max_width {
        return text.to_owned();
    }

    let ellipsis = "...";
    let mut result = String::new();
    for ch in text.chars() {
        let candidate = format!("{result}{ch}{ellipsis}");
        if measure_text(&candidate, None, font_size as u16, 1.0).width > max_width {
            break;
        }
        result.push(ch);
    }

    if result.is_empty() {
        ellipsis.to_owned()
    } else {
        format!("{result}{ellipsis}")
    }
}

pub fn draw_wrapped_text(
    text: &str,
    x: f32,
    y: f32,
    max_width: f32,
    font_size: f32,
    line_height: f32,
    color: Color,
) {
    let mut line_y = y;
    for line in wrapped_lines(text, max_width, font_size) {
        draw_text(&line, x, line_y, font_size, color);
        line_y += line_height;
    }
}
