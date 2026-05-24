use macroquad::prelude::*;

pub fn draw_interaction_prompt(_position: Vec2, text: &str) {
    let (key, label) = split_prompt(text);
    let label_width = measure_text(label, None, 22, 1.0).width;
    let width = (label_width + 178.0).clamp(320.0, 470.0);
    let x = screen_width() - width - 24.0;
    let y = screen_height() - 78.0;
    let center_y = y + 28.0;
    let label_rect = Rect::new(x + 18.0, y + 9.0, width - 100.0, 40.0);
    let key_center = vec2(x + width - 42.0, center_y);

    draw_prompt_backplate(label_rect, key_center);
    draw_prompt_flourish(x, center_y, width);
    draw_beveled_rect(
        Rect::new(
            label_rect.x + 5.0,
            label_rect.y + 6.0,
            label_rect.w,
            label_rect.h,
        ),
        9.0,
        Color::from_rgba(0, 0, 0, 86),
    );
    draw_beveled_rect(label_rect, 9.0, Color::from_rgba(25, 24, 23, 184));
    draw_beveled_rect_lines(label_rect, 9.0, 1.6, Color::from_rgba(221, 177, 96, 178));
    draw_beveled_rect_lines(
        Rect::new(
            label_rect.x + 4.0,
            label_rect.y + 4.0,
            label_rect.w - 8.0,
            label_rect.h - 8.0,
        ),
        6.0,
        0.8,
        Color::from_rgba(249, 224, 158, 86),
    );
    draw_text(
        &truncate_to_width(label, width - 118.0, 22.0),
        label_rect.x + 19.0,
        y + 35.0,
        22.0,
        Color::from_rgba(246, 238, 213, 255),
    );

    if !key.is_empty() {
        let key_rect = Rect::new(key_center.x - 26.0, key_center.y - 26.0, 52.0, 52.0);
        draw_key_medallion(key_center);
        draw_centered_text(key, key_rect.x, key_rect.y + 32.0, key_rect.w, 18.0);
    }
}

fn draw_prompt_backplate(label_rect: Rect, key_center: Vec2) {
    let back = Rect::new(
        label_rect.x - 12.0,
        label_rect.y - 7.0,
        label_rect.w + 82.0,
        label_rect.h + 16.0,
    );
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        12.0,
        Color::from_rgba(0, 0, 0, 88),
    );
    draw_beveled_rect(back, 12.0, Color::from_rgba(101, 67, 34, 138));
    draw_beveled_rect_lines(back, 12.0, 1.0, Color::from_rgba(236, 195, 116, 128));
    for offset in [-1.0, 1.0] {
        draw_poly(
            key_center.x + offset * 35.0,
            key_center.y,
            4,
            4.0,
            45.0,
            Color::from_rgba(242, 205, 126, 158),
        );
    }
}

fn draw_prompt_flourish(x: f32, center_y: f32, width: f32) {
    let brass = Color::from_rgba(221, 177, 96, 176);
    draw_line(x - 58.0, center_y, x + 24.0, center_y, 2.0, brass);
    draw_line(
        x + width - 74.0,
        center_y,
        x + width - 14.0,
        center_y,
        1.6,
        Color::from_rgba(221, 177, 96, 146),
    );
    draw_circle_lines(
        x - 20.0,
        center_y,
        10.0,
        1.4,
        Color::from_rgba(221, 177, 96, 160),
    );
    draw_circle_lines(
        x + width - 86.0,
        center_y,
        8.0,
        1.2,
        Color::from_rgba(221, 177, 96, 132),
    );
    for point in [vec2(x + 3.0, center_y), vec2(x + width - 72.0, center_y)] {
        draw_poly(
            point.x,
            point.y,
            4,
            5.0,
            45.0,
            Color::from_rgba(242, 205, 126, 172),
        );
    }
}

