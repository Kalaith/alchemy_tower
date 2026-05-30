use super::{GameplayState, ARCHIVE_TABS, SLOT_COUNT};
use crate::art::{draw_texture_centered, ArtAssets};
use crate::content::{input_bindings, ui_copy, ui_copy_optional, ui_format, ui_text};
use crate::data::GameData;
use crate::ui::{
    draw_action_button, draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle,
    draw_selection_card, draw_state_banner, draw_wrapped_text,
};

fn draw_overlay_section_title(x: f32, y: f32, title: &str, meta: Option<&str>) {
    draw_text(title, x, y, 24.0, dark::TEXT_BRIGHT);
    if let Some(meta) = meta {
        draw_text(meta, x + 208.0, y, 18.0, dark::TEXT_DIM);
    }
}

fn draw_overlay_section_box(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::from_rgba(16, 18, 26, 148));
    draw_rectangle_lines(x, y, w, h, 1.0, Color::from_rgba(160, 170, 190, 40));
}

fn draw_overlay_tab(rect: Rect, label: &str, selected: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            Color::from_rgba(30, 34, 44, 220)
        } else {
            Color::from_rgba(16, 18, 26, 150)
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        if selected {
            Color::from_rgba(255, 238, 196, 92)
        } else {
            Color::from_rgba(160, 170, 190, 56)
        },
    );
    let measured = measure_text(label, None, 18, 1.0);
    draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + 21.0,
        18.0,
        if selected {
            Color::from_rgba(248, 242, 230, 255)
        } else {
            dark::TEXT_DIM
        },
    );
}

fn archive_tab_label(tab: &str) -> &'static str {
    ui_copy(match tab {
        "timeline" => "overlay_archive_tab_timeline",
        "experiments" => "overlay_archive_tab_experiments",
        "mastery" => "overlay_archive_tab_mastery",
        "morphs" => "overlay_archive_tab_morphs",
        "disassembly" => "overlay_archive_tab_disassembly",
        _ => "overlay_archive_tab_duplication",
    })
}

#[path = "overlay_journal.rs"]
mod overlay_journal;
#[path = "overlay_journal_brews.rs"]
mod overlay_journal_brews;
#[path = "overlay_journal_routes.rs"]
mod overlay_journal_routes;
#[path = "overlay_alchemy.rs"]
mod overlay_alchemy;
#[path = "overlay_alchemy_sections.rs"]
mod overlay_alchemy_sections;
#[path = "overlay_alchemy_preview.rs"]
mod overlay_alchemy_preview;
#[path = "overlay_quest_board.rs"]
mod overlay_quest_board;
#[path = "overlay_archive.rs"]
mod overlay_archive;
#[path = "overlay_archive_experiments.rs"]
mod overlay_archive_experiments;
#[path = "overlay_archive_morphs.rs"]
mod overlay_archive_morphs;
#[path = "overlay_dialogue.rs"]
mod overlay_dialogue;
#[path = "overlay_ending.rs"]
mod overlay_ending;
#[path = "overlay_rune.rs"]
mod overlay_rune;
#[path = "overlay_shop.rs"]
mod overlay_shop;
