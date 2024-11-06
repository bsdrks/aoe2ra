#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AiMove {
    pub player_id: u8,
    pub player_num: u8,
    pub target_id: u32,
    pub x: f32,
    pub y: f32,
    pub unit_ids: Option<Vec<u32>>,
}