fn draw_key_medallion(center: Vec2) {
    draw_poly(
        center.x + 4.0,
        center.y + 6.0,
        4,
        33.0,
        45.0,
        Color::from_rgba(0, 0, 0, 84),
    );
    draw_poly(
        center.x,
        center.y,
        4,
        33.0,
        45.0,
        Color::from_rgba(94, 70, 36, 186),
    );
    draw_poly(
        center.x,
        center.y,
        4,
        26.0,
        45.0,
        Color::from_rgba(39, 75, 110, 238),
    );
    draw_poly_lines(
        center.x,
        center.y,
        4,
        33.0,
        45.0,
        2.0,
        Color::from_rgba(242, 205, 126, 224),
    );
    draw_poly_lines(
        center.x,
        center.y,
        4,
        23.0,
        45.0,
        1.0,
        Color::from_rgba(185, 255, 244, 126),
    );
    draw_circle(
        center.x - 7.0,
        center.y - 8.0,
        4.0,
        Color::from_rgba(174, 247, 235, 82),
    );
}

fn split_prompt(text: &str) -> (&str, &str) {
    if let Some((raw_key, label)) = text.split_once(": ") {
        let key = raw_key.split('/').next().unwrap_or(raw_key).trim();
        (key, label.trim())
    } else {
        ("", text)
    }
}

fn draw_centered_text(text: &str, x: f32, baseline_y: f32, width: f32, font_size: f32) {
    let measured = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
        text,
        x + (width - measured.width) * 0.5,
        baseline_y,
        font_size,
        Color::from_rgba(246, 238, 213, 255),
    );
}

fn truncate_to_width(text: &str, max_width: f32, font_size: f32) -> String {
    macroquad_toolkit::ui::truncate_text_to_width(text, max_width, font_size)
}

fn draw_beveled_rect(rect: Rect, bevel: f32, color: Color) {
    let bevel = bevel.min(rect.w * 0.5).min(rect.h * 0.5);
    draw_rectangle(rect.x + bevel, rect.y, rect.w - bevel * 2.0, rect.h, color);
    draw_rectangle(rect.x, rect.y + bevel, rect.w, rect.h - bevel * 2.0, color);
    for (a, b, c) in [
        (
            vec2(rect.x + bevel, rect.y),
            vec2(rect.x, rect.y + bevel),
            vec2(rect.x + bevel, rect.y + bevel),
        ),
        (
            vec2(rect.x + rect.w - bevel, rect.y),
            vec2(rect.x + rect.w, rect.y + bevel),
            vec2(rect.x + rect.w - bevel, rect.y + bevel),
        ),
        (
            vec2(rect.x + rect.w, rect.y + rect.h - bevel),
            vec2(rect.x + rect.w - bevel, rect.y + rect.h),
            vec2(rect.x + rect.w - bevel, rect.y + rect.h - bevel),
        ),
        (
            vec2(rect.x, rect.y + rect.h - bevel),
            vec2(rect.x + bevel, rect.y + rect.h),
            vec2(rect.x + bevel, rect.y + rect.h - bevel),
        ),
    ] {
        draw_triangle(a, b, c, color);
    }
}

fn draw_beveled_rect_lines(rect: Rect, bevel: f32, thickness: f32, color: Color) {
    let bevel = bevel.min(rect.w * 0.5).min(rect.h * 0.5);
    let points = [
        vec2(rect.x + bevel, rect.y),
        vec2(rect.x + rect.w - bevel, rect.y),
        vec2(rect.x + rect.w, rect.y + bevel),
        vec2(rect.x + rect.w, rect.y + rect.h - bevel),
        vec2(rect.x + rect.w - bevel, rect.y + rect.h),
        vec2(rect.x + bevel, rect.y + rect.h),
        vec2(rect.x, rect.y + rect.h - bevel),
        vec2(rect.x, rect.y + bevel),
    ];
    for index in 0..points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        draw_line(start.x, start.y, end.x, end.y, thickness, color);
    }
}
