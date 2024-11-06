#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cheat {
    pub player_id: u8,
    // Skip 1 bytes
    pub cheat_id: u8,
    // Skip 8 bytes
}
