pub mod action_0x23;
pub mod action_0x25;
pub mod action_0x29;
pub mod action_0x2c;
pub mod action_0x2d;
pub mod action_0x82;
pub mod action_0x83;
pub mod action_0x8c;
pub mod action_0xc4;
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
pub mod queue;
pub mod release;
pub mod repair;
pub mod research;
pub mod resign;
pub mod save;
pub mod sell;
pub mod spec;
pub mod stance;
pub mod stop;
pub mod toggle_gate;
pub mod town_bell;
pub mod tribute;
pub mod wall;
pub mod waypoint;

use crate::parser::{
    Parse,
    Parser,
};
pub use {
    action_0x23::Action0x23,
    action_0x25::Action0x25,
    action_0x29::Action0x29,
    action_0x2c::Action0x2c,
    action_0x2d::Action0x2d,
    action_0x82::Action0x82,
    action_0x83::Action0x83,
    action_0x8c::Action0x8c,
    action_0xc4::Action0xc4,
    action_type::ActionType,
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
    queue::Queue,
    r#move::Move,
    release::Release,
    repair::Repair,
    research::Research,
    resign::Resign,
    save::Save,
    sell::Sell,
    spec::Spec,
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
    Spec(Spec),
    Stance(Stance),
    Stop(Stop),
    ToggleGate(ToggleGate),
    TownBell(TownBell),
    Tribute(Tribute),
    Action0x23(Action0x23),
    Action0x25(Action0x25),
    Action0x29(Action0x29),
    Action0x2c(Action0x2c),
    Action0x2d(Action0x2d),
    Action0x82(Action0x82),
    Action0x83(Action0x83),
    Action0x8c(Action0x8c),
    Action0xc4(Action0xc4),
    Wall(Wall),
    Waypoint(Waypoint),
}

impl Parse for Action {
    fn parse(parser: &mut Parser) -> Self {
        println!("{}", parser.cursor);
        let length = parser.usize32();
        let cursor_next = parser.cursor + length;
        let action_type = ActionType::parse(parser);

        let action = match action_type {
            ActionType::AddAttribute => AddAttribute::parse(parser).into(),
            ActionType::AiCommand => unimplemented!(),
            ActionType::AiInteract => AiInteract::parse(parser).into(),
            ActionType::AiMove => AiMove::parse(parser).into(),
            ActionType::AiQueue => AiQueue::parse(parser).into(),
            ActionType::AiWaypoint => unimplemented!(),
            ActionType::AttackGround => AttackGround::parse(parser).into(),
            ActionType::BackToWork => BackToWork::parse(parser).into(),
            ActionType::Build => Build::parse(parser).into(),
            ActionType::Buy => Buy::parse(parser).into(),
            ActionType::Chapter => unimplemented!(),
            ActionType::Create => unimplemented!(),
            ActionType::DeAttackMove => DeAttackMove::parse(parser).into(),
            ActionType::DeAutoScout => DeAutoScout::parse(parser).into(),
            ActionType::DeQueue => DeQueue::parse(parser).into(),
            ActionType::Delete => Delete::parse(parser).into(),
            ActionType::DropRelic => unimplemented!(),
            ActionType::Flare => Flare::parse(parser).into(),
            ActionType::Follow => Follow::parse(parser).into(),
            ActionType::Formation => Formation::parse(parser).into(),
            ActionType::Game => Game::parse(parser).into(),
            ActionType::GatherPoint => GatherPoint::parse(parser).into(),
            ActionType::GiveAttribute => unimplemented!(),
            ActionType::Guard => Guard::parse(parser).into(),
            ActionType::Interact => Interact::parse(parser).into(),
            ActionType::Move => Move::parse(parser).into(),
            ActionType::MultiQueue => unimplemented!(),
            ActionType::Order => Order::parse(parser).into(),
            ActionType::Patrol => Patrol::parse(parser).into(),
            ActionType::PostGame => unimplemented!(),
            ActionType::Queue => unimplemented!(),
            ActionType::Release => Release::parse(parser).into(),
            ActionType::Repair => Repair::parse(parser).into(),
            ActionType::Research => Research::parse(parser).into(),
            ActionType::Resign => Resign::parse(parser).into(),
            ActionType::Save => unimplemented!(),
            ActionType::Sell => Sell::parse(parser).into(),
            ActionType::Spec => unimplemented!(),
            ActionType::Stance => Stance::parse(parser).into(),
            ActionType::Stop => Stop::parse(parser).into(),
            ActionType::ToggleGate => ToggleGate::parse(parser).into(),
            ActionType::TownBell => unimplemented!(),
            ActionType::Tribute => unimplemented!(),
            ActionType::Action0x23 => Action0x23::parse(parser).into(),
            ActionType::Action0x25 => Action0x25::parse(parser).into(),
            ActionType::Action0x29 => Action0x29::parse(parser).into(),
            ActionType::Action0x2c => Action0x2c::parse(parser).into(),
            ActionType::Action0x2d => Action0x2d::parse(parser).into(),
            ActionType::Action0x82 => Action0x82::parse(parser).into(),
            ActionType::Action0x83 => Action0x83::parse(parser).into(),
            ActionType::Action0x8c => Action0x8c::parse(parser).into(),
            ActionType::Action0xc4 => Action0xc4::parse(parser).into(),
            ActionType::Wall => Wall::parse(parser).into(),
            ActionType::Waypoint => Waypoint::parse(parser).into(),
        };

        assert!(
            parser.cursor != cursor_next,
            "expected cursor ({}) to be {cursor_next}, length: {length}, \
             after action {action_type:?}",
            parser.cursor,
        );

        action
    }
}

