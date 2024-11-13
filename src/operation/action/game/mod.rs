pub mod allied_victory;
pub mod cheat;
pub mod default_stance;
pub mod diplomacy;
pub mod farm_auto_queue;
pub mod farm_queue;
pub mod farm_unqueue;
pub mod fish_trap_auto_queue;
pub mod fish_trap_queue;
pub mod fish_trap_unqueue;
pub mod game_action_mode;
pub mod instant_build;
pub mod quick_build;
pub mod speed;
pub mod spy;
pub mod unknown_0x03;

use crate::parser::{
    Parse,
    Parser,
};
pub use {
    allied_victory::AlliedVictory,
    cheat::Cheat,
    default_stance::DefaultStance,
    diplomacy::Diplomacy,
    farm_auto_queue::FarmAutoQueue,
    farm_queue::FarmQueue,
    farm_unqueue::FarmUnqueue,
    fish_trap_auto_queue::FishTrapAutoQueue,
    fish_trap_queue::FishTrapQueue,
    fish_trap_unqueue::FishTrapUnqueue,
    game_action_mode::GameActionMode,
    instant_build::InstantBuild,
    quick_build::QuickBuild,
    speed::Speed,
    spy::Spy,
    unknown_0x03::Unknown0x03,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Game {
    AlliedVictory(AlliedVictory),
    Cheat(Cheat),
    DefaultStance(DefaultStance),
    Diplomacy(Diplomacy),
    FarmAutoQueue(FarmAutoQueue),
    FarmQueue(FarmQueue),
    FarmUnqueue(FarmUnqueue),
    FishTrapAutoQueue(FishTrapAutoQueue),
    FishTrapQueue(FishTrapQueue),
    FishTrapUnqueue(FishTrapUnqueue),
    InstantBuild(InstantBuild),
    QuickBuild(QuickBuild),
    Speed(Speed),
    Spy(Spy),
    Unknown0x03(Unknown0x03),
}

// Examples:
// 01000000_14000000_67011000_10000000_01000000_00000000_00000000_1B000000
// 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_859C0200
// 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_7A0A0000
// 01000000_14000000_67011000_13000000_01000000_00000000_00000000_1B000000
impl Parse for Game {
    fn parse(parser: &mut Parser) -> Self {
        let mode = GameActionMode::parse(parser);

        match mode {
            GameActionMode::AlliedVictory => unimplemented!(),
            GameActionMode::Cheat => unimplemented!(),
            GameActionMode::DefaultStance => unimplemented!(),
            GameActionMode::Diplomacy => unimplemented!(),
            GameActionMode::FarmAutoQueue => unimplemented!(),
            GameActionMode::FarmQueue => unimplemented!(),
            GameActionMode::FarmUnqueue => unimplemented!(),
            GameActionMode::FishTrapAutoQueue => unimplemented!(),
            GameActionMode::FishTrapQueue => unimplemented!(),
            GameActionMode::FishTrapUnqueue => unimplemented!(),
            GameActionMode::InstantBuild => {
                Self::InstantBuild(InstantBuild::parse(parser))
            }
            GameActionMode::QuickBuild => unimplemented!(),
            GameActionMode::Speed => Self::Speed(Speed::parse(parser)),
            GameActionMode::Spy => unimplemented!(),
            GameActionMode::Unknown0x03 => {
                Self::Unknown0x03(Unknown0x03::parse(parser))
            }
        }
    }
}
