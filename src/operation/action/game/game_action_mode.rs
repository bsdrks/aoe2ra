#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GameActionMode {
    AlliedVictory,
    Cheat,
    DefaultStance,
    Diplomacy,
    FarmAutoQueue,
    FarmQueue,
    FarmUnqueue,
    FishTrapAutoQueue,
    FishTrapQueue,
    FishTrapUnqueue,
    InstantBuild,
    QuickBuild,
    Speed,
    Spy,
    Unknown0x03,
}

impl From<u8> for GameActionMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Diplomacy,
            0x01 => Self::Speed,
            0x02 => Self::InstantBuild,
            0x03 => Self::Unknown0x03,
            0x04 => Self::QuickBuild,
            0x05 => Self::AlliedVictory,
            0x06 => Self::Cheat,
            0x0a => Self::Spy,
            0x0d => Self::FarmQueue,
            0x0e => Self::FarmUnqueue,
            0x10 => Self::FarmAutoQueue,
            0x11 => Self::FishTrapQueue,
            0x12 => Self::FishTrapUnqueue,
            0x13 => Self::FishTrapAutoQueue,
            0x14 => Self::DefaultStance,
            _ => unreachable!("Invalid value for GameActionMode: {}", value),
        }
    }
}
