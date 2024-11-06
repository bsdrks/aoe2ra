#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StanceType {
    Aggressive,
    Defensive,
    StandGround,
    Passive,
    Unknown,
}

impl From<u8> for StanceType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Aggressive,
            0x01 => Self::Defensive,
            0x02 => Self::StandGround,
            0x03 => Self::Passive,
            _ => Self::Unknown,
        }
    }
}
