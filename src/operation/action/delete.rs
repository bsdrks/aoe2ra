#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Delete {
    pub object_id: u32,
    pub player_id: u8,
}
