use std::sync::atomic::{AtomicBool, Ordering};

/// How much of the HUD is drawn.
///
/// The tower's frames are the most finished art in the game and the world they
/// surround is procedurally generated, which is the wrong way round: the ornate
/// panelling reads as the subject. Quiet mode is the answer that does not
/// require redrawing anything — keep what a player needs to act on and drop the
/// framing, so the valley is what fills the screen.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HudDensity {
    Full,
    Quiet,
}

/// One drawable region of the HUD.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum HudPanel {
    /// Ornament. The tower's name, in a frame, permanently.
    TitleBanner,
    /// Vitality can end the working day, so it survives quiet mode.
    VitalityMedallion,
    CoinChip,
    /// The journal holds the same thing in more detail.
    GoalNote,
    /// The clock decides whether ground is gatherable and when you collapse.
    TimePanel,
    MinimapFrame,
    SideStatusPanel,
    ControlTags,
    /// Actionable: the bottles a keypress away.
    PotionBelt,
    /// What just happened, and the only warning before a collapse.
    StatusStrip,
    /// The banners that announce a beat, a delivery, a route reopening. They
    /// are the payoff channel, not framing, so quiet keeps them.
    EventToasts,
}

/// What each density draws. Quiet keeps the four things a player acts on and
/// drops the six that repeat the journal or frame the picture.
pub(crate) fn visible_panels(density: HudDensity) -> &'static [HudPanel] {
    match density {
        HudDensity::Full => &[
            HudPanel::TitleBanner,
            HudPanel::VitalityMedallion,
            HudPanel::CoinChip,
            HudPanel::GoalNote,
            HudPanel::TimePanel,
            HudPanel::MinimapFrame,
            HudPanel::SideStatusPanel,
            HudPanel::ControlTags,
            HudPanel::PotionBelt,
            HudPanel::StatusStrip,
            HudPanel::EventToasts,
        ],
        HudDensity::Quiet => &[
            HudPanel::VitalityMedallion,
            HudPanel::TimePanel,
            HudPanel::PotionBelt,
            HudPanel::StatusStrip,
            HudPanel::EventToasts,
        ],
    }
}

static QUIET_HUD: AtomicBool = AtomicBool::new(false);

pub(crate) fn hud_density() -> HudDensity {
    if QUIET_HUD.load(Ordering::Relaxed) {
        HudDensity::Quiet
    } else {
        HudDensity::Full
    }
}

pub(crate) fn set_quiet_hud(quiet: bool) {
    QUIET_HUD.store(quiet, Ordering::Relaxed);
}

pub(crate) fn quiet_hud_enabled() -> bool {
    QUIET_HUD.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::{visible_panels, HudDensity, HudPanel};

    /// Quiet mode is for seeing the world, so it has to actually remove
    /// something — and it has to keep the things that are not decoration.
    #[test]
    fn quiet_drops_the_framing_and_keeps_what_you_act_on() {
        let full = visible_panels(HudDensity::Full);
        let quiet = visible_panels(HudDensity::Quiet);

        assert!(
            quiet.len() < full.len(),
            "quiet mode draws as much as the full HUD, so it is not quiet"
        );
        for panel in quiet {
            assert!(
                full.contains(panel),
                "{panel:?} appears only in quiet mode, which cannot be right"
            );
        }

        // Vitality can end the day and the clock decides whether the ground is
        // even gatherable. Losing either to a display preference would make the
        // option a trap rather than a view.
        for required in [
            HudPanel::VitalityMedallion,
            HudPanel::TimePanel,
            HudPanel::StatusStrip,
            HudPanel::PotionBelt,
            HudPanel::EventToasts,
        ] {
            assert!(
                quiet.contains(&required),
                "quiet mode hides {required:?}, which the player needs to act"
            );
        }

        // And the framing is what should go.
        for ornament in [HudPanel::TitleBanner, HudPanel::MinimapFrame] {
            assert!(
                !quiet.contains(&ornament),
                "quiet mode still draws {ornament:?}"
            );
        }
    }

    /// The full HUD is the default: a display preference should never change
    /// what a new player sees before they have opted into anything.
    #[test]
    fn the_full_hud_is_what_you_get_without_asking() {
        super::set_quiet_hud(false);
        assert_eq!(super::hud_density(), HudDensity::Full);
        super::set_quiet_hud(true);
        assert_eq!(super::hud_density(), HudDensity::Quiet);
        super::set_quiet_hud(false);
    }
}
