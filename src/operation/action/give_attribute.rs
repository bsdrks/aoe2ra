#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GiveAttribute {
    pub player_id: u8,
    pub target_id: u8,
    pub attribute: u8,
    pub amount: f32,
}
