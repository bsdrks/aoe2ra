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
