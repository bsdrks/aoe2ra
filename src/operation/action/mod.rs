pub mod action_type;
pub mod add_attribute;
pub mod ai_command;
pub mod ai_interact;
pub mod ai_move;
pub mod ai_queue;
pub mod ai_waypoint;
pub mod attack_ground;
pub mod back_to_work;
pub mod build;
pub mod buy;
pub mod chapter;
pub mod create;
pub mod de_attack_move;
pub mod de_auto_scout;
pub mod de_queue;
pub mod delete;
pub mod drop_relic;
pub mod flare;
pub mod follow;
pub mod formation;
pub mod game;
pub mod gather_point;
pub mod give_attribute;
pub mod guard;
pub mod interact;
pub mod r#move;
pub mod multi_queue;
pub mod order;
pub mod patrol;
// pub mod post_game;
pub mod queue;
pub mod release;
pub mod repair;
pub mod research;
pub mod resign;
pub mod save;
pub mod sell;
// pub mod spec;
pub mod stance;
pub mod stop;
pub mod toggle_gate;
pub mod town_bell;
pub mod tribute;
pub mod wall;
pub mod waypoint;

pub use action_type::ActionType;

pub use {
    add_attribute::AddAttribute,
    ai_command::AiCommand,
    ai_interact::AiInteract,
    ai_move::AiMove,
    ai_queue::AiQueue,
    ai_waypoint::AiWaypoint,
    attack_ground::AttackGround,
    back_to_work::BackToWork,
    build::Build,
    buy::Buy,
    chapter::Chapter,
    create::Create,
    de_attack_move::DeAttackMove,
    de_auto_scout::DeAutoScout,
    de_queue::DeQueue,
    delete::Delete,
    drop_relic::DropRelic,
    flare::Flare,
    follow::Follow,
    formation::Formation,
    game::Game,
    gather_point::GatherPoint,
    give_attribute::GiveAttribute,
    guard::Guard,
    interact::Interact,
    multi_queue::MultiQueue,
    order::Order,
    patrol::Patrol,
};
// pub use post_game::PostGame;
pub use {
    queue::Queue,
    r#move::Move,
    release::Release,
    repair::Repair,
    research::Research,
    resign::Resign,
    save::Save,
    sell::Sell,
};
// pub use spec::Spec;
pub use {
    stance::Stance,
    stop::Stop,
    toggle_gate::ToggleGate,
    town_bell::TownBell,
    tribute::Tribute,
    wall::Wall,
    waypoint::Waypoint,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Action {
    AddAttribute(AddAttribute),
    AiCommand(AiCommand),
    AiInteract(AiInteract),
    AiMove(AiMove),
    AiQueue(AiQueue),
    AiWaypoint(AiWaypoint),
    AttackGround(AttackGround),
    BackToWork(BackToWork),
    Build(Build),
    Buy(Buy),
    Chapter(Chapter),
    Create(Create),
    DeAttackMove(DeAttackMove),
    DeAutoScout(DeAutoScout),
    DeQueue(DeQueue),
    Delete(Delete),
    DropRelic(DropRelic),
    Flare(Flare),
    Follow(Follow),
    Formation(Formation),
    Game(Game),
    GatherPoint(GatherPoint),
    GiveAttribute(GiveAttribute),
    Guard(Guard),
    Interact(Interact),
    Move(Move),
    MultiQueue(MultiQueue),
    Order(Order),
    Patrol(Patrol),
    // PostGame(PostGame),
    Queue(Queue),
    Release(Release),
    Repair(Repair),
    Research(Research),
    Resign(Resign),
    Save(Save),
    Sell(Sell),
    // Spec(Spec),
    Stance(Stance),
    Stop(Stop),
    ToggleGate(ToggleGate),
    TownBell(TownBell),
    Tribute(Tribute),
    Wall(Wall),
    Waypoint(Waypoint),
}