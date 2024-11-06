#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AiQueue {
    pub building_id: u32,
    pub player_id: u8,
    pub unit_type: u16,
}
