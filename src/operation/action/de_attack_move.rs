#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeAttackMove {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub flags: Option<Vec<u8>>,
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}
