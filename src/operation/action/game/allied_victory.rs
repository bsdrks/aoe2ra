#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AlliedVictory {
    pub player_id: u8,
    // Skip 1 byte
    pub other_player_id: u8,
    pub status: bool,
    // Skip 7 bytes
}
