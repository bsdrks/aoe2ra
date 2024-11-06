#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Queue {
    pub unit_type: u16,
    pub num_buildings: u8,
    pub queue_amount: u8,
    pub building_ids: Vec<u32>,
}
