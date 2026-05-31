use crate::content::ui_copy;

pub(super) fn quality_band_rank(band: &str) -> u8 {
    match band {
        value if value == ui_copy("quality_band_crude") => 0,
        value if value == ui_copy("quality_band_serviceable") => 1,
        value if value == ui_copy("quality_band_fine") => 2,
        value if value == ui_copy("quality_band_excellent") => 3,
        value if value == ui_copy("quality_band_masterwork") => 4,
        _ => 0,
    }
}

pub(super) fn planter_stage_label(growth_days: u32, total_days: u32) -> &'static str {
    if growth_days == 0 {
        ui_copy("planter_stage_seeded")
    } else if growth_days >= total_days {
        ui_copy("planter_stage_ripe")
    } else if growth_days * 2 >= total_days {
        ui_copy("planter_stage_budding")
    } else {
        ui_copy("planter_stage_sprouting")
    }
}
