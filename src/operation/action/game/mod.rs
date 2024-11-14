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
pub mod game_0x03;
pub mod game_0x08;
pub mod game_action_mode;
pub mod instant_build;
pub mod quick_build;
pub mod speed;
pub mod spy;

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
    game_0x03::Game0x03,
    game_0x08::Game0x08,
    game_action_mode::GameActionMode,
    instant_build::InstantBuild,
    quick_build::QuickBuild,
    speed::Speed,
    spy::Spy,
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
    Game0x03(Game0x03),
    Game0x08(Game0x08),
}

// Examples:
// 01000000_14000000_67011000_10000000_01000000_00000000_00000000_1B000000
// 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_859C0200
// 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_7A0A0000
// 01000000_14000000_67011000_13000000_01000000_00000000_00000000_1B000000
// 01000000_14000000_67031000_10000000_03000000_00000000_00000000_A0010000
// 01000000_14000000_67081000_10000000_08000000_00000000_00000000_A0010000
impl Parse for Game {
    fn parse(parser: &mut Parser) -> Self {
        let mode = GameActionMode::parse(parser);

        match mode {
            GameActionMode::AlliedVictory => {
                Self::AlliedVictory(AlliedVictory::parse(parser))
            }
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
            GameActionMode::QuickBuild => {
                Self::QuickBuild(QuickBuild::parse(parser))
            }
            GameActionMode::Speed => Self::Speed(Speed::parse(parser)),
            GameActionMode::Spy => unimplemented!(),
            GameActionMode::Game0x08 => {
                Self::Game0x08(Game0x08::parse(parser))
            }
            GameActionMode::Game0x03 => {
                Self::Game0x03(Game0x03::parse(parser))
            }
        }
    }
}
