#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Interact {
    pub player_id: u8,
    pub target_id: u32,
    pub x: f32,
    pub y: f32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Option<Vec<u32>>,
}
