#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Wall {
    pub player_id: u8,
    pub start_x: u16,
    pub start_y: u16,
    pub end_x: u16,
    pub end_y: u16,
    pub building_id: u32,
    pub flags: Vec<u8>,
    pub unit_ids: Vec<u32>,
}
