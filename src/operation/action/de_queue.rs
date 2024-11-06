#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeQueue {
    pub player_id: u8,
    pub building_type: u16,

    pub unit_type: u16,
    pub queue_amount: u8,
    pub building_ids: Vec<u32>,
}
