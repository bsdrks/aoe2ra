#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Move {
    pub player_id: u8,
    pub x: f32,
    pub y: f32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Vec<u32>,
}
