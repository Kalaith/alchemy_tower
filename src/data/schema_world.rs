use super::{AreaRenderDefinition, GatherNodeRenderDefinition};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GameConfig {
    pub(crate) starting_area: String,
    pub(crate) starting_position: [f32; 2],
    pub(crate) move_speed: f32,
    pub(crate) interaction_range: f32,
    pub(crate) day_length_seconds: f32,
    pub(crate) save_version: u32,
    #[serde(default)]
    pub(crate) archive_required_completed_quests: Vec<String>,
    #[serde(default)]
    pub(crate) archive_required_journal_milestones: Vec<String>,
    /// Tuning that used to live as `const` in the Rust that read it. Balance
    /// belongs beside the content it balances — every other number the designer
    /// turns is already in `assets/`, and these were the last that were not.
    pub(crate) balance: BalanceDefinition,
}

/// `deny_unknown_fields` throughout: a tuning value nobody reads is worse than
/// a missing one, because the file says it is configured and the game ignores
/// it. Nothing here takes a serde default either — a balance block that half
/// loads should fail loudly rather than silently fall back to numbers no
/// designer can see.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BalanceDefinition {
    pub(crate) rapport: RapportTuning,
    pub(crate) salvage: SalvageTuning,
    pub(crate) quality_value_percent: QualityValueTuning,
    pub(crate) vitality: VitalityTuning,
    pub(crate) gathering: GatheringTuning,
}

/// What a day's work costs and what rest gives back.
///
/// Vitality shipped as a number that only ever went up: `Restore` — 51 of the
/// game's 110 authored effect blocks — had nothing to restore *from*. It is the
/// day's stamina now, so a restorative is what lets somebody keep working.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VitalityTuning {
    /// Spent standing over a cauldron.
    pub(crate) brew_cost: f32,
    /// Spent walking a route and bending down.
    pub(crate) gather_cost: f32,
    /// A night in a bed, by choice.
    pub(crate) sleep_restores: f32,
    /// Waking up somewhere you did not choose, having run out. Deliberately
    /// less than a proper night: collapsing should cost something beyond the
    /// morning it already eats.
    pub(crate) collapse_restores: f32,
}

/// Rules about working the ground.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GatheringTuning {
    /// Time windows too dark to pick anything in without a light. `Glow` used
    /// to tint the player sprite and nothing else; this is what it is for.
    /// A list rather than a flag so evening can be added without touching Rust.
    pub(crate) dark_time_windows: Vec<String>,
}

/// Standing thresholds. FRIEND is reachable by seeing one errand through;
/// accepting and finishing a three-beat arc is worth exactly KIN.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RapportTuning {
    pub(crate) friend: i32,
    pub(crate) confidant: i32,
    pub(crate) kin: i32,
}

/// The off-book brewing curve — see `state::gameplay_salvage_discovery`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SalvageTuning {
    /// What a first guess at a mixture can come to.
    pub(crate) blind_cap: u32,
    /// How far that cap lifts per attempt at the same mixture.
    pub(crate) cap_per_attempt: u32,
    /// Quality added per attempt, so practice shows below the cap too.
    pub(crate) bonus_per_attempt: u32,
    /// Where practice stops paying, so an off-book mixture never overtakes the
    /// written recipes.
    pub(crate) practice_cap: u32,
    /// Attempts before the player is credited with having found the formula.
    pub(crate) discovery_attempts: u32,
}

/// What a bottle of each grade is worth, as a percentage of base value.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct QualityValueTuning {
    pub(crate) crude: u32,
    pub(crate) serviceable: u32,
    pub(crate) fine: u32,
    pub(crate) excellent: u32,
    pub(crate) masterwork: u32,
}

