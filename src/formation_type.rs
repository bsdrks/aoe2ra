#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FormationType {
    Box,
    Flank,
    Line,
    Staggered,
    Unknown,
}

impl From<u32> for FormationType {
    fn from(value: u32) -> Self {
        match value {
            0x02 => Self::Line,
            0x04 => Self::Box,
            0x06 => Self::Staggered,
            0x08 => Self::Flank,
            _ => Self::Unknown,
        }
    }
}
