#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttackGround {
    pub x: f32,
    pub y: f32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Vec<u32>,
}
