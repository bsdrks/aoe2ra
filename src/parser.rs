use crate::{
    body::Body,
    body_meta::BodyMeta,
    check_flags::check_flags,
    formation_type::FormationType,
    operation::{
        action::{
            game::{
                GameActionMode,
                InstantBuild,
                Speed,
            },
            ActionType,
            AddAttribute,
            AiInteract,
            AiMove,
            AiQueue,
            AttackGround,
            BackToWork,
            Build,
            Buy,
            DeAttackMove,
            DeAutoScout,
            DeQueue,
            Delete,
            Flare,
            Follow,
            Formation,
            Game,
            GatherPoint,
            Guard,
            Interact,
            Move,
            Order,
            Patrol,
            Release,
            Repair,
            Research,
            Resign,
            Sell,
            Stance,
            Stop,
            ToggleGate,
            Wall,
            Waypoint,
        },
        sync::checksum::Checksum,
        Action,
        Chat,
        Operation,
        OperationType,
        Sync,
        ViewLock,
    },
    order_type::OrderType,
    release_type::ReleaseType,
    resource_type::ResourceType,
    stance_type::StanceType,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

pub struct Parser {
    pub cursor: usize,
    pub raw: Vec<u8>,
}

impl Parser {
    #[must_use]
    pub const fn new(raw: Vec<u8>) -> Self {
        Self { cursor: 0, raw }
    }

    pub fn parse_body(&mut self) -> Body {
        self.cursor = self.parse_u32() as usize;

        Body {
            meta: self.parse_body_meta(),
            operations: self.parse_operations(),
        }
    }

    fn parse_body_meta(&mut self) -> BodyMeta {
        let next = self.peek_u32();
        let log_version = (next != 500).then(|| self.parse_u32());
        let checksum_interval = self.parse_u32();
        let multiplayer = self.parse_u32();
        let rec_owner = self.parse_u32();
        let reveal_map = self.parse_u32();
        let use_sequence_numbers = self.parse_u32();
        let number_of_chapters = self.parse_u32();
        let aok = self.peek_bool_u32();

        if aok {
            self.skip_u8s(4);
            let p = self.peek_u32();

            if p != 2 {
                self.skip_u8s(8);
            }
        }

        BodyMeta {
            log_version,
            checksum_interval,
            multiplayer: multiplayer != 0,
            rec_owner,
            reveal_map,
            use_sequence_numbers: use_sequence_numbers != 0,
            number_of_chapters,
            aok,
        }
    }

    fn parse_operations(&mut self) -> Vec<Operation> {
        let mut operations = Vec::new();

        while self.cursor < self.raw.len() {
            let operation = self.parse_operation();

            match operation {
                Some(Operation::Chat(Chat { ref text }))
                    if text == "\u{3}" =>
                {
                    break;
                }
                Some(operation) => operations.push(operation),
                _ => {}
            }
        }

        operations
    }

    fn parse_operation(&mut self) -> Option<Operation> {
        let operation_type = self.parse_operation_type();

        match operation_type {
            OperationType::Action => Some(self.parse_action()),
            OperationType::Sync => Some(self.parse_sync()),
            OperationType::ViewLock => Some(self.parse_view_lock()),
            OperationType::Chat => Some(self.parse_chat()),
            OperationType::Other => None,
        }
    }

    fn parse_operation_type(&mut self) -> OperationType {
        let operation_type_u32 = self.parse_u32();

        OperationType::from(operation_type_u32)
    }

    fn parse_action(&mut self) -> Operation {
        let length = self.parse_u32();
        let cursor_next = self.cursor + 4 + length as usize;
        let action_type_u8 = self.parse_u8();
        let action_type = ActionType::from(action_type_u8);

        let action = match action_type {
            ActionType::AddAttribute => self.parse_add_attribute(),
            ActionType::AiCommand => unimplemented!("AiCommand"),
            ActionType::AiInteract => self.parse_ai_interact(),
            ActionType::AiMove => self.parse_ai_move(),
            ActionType::AiQueue => self.parse_ai_queue(),
            ActionType::AiWaypoint => unimplemented!("AiWaypoint"),
            ActionType::AttackGround => self.parse_attack_ground(),
            ActionType::BackToWork => self.parse_back_to_work(),
            ActionType::Build => self.parse_build(),
            ActionType::Buy => self.parse_buy(),
            ActionType::Chapter => unimplemented!("Chapter"),
            ActionType::Create => unimplemented!("Create"),
            ActionType::DeAttackMove => self.parse_de_attack_move(),
            ActionType::DeAutoScout => self.parse_de_auto_scout(),
            ActionType::DeQueue => self.parse_de_queue(),
            ActionType::Delete => self.parse_delete(),
            ActionType::DropRelic => unimplemented!("DropRelic"),
            ActionType::Flare => self.parse_flare(),
            ActionType::Follow => self.parse_follow(),
            ActionType::Formation => self.parse_formation(),
            ActionType::Game => self.parse_game(),
            ActionType::GatherPoint => self.parse_gather_point(),
            ActionType::GiveAttribute => unimplemented!("GiveAttribute"),
            ActionType::Guard => self.parse_guard(),
            ActionType::Interact => self.parse_interact(),
            ActionType::Move => self.parse_move(),
            ActionType::MultiQueue => unimplemented!("MultiQueue"),
            ActionType::Order => self.parse_order(),
            ActionType::Patrol => self.parse_patrol(),
            ActionType::PostGame => unimplemented!("PostGame"),
            ActionType::Queue => unimplemented!("Queue"),
            ActionType::Release => self.parse_release(),
            ActionType::Repair => self.parse_repair(),
            ActionType::Research => self.parse_research(),
            ActionType::Resign => self.parse_resign(),
            ActionType::Save => unimplemented!("Save"),
            ActionType::Sell => self.parse_sell(),
            ActionType::Spec => unimplemented!("Spec"),
            ActionType::Stance => self.parse_stance(),
            ActionType::Stop => self.parse_stop(),
            ActionType::ToggleGate => self.parse_toggle_gate(),
            ActionType::TownBell => unimplemented!("TownBell"),
            ActionType::Tribute => unimplemented!("Tribute"),
            ActionType::Wall => self.parse_wall(length),
            ActionType::Waypoint => self.parse_waypoint(),
            ActionType::Unknown => Operation::Unknown,
        };

        assert!(
            self.cursor <= cursor_next,
            "expected cursor to be at {} but was at {}, parsed {:?}, length: \
             {}",
            cursor_next,
            self.cursor,
            action,
            length
        );

        self.cursor = cursor_next;

        action
    }

    fn parse_sync(&mut self) -> Operation {
        let time_increment = self.parse_u32();
        let next = self.peek_u32();
        let checksum = (next == 0).then(|| self.parse_sync_checksum());

        Operation::Sync(r#Sync {
            time_increment,
            checksum,
        })
    }

    fn parse_view_lock(&mut self) -> Operation {
        let x = self.parse_f32();
        let y = self.parse_f32();
        let player_id = self.parse_u8();
        self.skip_u8s(3);

        Operation::ViewLock(ViewLock { x, y, player_id })
    }

    fn parse_chat(&mut self) -> Operation {
        self.skip_u8s(4);
        let length = self.parse_u32();
        let text = self.parse_u8s(length as usize);

        Operation::Chat(Chat {
            text: text.into_iter().map(|b| b as char).collect(),
        })
    }

    fn parse_sync_checksum(&mut self) -> Checksum {
        self.skip_u8s(8);
        let sync = self.parse_u32();
        self.skip_u8s(4);
        let sequence = self.parse_u32();
        let checksum = (sequence > 0).then(|| self.parse_u8s(332));
        self.skip_u8s(8);

        Checksum {
            sync,
            sequence,
            checksum,
        }
    }

    fn parse_add_attribute(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let attribute = self.parse_u8();
        self.skip_u8();
        let amount = self.parse_f32();

        Operation::Action(Action::AddAttribute(AddAttribute {
            player_id,
            attribute,
            amount,
        }))
    }

    fn parse_ai_interact(&mut self) -> Operation {
        self.skip_u8s(2);
        let target_id = self.parse_u32();
        let selected = self.parse_u8();
        self.skip_u8s(3);
        let x = self.parse_f32();
        let y = self.parse_f32();

        let unit_ids =
            (selected < 0xff).then(|| self.parse_u32s(selected as usize));

        Operation::Action(Action::AiInteract(AiInteract {
            target_id,
            x,
            y,
            unit_ids,
        }))
    }

    fn parse_ai_move(&mut self) -> Operation {
        let selected = self.parse_u8();
        let player_id = self.parse_u8();
        let player_num = self.parse_u8();
        self.skip_u8s(8);
        let target_id = self.parse_u32();
        self.skip_u8s(4);
        let x = self.parse_f32();
        let y = self.parse_f32();
        self.skip_u8s(12);

        let unit_ids =
            (selected > 0x01).then(|| self.parse_u32s(selected as usize));

        Operation::Action(Action::AiMove(AiMove {
            player_id,
            player_num,
            target_id,
            x,
            y,
            unit_ids,
        }))
    }

    fn parse_ai_queue(&mut self) -> Operation {
        self.skip_u8s(3);
        let building_id = self.parse_u32();
        let player_id = self.parse_u8();
        self.skip_u8();
        let unit_type = self.parse_u16();
        self.skip_u8s(4);

        Operation::Action(Action::AiQueue(AiQueue {
            building_id,
            player_id,
            unit_type,
        }))
    }

    fn parse_attack_ground(&mut self) -> Operation {
        let selected = self.parse_u8();
        self.skip_u8s(2);
        let x = self.parse_f32();
        let y = self.parse_f32();
        let next = self.peek_u8s(4);
        let flags = check_flags(&next).then(|| self.parse_u8s(4));
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::AttackGround(AttackGround {
            x,
            y,
            flags,
            unit_ids,
        }))
    }

    fn parse_back_to_work(&mut self) -> Operation {
        self.skip_u8s(3);
        let town_center_id = self.parse_u32();

        Operation::Action(Action::BackToWork(BackToWork { town_center_id }))
    }

    fn parse_build(&mut self) -> Operation {
        let selected = self.parse_u8();
        let player_id = self.parse_u8();
        self.skip_u8();
        let x = self.parse_f32();
        let y = self.parse_f32();
        let building_type = self.parse_u32();
        self.skip_u8s(4);
        let sprite_id = self.parse_u32();
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Build(Build {
            player_id,
            x,
            y,
            building_type,
            sprite_id,
            unit_ids,
        }))
    }

    fn parse_buy(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let resource_type_u8 = self.parse_u8();
        let resource_type = ResourceType::from(resource_type_u8);
        let amount = self.parse_u8();
        self.skip_u8s(4);

        Operation::Action(Action::Buy(Buy {
            player_id,
            resource_type,
            amount,
        }))
    }

    fn parse_de_attack_move(&mut self) -> Operation {
        let selected = self.parse_u8();
        let waypoints = self.parse_u16();
        let xs = self.parse_f32s(10);
        let ys = self.parse_f32s(10);
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::DeAttackMove(DeAttackMove {
            waypoints,
            xs,
            ys,
            unit_ids,
        }))
    }

    fn parse_de_auto_scout(&mut self) -> Operation {
        let selected = self.parse_u8();
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::DeAutoScout(DeAutoScout { unit_ids }))
    }

    fn parse_de_queue(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let building_type = self.parse_u16();
        let selected = self.parse_u8();
        self.skip_u8();
        let unit_type = self.parse_u16();
        let queue_amount = self.parse_u8();
        self.skip_u8();
        let building_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::DeQueue(DeQueue {
            player_id,
            building_type,
            unit_type,
            queue_amount,
            building_ids,
        }))
    }

    fn parse_delete(&mut self) -> Operation {
        self.skip_u8s(3);
        let object_id = self.parse_u32();
        let player_id = self.parse_u8();
        self.skip_u8s(3);

        Operation::Action(Action::Delete(Delete {
            object_id,
            player_id,
        }))
    }

    fn parse_flare(&mut self) -> Operation {
        self.skip_u8s(7);
        let player_ids = self.parse_u8s(9);
        self.skip_u8s(3);
        let x = self.parse_f32();
        let y = self.parse_f32();
        let player_id = self.parse_u8();
        let player_num = self.parse_u8();
        self.skip_u8s(2);

        Operation::Action(Action::Flare(Flare {
            player_ids,
            x,
            y,
            player_id,
            player_num,
        }))
    }

    fn parse_follow(&mut self) -> Operation {
        let selected = self.parse_u8();
        self.skip_u8s(2);
        let followed_unit_id = self.parse_u32();
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Follow(Follow {
            followed_unit_id,
            unit_ids,
        }))
    }

    fn parse_formation(&mut self) -> Operation {
        let selected = self.parse_u8();
        let player_id = self.parse_u8();
        self.skip_u8();
        let formation_type_u8 = self.parse_u8();
        let formation_type = FormationType::from(formation_type_u8);
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Formation(Formation {
            player_id,
            formation_type,
            unit_ids,
        }))
    }

    fn parse_game(&mut self) -> Operation {
        let mode_u8 = self.parse_u8();
        let mode = GameActionMode::from(mode_u8);
        let player_id = self.parse_u8();
        self.skip_u8();

        Operation::Action(Action::Game(match mode {
            GameActionMode::AlliedVictory => unimplemented!(),
            GameActionMode::Cheat => unimplemented!(),
            GameActionMode::DefaultStance => unimplemented!(),
            GameActionMode::Diplomacy => unimplemented!(),
            GameActionMode::FishTrapQueue => unimplemented!(),
            GameActionMode::FishTrapAutoQueue => unimplemented!(),
            GameActionMode::FishTrapUnqueue => unimplemented!(),
            GameActionMode::QuickBuild => unimplemented!(),
            GameActionMode::Spy => unimplemented!(),
            GameActionMode::Speed => self.parse_speed(player_id),
            GameActionMode::FarmAutoQueue => unimplemented!(),
            GameActionMode::FarmUnqueue => unimplemented!(),
            GameActionMode::InstantBuild => {
                self.parse_instant_build(player_id)
            }
            GameActionMode::FarmQueue => unimplemented!(),
        }))
    }

    fn parse_speed(&mut self, player_id: u8) -> Game {
        let speed = self.parse_f32();

        Game::Speed(Speed { player_id, speed })
    }

    fn parse_instant_build(&mut self, player_id: u8) -> Game {
        self.skip_u8s(9);

        Game::InstantBuild(InstantBuild { player_id })
    }

    fn parse_gather_point(&mut self) -> Operation {
        let selected = self.parse_u8();
        self.skip_u8s(2);
        let target_id = self.parse_u32_opt();
        let target_type = self.parse_u32_opt();
        let x = self.parse_f32();
        let y = self.parse_f32();
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::GatherPoint(GatherPoint {
            target_id,
            target_type,
            x,
            y,
            unit_ids,
        }))
    }

    fn parse_guard(&mut self) -> Operation {
        let selected = self.parse_u8();
        self.skip_u8s(2);
        let guarded_unit_id = self.parse_u32();
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Guard(Guard {
            guarded_unit_id,
            unit_ids,
        }))
    }

    fn parse_interact(&mut self) -> Operation {
        let player_id = self.parse_u8();
        self.skip_u8s(2);
        let target_id = self.parse_u32();
        let x = self.parse_f32();
        let y = self.parse_f32();
        let selected = self.parse_u8();
        self.skip_u8s(3);
        let next = self.peek_u8s(4);
        let flags = check_flags(&next).then(|| self.parse_u8s(4));

        let unit_ids =
            (selected < 0xff).then(|| self.parse_u32s(selected as usize));

        Operation::Action(Action::Interact(Interact {
            player_id,
            target_id,
            x,
            y,
            flags,
            unit_ids,
        }))
    }

    fn parse_move(&mut self) -> Operation {
        let player_id = self.parse_u8();
        self.skip_u8s(2);
        let constant = self.parse_u32();

        assert_eq!(
            constant, 0xffff_ffff,
            "expected constant 0xffff_ffff, cursor: {}",
            self.cursor
        );

        let selected = self.parse_u8();
        self.skip_u8s(3);
        let x = self.parse_f32();
        let y = self.parse_f32();
        let next = self.peek_u8s(4);
        let flags = check_flags(&next).then(|| self.parse_u8s(4));
        let unit_ids = self.parse_u32s(selected as usize);
        self.skip_u8s(4);

        Operation::Action(Action::Move(Move {
            player_id,
            x,
            y,
            flags,
            unit_ids,
        }))
    }

    fn parse_order(&mut self) -> Operation {
        let selected = self.parse_u8();
        self.skip_u8s(2);
        let building_id = self.parse_i32();
        let order_type_u8 = self.parse_u8();
        let order_type = OrderType::from(order_type_u8);
        let cancel_order = self.parse_u8();
        self.skip_u8s(2);
        let x = self.parse_f32();
        let y = self.parse_f32();
        self.skip_u8s(4);
        let next = self.peek_u8s(4);
        let flags = check_flags(&next).then(|| self.parse_u8s(4));
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Order(Order {
            building_id,
            order_type,
            cancel_order,
            x,
            y,
            flags,
            unit_ids,
        }))
    }

    fn parse_patrol(&mut self) -> Operation {
        let selected = self.parse_u8();
        let waypoints = self.parse_u16();
        let xs = self.parse_f32s(10);
        let ys = self.parse_f32s(10);
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Patrol(Patrol {
            waypoints,
            xs,
            ys,
            unit_ids,
        }))
    }

    fn parse_release(&mut self) -> Operation {
        let selected = self.parse_u8();
        self.skip_u8s(2);
        let x = self.parse_f32_opt();
        let y = self.parse_f32_opt();
        let release_type_u8 = self.parse_u8();
        let release_type = ReleaseType::from(release_type_u8);
        self.skip_u8s(3);
        let release_id = self.parse_u32_opt();
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Release(Release {
            x,
            y,
            release_type,
            release_id,
            unit_ids,
        }))
    }

    fn parse_repair(&mut self) -> Operation {
        let selected = self.parse_u8();
        self.skip_u8s(2);
        let repaired_id = self.parse_u32();
        let next = self.peek_u8s(4);
        let flags = check_flags(&next).then(|| self.parse_u8s(4));
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Repair(Repair {
            repaired_id,
            flags,
            unit_ids,
        }))
    }

    fn parse_research(&mut self) -> Operation {
        self.skip_u8s(3);
        let building_id = self.parse_u32();
        let player_id = self.parse_u8();
        self.skip_u8s(1);
        let check = self.peek_i32();

        let (technology_type, selected_ids) = if check == -1 {
            let selected = self.parse_u16();
            let technology_type = self.parse_u32();
            let selected_ids = self.parse_u32s(selected as usize);

            (technology_type, selected_ids)
        } else {
            let technology_type = u32::from(self.parse_u16());
            let selected_ids = self.parse_u32s(1);

            (technology_type, selected_ids)
        };

        Operation::Action(Action::Research(Research {
            building_id,
            player_id,
            technology_type,
            selected_ids,
        }))
    }

    fn parse_resign(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let player_num = self.parse_u8();
        let disconnected = self.parse_bool_u8();

        Operation::Action(Action::Resign(Resign {
            player_id,
            player_num,
            disconnected,
        }))
    }

    fn parse_sell(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let resource_type_u8 = self.parse_u8();
        let resource_type = ResourceType::from(resource_type_u8);
        let amount = self.parse_u8();
        self.skip_u8s(4);

        Operation::Action(Action::Sell(Sell {
            player_id,
            resource_type,
            amount,
        }))
    }

    fn parse_stance(&mut self) -> Operation {
        let selected = self.parse_u8();
        let stance_type_u8 = self.parse_u8();
        let stance_type = StanceType::from(stance_type_u8);
        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Stance(Stance {
            stance_type,
            unit_ids,
        }))
    }

    fn parse_stop(&mut self) -> Operation {
        let selected = self.parse_u8();
        let object_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Stop(Stop { object_ids }))
    }

    fn parse_toggle_gate(&mut self) -> Operation {
        self.skip_u8s(3);
        let gate_id = self.parse_u32();

        Operation::Action(Action::ToggleGate(ToggleGate { gate_id }))
    }

    fn parse_wall(&mut self, length: u32) -> Operation {
        let selected = self.parse_u8();
        let player_id = self.parse_u8();

        let (start_x, start_y, end_x, end_y, building_id, flags) =
            if length - 16 - (u32::from(selected) * 4) == 8 {
                self.skip_u8();
                let start_x = self.parse_u16();
                let start_y = self.parse_u16();
                let end_x = self.parse_u16();
                let end_y = self.parse_u16();
                let building_id = self.parse_u32();
                self.skip_u8s(4);
                let flags = self.parse_u8s(4);

                (start_x, start_y, end_x, end_y, building_id, flags)
            } else {
                let start_x = u16::from(self.parse_u8());
                let start_y = u16::from(self.parse_u8());
                let end_x = u16::from(self.parse_u8());
                let end_y = u16::from(self.parse_u8());
                self.skip_u8();
                let building_id = self.parse_u32();
                self.skip_u8s(4);

                (start_x, start_y, end_x, end_y, building_id, Vec::new())
            };

        let unit_ids = self.parse_u32s(selected as usize);

        Operation::Action(Action::Wall(Wall {
            player_id,
            start_x,
            start_y,
            end_x,
            end_y,
            building_id,
            flags,
            unit_ids,
        }))
    }

    fn parse_waypoint(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let selected = self.parse_u8();
        let x = self.parse_f32();
        let y = self.parse_f32();

        let selected_ids =
            (selected != 0xff).then(|| self.parse_u32s(selected as usize));

        Operation::Action(Action::Waypoint(Waypoint {
            player_id,
            x,
            y,
            selected_ids,
        }))
    }

    // Parse primitive values

    fn take_4(&self) -> [u8; 4] {
        [
            self.raw[self.cursor],
            self.raw[self.cursor + 1],
            self.raw[self.cursor + 2],
            self.raw[self.cursor + 3],
        ]
    }

    fn parse_bool_u8(&mut self) -> bool {
        match self.parse_u8() {
            0x00 => false,
            0x01 => true,
            n => panic!("0x00 or 0x01, found: 0x{n:02x}"),
        }
    }

    fn parse_f32(&mut self) -> f32 {
        let mut value = f32::from_le_bytes(self.take_4());

        if value.abs() < f32::EPSILON {
            value = 0.0;
        }

        self.cursor += 4;

        value
    }

    fn parse_f32_opt(&mut self) -> Option<f32> {
        let a = self.raw[self.cursor];
        let b = self.raw[self.cursor + 1];
        let c = self.raw[self.cursor + 2];
        let d = self.raw[self.cursor + 3];

        if a == 0xff && b == 0xff && c == 0xff && d == 0xff {
            self.cursor += 4;

            return None;
        }

        let value = f32::from_le_bytes([a, b, c, d]);

        if value.abs() < f32::EPSILON {
            self.cursor += 4;

            return Some(0.0);
        }

        self.cursor += 4;

        Some(value)
    }

    fn parse_f32s(&mut self, count: usize) -> Vec<f32> {
        (0..count).map(|_| self.parse_f32()).collect()
    }

    fn parse_i32(&mut self) -> i32 {
        let value = i32::from_le_bytes(self.take_4());

        self.cursor += 4;

        value
    }

    fn parse_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes([
            self.raw[self.cursor],
            self.raw[self.cursor + 1],
        ]);

        self.cursor += 2;

        value
    }

    fn parse_u32(&mut self) -> u32 {
        let value = self.peek_u32();

        self.cursor += 4;

        value
    }

    fn parse_u32_opt(&mut self) -> Option<u32> {
        match self.parse_u32() {
            0xffff_ffff => None,
            n => Some(n),
        }
    }

    fn parse_u32s(&mut self, count: usize) -> Vec<u32> {
        (0..count).map(|_| self.parse_u32()).collect()
    }

    fn parse_u8(&mut self) -> u8 {
        let value = self.raw[self.cursor];

        self.cursor += 1;

        value
    }

    fn parse_u8s(&mut self, count: usize) -> Vec<u8> {
        let cursor_next = self.cursor + count;
        let value = self.raw[self.cursor..cursor_next].to_vec();

        self.cursor = cursor_next;

        value
    }

    // Skip primitive values

    fn skip_u8(&mut self) {
        self.cursor += 1;
    }

    fn skip_u8s(&mut self, count: usize) {
        self.cursor += count;
    }

    // Peek primitive values

    fn peek_bool_u32(&self) -> bool {
        self.peek_u32() != 0
    }

    fn peek_i32(&self) -> i32 {
        i32::from_le_bytes(self.take_4())
    }

    fn peek_u32(&self) -> u32 {
        u32::from_le_bytes(self.take_4())
    }

    fn peek_u8s(&self, count: usize) -> Vec<u8> {
        self.raw[self.cursor..self.cursor + count].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bool_u8_false() {
        let mut parser = Parser {
            cursor: 0,
            raw: vec![0x00],
        };

        assert!(!parser.parse_bool_u8());
    }

    #[test]
    fn test_parse_bool_u8_true() {
        let mut parser = Parser {
            cursor: 0,
            raw: vec![0x01],
        };

        assert!(parser.parse_bool_u8());
    }

    #[test]
    #[should_panic(expected = "0x00 or 0x01, found: 0x02")]
    fn test_parse_bool_panic() {
        let mut parser = Parser {
            cursor: 0,
            raw: vec![0x02],
        };

        let _ = parser.parse_bool_u8();
    }
}
