use crate::view_models::journal::JournalBrewsTabView;
use super::draw_wrapped_text;
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_journal_brews_tab_view(
    view: &JournalBrewsTabView,
    x: f32,
    y: f32,
    _w: f32,
    h: f32,
) {
        draw_text(
            view.title,
            x + 20.0,
            y + 136.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut brew_y = y + 168.0;
        if view.entries.is_empty() {
            draw_text(
                &view.empty_text,
                x + 20.0,
                brew_y,
                20.0,
                dark::TEXT_DIM,
            );
            return;
        }
        for entry in &view.entries {
            draw_text(
                &entry.title,
                x + 20.0,
                brew_y,
                20.0,
                dark::TEXT_BRIGHT,
            );
            brew_y += 20.0;
            draw_text(
                &entry.state_line,
                x + 20.0,
                brew_y,
                17.0,
                dark::TEXT_DIM,
            );
            brew_y += 18.0;
            draw_wrapped_text(
                &entry.recap,
                x + 20.0,
                brew_y,
                520.0,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            if let Some(effects_text) = &entry.effects_text {
                draw_text(
                    effects_text,
                    x + 580.0,
                    brew_y - 2.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                if let Some(traits_text) = &entry.traits_text {
                    draw_text(
                        traits_text,
                        x + 580.0,
                        brew_y + 20.0,
                        18.0,
                        dark::TEXT_DIM,
                    );
                }
            }
            brew_y += 40.0;
            if let Some(best_brew_text) = &entry.best_brew_text {
                draw_text(
                    best_brew_text,
                    x + 20.0,
                    brew_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                brew_y += 22.0;
            }
            if let Some(formula_text) = &entry.formula_text {
                draw_text(
                    formula_text,
                    x + 20.0,
                    brew_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                brew_y += 22.0;
            }
            if let Some(successful_brews_text) = &entry.successful_brews_text {
                draw_text(
                    successful_brews_text,
                    x + 20.0,
                    brew_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                brew_y += 22.0;
            }
            if brew_y > y + h - 40.0 {
                break;
            }
        }
}
