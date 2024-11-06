#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Flare {
    pub player_ids: Vec<u8>,
    pub x: f32,
    pub y: f32,
    pub player_id: u8,
    pub player_num: u8,
}
