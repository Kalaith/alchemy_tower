use macroquad::prelude::*;

pub(crate) fn draw_texture_cover(texture: &Texture2D, rect: Rect, tint: Color) {
    let texture_width = texture.width();
    let texture_height = texture.height();
    if texture_width <= 0.0 || texture_height <= 0.0 || rect.w <= 0.0 || rect.h <= 0.0 {
        return;
    }

    let texture_aspect = texture_width / texture_height;
    let rect_aspect = rect.w / rect.h;
    let source = if texture_aspect > rect_aspect {
        let width = texture_height * rect_aspect;
        Rect::new((texture_width - width) * 0.5, 0.0, width, texture_height)
    } else {
        let height = texture_width / rect_aspect;
        Rect::new(0.0, (texture_height - height) * 0.5, texture_width, height)
    };

    draw_texture_ex(
        texture,
        rect.x,
        rect.y,
        tint,
        DrawTextureParams {
            source: Some(source),
            dest_size: Some(vec2(rect.w, rect.h)),
            ..Default::default()
        },
    );
}

pub(crate) fn draw_texture_centered(texture: &Texture2D, center: Vec2, size: Vec2, tint: Color) {
    draw_texture_ex(
        texture,
        center.x - size.x * 0.5,
        center.y - size.y * 0.5,
        tint,
        DrawTextureParams {
            dest_size: Some(size),
            ..Default::default()
        },
    );
}

pub(crate) fn draw_character_frame(
    texture: &Texture2D,
    center: Vec2,
    facing: Vec2,
    moving: bool,
    alpha: f32,
) {
    let row = if facing.y > 0.5 {
        0.0
    } else if facing.x < -0.5 {
        1.0
    } else if facing.x > 0.5 {
        2.0
    } else {
        3.0
    };
    let column = if moving {
        1.0 + ((get_time() * 7.0) as i32).rem_euclid(4) as f32
    } else {
        0.0
    };
    draw_texture_ex(
        texture,
        center.x - 32.0,
        center.y - 32.0,
        Color::new(1.0, 1.0, 1.0, alpha),
        DrawTextureParams {
            source: Some(Rect::new(column * 64.0, row * 64.0, 64.0, 64.0)),
            dest_size: Some(vec2(64.0, 64.0)),
            ..Default::default()
        },
    );
}
