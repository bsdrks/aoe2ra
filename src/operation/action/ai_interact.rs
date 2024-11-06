#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AiInteract {
    pub target_id: u32,
    pub x: f32,
    pub y: f32,
    pub unit_ids: Option<Vec<u32>>,
}
