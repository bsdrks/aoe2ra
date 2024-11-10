#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Resign {
    pub player_id: u8,
    pub player_num: u8,
    pub disconnected: bool,
    pub unknown_u8_1: u8,
    pub unknown_u32_1: u32,
}
