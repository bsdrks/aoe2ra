#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Create {
    pub unit_type: u16,
    pub player_id: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
