#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unknown0x25 {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: Option<u32>,
    pub unknown_u32_2: u32,
    pub unknown_u8_3: u8,
}