impl QualityValueTuning {
    /// By `quality_band_rank`, which scores Crude 0 through Masterwork 4.
    pub(crate) fn for_rank(&self, rank: u8) -> u32 {
        match rank {
            0 => self.crude,
            1 => self.serviceable,
            2 => self.fine,
            3 => self.excellent,
            _ => self.masterwork,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RectDefinition {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl RectDefinition {
    pub(crate) fn contains_xy(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct JournalMilestoneEntry {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) text: String,
}

/// `deny_unknown_fields` because the Southern Pass shipped for months with a
/// `required_completed_quest` key that no field claimed: serde dropped it in
/// silence, the gate never locked, and the milestone behind it never fired.
/// A gate that is not read should refuse to load, not quietly open.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WarpDefinition {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) rect: RectDefinition,
    pub(crate) target_area: String,
    pub(crate) target_position: [f32; 2],
    #[serde(default)]
    pub(crate) required_total_brews: u32,
    #[serde(default)]
    pub(crate) required_coins: u32,
    #[serde(default)]
    pub(crate) required_item_id: String,
    #[serde(default)]
    pub(crate) required_item_amount: u32,
    #[serde(default)]
    pub(crate) required_journal_milestone: String,
    /// Recipe that must be brought to the "mastered" mastery stage before this
    /// warp opens. A skill gate rather than a grind gate — see
    /// `alchemy::MASTERED_BREW_COUNT`.
    #[serde(default)]
    pub(crate) required_mastered_recipe: String,
    #[serde(default)]
    pub(crate) required_journal_hint: String,
    /// A route the town only reopens once a story beat has landed — the same
    /// gate gather nodes and stations already use, so a finished chain changes
    /// where the player can walk and not just what the journal says.
    #[serde(default)]
    pub(crate) required_completed_quest: String,
    #[serde(default)]
    pub(crate) unlock_milestones: Vec<JournalMilestoneEntry>,
    #[serde(default)]
    pub(crate) locked_note: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GatherNodeDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) item_id: String,
    pub(crate) color: [u8; 4],
    pub(crate) position: [f32; 2],
    pub(crate) radius: f32,
    #[serde(default)]
    pub(crate) route_id: String,
    #[serde(default)]
    pub(crate) seasons: Vec<String>,
    #[serde(default)]
    pub(crate) weathers: Vec<String>,
    #[serde(default)]
    pub(crate) time_windows: Vec<String>,
    #[serde(default = "default_spawn_chance")]
    pub(crate) spawn_chance: u32,
    #[serde(default)]
    pub(crate) note: String,
    /// Ground that only starts growing once a quest has been finished, so a
    /// completed story beat leaves a visible change in the world instead of
    /// only a journal entry.
    #[serde(default)]
    pub(crate) required_completed_quest: String,
    #[serde(default)]
    pub(crate) render: GatherNodeRenderDefinition,
}

/// Something in the world a brew can be used *on*, rather than drunk or handed
/// over: a wilted plant, a frightened creature, a slumped root wall across a
/// path. The game is about applied alchemy and until these existed the only
/// thing a bottle could do to the world was change an NPC's mind about you.
///
/// Resolving one records journal milestones, which is deliberately the same
/// currency every other gate already reads — a warp, a station or a gather node
/// can wait on a target being treated without any new gating machinery.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ApplyTargetDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) position: [f32; 2],
    pub(crate) radius: f32,
    pub(crate) color: [u8; 4],
    /// The kind of brew this needs poured on it.
    pub(crate) required_effect_kind: String,
    /// And how good it has to be. Empty means any bottle that does the job.
    #[serde(default)]
    pub(crate) minimum_quality_band: String,
    /// What the player is told before treating it, and after.
    pub(crate) untreated_note: String,
    pub(crate) treated_note: String,
    #[serde(default)]
    pub(crate) completion_milestones: Vec<JournalMilestoneEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GatheringRouteDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) area_id: String,
    pub(crate) description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct QuestDefinition {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) required_item_id: String,
    pub(crate) required_amount: u32,
    pub(crate) reward_coins: u32,
    pub(crate) giver_npc_id: String,
    #[serde(default)]
    pub(crate) minimum_quality_band: String,
    #[serde(default)]
    pub(crate) required_trait: String,
    #[serde(default)]
    pub(crate) required_traits: Vec<String>,
    #[serde(default)]
    pub(crate) minimum_trait_matches: u32,
    #[serde(default)]
    pub(crate) required_effect_kind: String,
    #[serde(default)]
    pub(crate) required_effect_kinds: Vec<String>,
    #[serde(default)]
    pub(crate) minimum_effect_matches: u32,
    #[serde(default)]
    pub(crate) prerequisite_quests: Vec<String>,
    #[serde(default)]
    pub(crate) required_unlocked_warp: String,
    #[serde(default)]
    pub(crate) minimum_total_brews: u32,
    /// A recipe the player must have *mastered* — seven clean brews of the same
    /// formula — before this request will post. Brew count alone says how busy
    /// somebody has been; this says they can make one particular thing the same
    /// way twice, which is a different claim and the one an infirmary cares
    /// about.
    #[serde(default)]
    pub(crate) required_mastered_recipe: String,
    /// The townsperson whose work this request serves. Board orders are posted
    /// by the board rather than by a person, so without this the whole
    /// repeatable layer earned no standing with anybody — the prose named the
    /// infirmary or the lamplighters and the game did nothing with it.
    /// Delivering awards this person +1 rapport.
    #[serde(default)]
    pub(crate) rapport_npc_id: String,
    /// Standing this request waits on: the named townsperson must trust the
    /// player this far before it is posted at all. This is what the upper
    /// rapport tiers are *for*.
    #[serde(default)]
    pub(crate) required_rapport_npc_id: String,
    #[serde(default)]
    pub(crate) required_rapport: i32,
    #[serde(default)]
    pub(crate) completion_milestones: Vec<JournalMilestoneEntry>,
    /// What the giver says while this step is on offer, and once it is accepted.
    /// A quest chain needs its own voice per beat — without these every step of
    /// an arc reads as the same conversation.
    #[serde(default)]
    pub(crate) giver_intro_line: String,
    #[serde(default)]
    pub(crate) giver_active_line: String,
    /// Board requests only: when true, the request returns to the board after a
    /// cooldown instead of being permanently completed, giving the mid-game a
    /// recurring reason to brew and deliver.
    #[serde(default)]
    pub(crate) repeatable: bool,
    /// Days after a repeatable request is delivered before it is offered again
    /// (clamped to at least 1 day at delivery time).
    #[serde(default)]
    pub(crate) repeat_cooldown_days: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct AreaDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) size: [f32; 2],
    pub(crate) background: [u8; 4],
    pub(crate) accent: [u8; 4],
    #[serde(default = "default_footstep_sound_set")]
    pub(crate) footstep_sound_set: String,
    pub(crate) blockers: Vec<RectDefinition>,
    pub(crate) warps: Vec<WarpDefinition>,
    pub(crate) gather_nodes: Vec<GatherNodeDefinition>,
    #[serde(default)]
    pub(crate) apply_targets: Vec<ApplyTargetDefinition>,
    #[serde(default)]
    pub(crate) render: AreaRenderDefinition,
}

fn default_spawn_chance() -> u32 {
    100
}

fn default_footstep_sound_set() -> String {
    "dirt_path".to_owned()
}
