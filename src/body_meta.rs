#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BodyMeta {
    pub log_version: Option<u32>,
    pub checksum_interval: u32,
    pub multiplayer: bool,
    pub rec_owner: u32,
    pub reveal_map: u32,
    pub use_sequence_numbers: bool,
    pub number_of_chapters: u32,
    pub aok: bool,
    // skipping misc. AoK fields
}
