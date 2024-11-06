#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Save {
    pub exited: bool,
    pub player_id: u8,
    pub file_name: String,
    pub checksum: u32,
}
