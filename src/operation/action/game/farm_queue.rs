#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FarmQueue {
    pub player_id: u8,
    // Skip 1 byte
    pub amount: u8,
    // Skip 8 bytes
}
