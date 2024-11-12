#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrderType {
    Garrison,
    PackTrebuchet,
    UnpackTrebuchet,
    Other,
}

impl From<u32> for OrderType {
    fn from(value: u32) -> Self {
        match value {
            0x01 => Self::PackTrebuchet,
            0x02 => Self::UnpackTrebuchet,
            0x05 => Self::Garrison,
            _ => Self::Other,
        }
    }
}