impl From<AddAttribute> for Action {
    fn from(value: AddAttribute) -> Self {
        Self::AddAttribute(value)
    }
}

impl From<AiInteract> for Action {
    fn from(value: AiInteract) -> Self {
        Self::AiInteract(value)
    }
}

impl From<AiMove> for Action {
    fn from(value: AiMove) -> Self {
        Self::AiMove(value)
    }
}

impl From<AiQueue> for Action {
    fn from(value: AiQueue) -> Self {
        Self::AiQueue(value)
    }
}

impl From<AttackGround> for Action {
    fn from(value: AttackGround) -> Self {
        Self::AttackGround(value)
    }
}

impl From<BackToWork> for Action {
    fn from(value: BackToWork) -> Self {
        Self::BackToWork(value)
    }
}

impl From<Build> for Action {
    fn from(value: Build) -> Self {
        Self::Build(value)
    }
}

impl From<Buy> for Action {
    fn from(value: Buy) -> Self {
        Self::Buy(value)
    }
}

impl From<DeAttackMove> for Action {
    fn from(value: DeAttackMove) -> Self {
        Self::DeAttackMove(value)
    }
}

impl From<DeAutoScout> for Action {
    fn from(value: DeAutoScout) -> Self {
        Self::DeAutoScout(value)
    }
}

impl From<DeQueue> for Action {
    fn from(value: DeQueue) -> Self {
        Self::DeQueue(value)
    }
}

impl From<Delete> for Action {
    fn from(value: Delete) -> Self {
        Self::Delete(value)
    }
}

impl From<Flare> for Action {
    fn from(value: Flare) -> Self {
        Self::Flare(value)
    }
}

impl From<Follow> for Action {
    fn from(value: Follow) -> Self {
        Self::Follow(value)
    }
}

impl From<Formation> for Action {
    fn from(value: Formation) -> Self {
        Self::Formation(value)
    }
}

impl From<Game> for Action {
    fn from(value: Game) -> Self {
        Self::Game(value)
    }
}

impl From<GatherPoint> for Action {
    fn from(value: GatherPoint) -> Self {
        Self::GatherPoint(value)
    }
}

impl From<Guard> for Action {
    fn from(value: Guard) -> Self {
        Self::Guard(value)
    }
}

impl From<Interact> for Action {
    fn from(value: Interact) -> Self {
        Self::Interact(value)
    }
}

impl From<Move> for Action {
    fn from(value: Move) -> Self {
        Self::Move(value)
    }
}

impl From<Order> for Action {
    fn from(value: Order) -> Self {
        Self::Order(value)
    }
}

impl From<Patrol> for Action {
    fn from(value: Patrol) -> Self {
        Self::Patrol(value)
    }
}

impl From<Queue> for Action {
    fn from(value: Queue) -> Self {
        Self::Queue(value)
    }
}

impl From<Release> for Action {
    fn from(value: Release) -> Self {
        Self::Release(value)
    }
}

impl From<Repair> for Action {
    fn from(value: Repair) -> Self {
        Self::Repair(value)
    }
}

impl From<Research> for Action {
    fn from(value: Research) -> Self {
        Self::Research(value)
    }
}

impl From<Resign> for Action {
    fn from(value: Resign) -> Self {
        Self::Resign(value)
    }
}

impl From<Sell> for Action {
    fn from(value: Sell) -> Self {
        Self::Sell(value)
    }
}

impl From<Stance> for Action {
    fn from(value: Stance) -> Self {
        Self::Stance(value)
    }
}

impl From<Stop> for Action {
    fn from(value: Stop) -> Self {
        Self::Stop(value)
    }
}

impl From<ToggleGate> for Action {
    fn from(value: ToggleGate) -> Self {
        Self::ToggleGate(value)
    }
}

impl From<Action0x23> for Action {
    fn from(value: Action0x23) -> Self {
        Self::Action0x23(value)
    }
}

impl From<Action0x25> for Action {
    fn from(value: Action0x25) -> Self {
        Self::Action0x25(value)
    }
}

impl From<Action0x29> for Action {
    fn from(value: Action0x29) -> Self {
        Self::Action0x29(value)
    }
}

impl From<Action0x2c> for Action {
    fn from(value: Action0x2c) -> Self {
        Self::Action0x2c(value)
    }
}

impl From<Action0x2d> for Action {
    fn from(value: Action0x2d) -> Self {
        Self::Action0x2d(value)
    }
}

impl From<Action0x82> for Action {
    fn from(value: Action0x82) -> Self {
        Self::Action0x82(value)
    }
}

impl From<Action0x83> for Action {
    fn from(value: Action0x83) -> Self {
        Self::Action0x83(value)
    }
}

impl From<Action0x8c> for Action {
    fn from(value: Action0x8c) -> Self {
        Self::Action0x8c(value)
    }
}

impl From<Action0xc4> for Action {
    fn from(value: Action0xc4) -> Self {
        Self::Action0xc4(value)
    }
}

impl From<Wall> for Action {
    fn from(value: Wall) -> Self {
        Self::Wall(value)
    }
}

impl From<Waypoint> for Action {
    fn from(value: Waypoint) -> Self {
        Self::Waypoint(value)
    }
}
