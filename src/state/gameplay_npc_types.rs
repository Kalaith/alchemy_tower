use macroquad::prelude::Vec2;

#[derive(Clone, Debug)]
pub(super) struct NpcRuntimeState {
    pub(super) area_id: String,
    pub(super) position: Vec2,
    pub(super) direction: Vec2,
    pub(super) moving: bool,
    pub(super) target_area_id: Option<String>,
}

#[derive(Clone, Debug)]
pub(super) struct TravelSegment {
    pub(super) area_id: String,
    pub(super) start: Vec2,
    pub(super) end: Vec2,
}

#[derive(Clone, Debug)]
pub(super) struct NpcMotionTracker {
    pub(super) area_id: String,
    pub(super) position: Vec2,
    pub(super) direction: Vec2,
    pub(super) moving: bool,
    pub(super) target_area_id: Option<String>,
    pub(super) schedule_index: Option<usize>,
    pub(super) target_schedule_index: Option<usize>,
    pub(super) route_segments: Vec<TravelSegment>,
    pub(super) route_segment_index: usize,
}
