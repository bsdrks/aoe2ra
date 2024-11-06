#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QuickBuild {
    pub player_id: u8,
    // Skip 1 byte
    pub status: bool,
    // Skip 8 bytes
}
