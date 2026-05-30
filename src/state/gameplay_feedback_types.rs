use macroquad::prelude::{Color, Vec2};

use crate::data::EffectKind;

#[derive(Clone, Debug)]
pub(super) struct ActiveEffect {
    pub(super) kind: EffectKind,
    pub(super) magnitude: f32,
    pub(super) remaining_seconds: f32,
    pub(super) description: String,
}

#[derive(Clone, Debug)]
pub(super) struct GatherToast {
    pub(super) remaining_seconds: f32,
}

#[derive(Clone, Debug)]
pub(super) struct GatherFeedback {
    pub(super) position: Vec2,
    pub(super) remaining_seconds: f32,
    pub(super) color: Color,
    pub(super) emphasis: bool,
    pub(super) burst_scale: f32,
}

#[derive(Clone, Debug, Default)]
pub(super) struct FieldDiscoveryFeedback {
    pub(super) new_note: bool,
    pub(super) improved_quality: bool,
    pub(super) variant_discovered: bool,
}
