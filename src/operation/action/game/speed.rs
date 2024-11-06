#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Speed {
    pub player_id: u8,
    // Skip 5 bytes
    pub speed: f32,
    // Skip 1 bytes
}
