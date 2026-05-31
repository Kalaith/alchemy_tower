pub(crate) const HOTBAR_SLOT_COUNT: usize = 8;
pub(crate) type HudColor = [f32; 4];
pub(crate) type HudPoint = [f32; 2];

pub(crate) struct HudPotionSlot {
    pub(crate) key_label: &'static str,
    pub(crate) icon_id: Option<String>,
    pub(crate) amount: u32,
}

pub(crate) struct HudGoal {
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) detail: String,
    pub(crate) action: String,
    pub(crate) icon_id: Option<String>,
    pub(crate) amount_text: String,
}

pub(crate) struct HudFeedbackView {
    pub(crate) position: HudPoint,
    pub(crate) radius: f32,
    pub(crate) color: HudColor,
    pub(crate) sparkle_points: [HudPoint; 8],
    pub(crate) burst_scale: f32,
}

pub(crate) struct HudControlTag {
    pub(crate) key_label: String,
    pub(crate) label: String,
}

pub(crate) struct HudView {
    pub(crate) game_title: String,
    pub(crate) vitality_label: String,
    pub(crate) vitality_text: String,
    pub(crate) coins_label: String,
    pub(crate) coins_value: String,
    pub(crate) clock_text: String,
    pub(crate) season_weather_text: String,
    pub(crate) day_text: String,
    pub(crate) sleep_warning_text: Option<String>,
    pub(crate) goal_prefix: String,
    pub(crate) goal: HudGoal,
    pub(crate) status_text: String,
    pub(crate) area_label: String,
    pub(crate) inventory_label: String,
    pub(crate) effects_label: String,
    pub(crate) no_effects_label: String,
    pub(crate) journal_label: String,
    pub(crate) journal_key_label: String,
    pub(crate) minimap_north_label: String,
    pub(crate) control_tags: Vec<HudControlTag>,
    pub(crate) truncation_suffix: String,
    pub(crate) potions: [HudPotionSlot; HOTBAR_SLOT_COUNT],
    pub(crate) inventory_count: u32,
    pub(crate) effect_count: usize,
    pub(crate) feedbacks: Vec<HudFeedbackView>,
}
