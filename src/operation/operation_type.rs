#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperationType {
    Action,
    Chat,
    Other,
    Sync,
    ViewLock,
}

impl From<u32> for OperationType {
    fn from(value: u32) -> Self {
        match value {
            0x01 => Self::Action,
            0x02 => Self::Sync,
            0x03 => Self::ViewLock,
            0x04 => Self::Chat,
            _ => Self::Other,
        }
    }
}
