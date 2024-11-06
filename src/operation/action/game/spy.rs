#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Spy {
    pub player_id: u8,
    // Skip 9 bytes
}
