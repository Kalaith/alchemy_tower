use super::gameplay_feedback_types::ActiveEffect;
use super::GameplayState;
use crate::data::{EffectDefinition, EffectKind};

impl GameplayState {
    pub(super) fn apply_effect(&mut self, effect: &EffectDefinition) {
        match effect.kind {
            EffectKind::Restore => self.vitality = (self.vitality + effect.magnitude).min(100.0),
            EffectKind::Speed | EffectKind::Glow | EffectKind::Misfire => {
                if let Some(existing) = self
                    .runtime
                    .active_effects
                    .iter_mut()
                    .find(|active| active.kind == effect.kind)
                {
                    existing.magnitude = existing.magnitude.max(effect.magnitude);
                    existing.remaining_seconds =
                        existing.remaining_seconds.max(effect.duration_seconds);
                    existing.description = effect.description.clone();
                } else {
                    self.runtime.active_effects.push(ActiveEffect {
                        kind: effect.kind,
                        magnitude: effect.magnitude,
                        remaining_seconds: effect.duration_seconds,
                        description: effect.description.clone(),
                    });
                }
            }
        }
    }

    pub(super) fn move_speed_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;
        for effect in &self.runtime.active_effects {
            if effect.kind == EffectKind::Speed {
                multiplier += effect.magnitude;
            } else if effect.kind == EffectKind::Misfire {
                multiplier -= 0.25 * effect.magnitude;
            }
        }
        multiplier.max(0.55)
    }

    pub(super) fn effect_active(&self, kind: EffectKind) -> bool {
        self.runtime
            .active_effects
            .iter()
            .any(|effect| effect.kind == kind)
    }
}
