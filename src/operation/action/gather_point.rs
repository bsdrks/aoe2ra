#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GatherPoint {

    pub target_id: Option<u32>,
    pub target_type: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub unit_ids: Vec<u32>,
}
