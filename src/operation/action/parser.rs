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
            Unknown0x25,
            Unknown0x29,
            Unknown0x82,
            Unknown0x83,
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

    #[must_use]
    pub fn parse_body(&mut self) -> Body {
        let cursor = self.parse_usize_u32();
        self.set_cursor(cursor);

        Body {
            meta: self.parse_body_meta(),
            operations: self.parse_operations(),
        }
    }

    fn advance_cursor(&mut self, count: usize) {
        self.cursor += count;
    }

    fn parse_action_type(&mut self) -> ActionType {
        let action_type_u8 = self.parse_u8();

        ActionType::from(action_type_u8)
    }

    fn parse_action(&mut self) -> Operation {
        println!();
        println!("{}", self.cursor);
        let length = self.parse_usize_u32();
        println!("{length}");
        let cursor_next = self.cursor + length;
        let action_type = self.parse_action_type();
        println!("{action_type:?}");

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
            ActionType::Unknown0x25 => self.parse_unknown_0x25(),
            ActionType::Unknown0x29 => self.parse_unknown_0x29(),
            ActionType::Unknown0x82 => self.parse_unknown_0x82(),
            ActionType::Unknown0x83 => self.parse_unknown_0x83(),
            ActionType::Wall => self.parse_wall(length),
            ActionType::Waypoint => self.parse_waypoint(),
        };

        println!("{action:?}");

        assert!(
            self.cursor != cursor_next,
            "expected cursor ({}) to be {cursor_next}, length: {length}, \
             after action {action_type:?}",
            self.cursor,
        );

        action
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

    // Examples:
    // 01000000_1C000000_02021800_FB360000_E8E4C042_316C9942_01000000_01000000_C21D0000_9C130000
    fn parse_ai_interact(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let target_id = self.parse_u32();
        let x = self.parse_f32();
        let y = self.parse_f32();
        let selected = self.parse_usize_u32_opt();
        let flags = self.parse_flags(4);
        let unknown_u32_1 = self.parse_u32();
        let unit_ids = selected.map(|selected| self.parse_u32s(selected));

        Operation::Action(Action::AiInteract(AiInteract {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            target_id,
            x,
            y,
            flags,
            unknown_u32_1,
            unit_ids,
        }))
    }

    // Examples:
    // 01000000_2C000000_0A022800_01000000_CA1D0000_FFFFFFFF_00000000_C1020000_0000A442_00009642_000080BF_0000803F_01FF0100_43000000
    // 01000000_2C000000_0A022800_01000000_C71D0000_53370000_00000000_BE020000_0000C442_00008A42_00000000_0000803F_63020101_56280000
    // 01000000_38000000_0A023400_03000000_481D0000_FFFFFFFF_00000000_C2020000_000080BF_000080BF_000080BF_000080BF_64FF0100_00000000_491D0000_1B380000_B8680000
    fn parse_ai_move(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let selected = self.parse_usize_u32_opt();
        let unknown_u32_1 = self.parse_u32();
        let unknown_u32_2 = self.parse_u32_opt();
        let unknown_u32_3 = self.parse_u32();
        let unknown_u32_4 = self.parse_u32();
        let x = self.parse_f32();
        let y = self.parse_f32();
        let unknown_f32_1 = self.parse_f32();
        let unknown_f32_2 = self.parse_f32();
        let unknown_u32_5 = self.parse_u32();
        let flags = self.parse_flags(4);
        let unit_ids = selected.map(|selected| self.parse_u32s(selected));

        Operation::Action(Action::AiMove(AiMove {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u32_4,
            x,
            y,
            unknown_f32_1,
            unknown_f32_2,
            unknown_u32_5,
            flags,
            unit_ids,
        }))
    }

    // Examples:
    // 01000000_10000000_64020C00_BC1D0000_FFFFFFFF_53000000_C77F0000
    fn parse_ai_queue(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let unknown_u32_1 = self.parse_u32();
        let unknown_u32_2 = self.parse_u32_opt();
        let unknown_u32_3 = self.parse_u32();
        let unknown_u32_4 = self.parse_u32();

        Operation::Action(Action::AiQueue(AiQueue {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u32_4,
        }))
    }

    fn parse_attack_ground(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        self.skip_u8s(2);
        let x = self.parse_f32();
        let y = self.parse_f32();
        let flags = self.parse_flags(4);
        let unit_ids = self.parse_u32s(selected);

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

    fn parse_bool_u8(&mut self) -> bool {
        match self.parse_u8() {
            0x00 => false,
            0x01 => true,
            n => panic!("0x00 or 0x01, found: {n}"),
        }
    }

    // Examples:
    // 01000000_28000000_66012400_02000000_00005842_00007041_46000000_FFFFFFFF_FFFFFFFF_00000001_C41D0000_C51D0000_C9150000
    // 01000000_24000000_66022000_01000000_0000C442_00008A42_46000000_00000000_FFFFFFFF_00000001_C81D0000_49280000
    fn parse_build(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let selected = self.parse_usize_u32();
        let x = self.parse_f32();
        let y = self.parse_f32();
        let building_type = self.parse_u32();
        let unknown_u32_1 = self.parse_u32();
        let unknown_u32_2 = self.parse_u32();
        let unknown_u32_3 = self.parse_u32();
        let sprite_id = self.parse_u32();
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::Build(Build {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            x,
            y,
            building_type,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
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

    fn parse_chat(&mut self) -> Operation {
        self.skip_u8s(4);
        let length = self.parse_u32();
        let text = self.parse_u8s(length as usize);

        Operation::Chat(Chat {
            text: text.into_iter().map(|b| b as char).collect(),
        })
    }

    fn parse_de_attack_move(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        let waypoints = self.parse_u16();
        let xs = self.parse_f32s(10);
        let ys = self.parse_f32s(10);
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::DeAttackMove(DeAttackMove {
            waypoints,
            xs,
            ys,
            unit_ids,
        }))
    }

    fn parse_de_auto_scout(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::DeAutoScout(DeAutoScout { unit_ids }))
    }

    // Examples:
    // 01000000_14000000_81011000_01000000_00006D00_53000100_B61D0000_4A040000
    // 01000000_14000000_81011000_01000000_00006D00_53000100_B61D0000_92060000
    fn parse_de_queue(&mut self) -> Operation {
        println!("parsing de_queue: {}", self.cursor);
        let player_id = self.parse_u8();
        let building_type = self.parse_u16();
        let selected = self.parse_usize_u8();
        let unit_type = self.parse_u16();
        let queue_amount = self.parse_u8();
        self.skip_u8();
        let building_ids = self.parse_u32s(selected);

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

    fn parse_f32(&mut self) -> f32 {
        let value = f32::from_le_bytes(self.take_4());

        self.advance_cursor(4);

        value
    }

    fn parse_f32s(&mut self, count: usize) -> Vec<f32> {
        (0..count).map(|_| self.parse_f32()).collect()
    }

    fn parse_flags(&mut self, count: usize) -> Option<Vec<u8>> {
        let next = self.peek_u8s(count);

        check_flags(&next).then(|| self.parse_u8s(count))
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
        let selected = self.parse_usize_u8();
        self.skip_u8s(2);
        let followed_unit_id = self.parse_u32();
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::Follow(Follow {
            followed_unit_id,
            unit_ids,
        }))
    }

    fn parse_formation(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        let player_id = self.parse_u8();
        self.skip_u8();
        let formation_type_u8 = self.parse_u8();
        let formation_type = FormationType::from(formation_type_u8);
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::Formation(Formation {
            player_id,
            formation_type,
            unit_ids,
        }))
    }

    fn parse_game_mode(&mut self) -> GameActionMode {
        let mode_u8 = self.parse_u8();

        GameActionMode::from(mode_u8)
    }

    // Examples:
    // 01000000_14000000_67011000_10000000_01000000_00000000_00000000_1B000000
    // 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_859C0200
    // 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_7A0A0000
    // 01000000_14000000_67011000_13000000_01000000_00000000_00000000_1B000000
    fn parse_game(&mut self) -> Operation {
        let mode = self.parse_game_mode();

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
            GameActionMode::Speed => self.parse_speed(),
            GameActionMode::FarmAutoQueue => unimplemented!(),
            GameActionMode::FarmUnqueue => unimplemented!(),
            GameActionMode::InstantBuild => self.parse_instant_build(),
            GameActionMode::FarmQueue => unimplemented!(),
        }))
    }

    // 01000000_1D000000_78011900_01000000_AB8A7C42_5515A741_FFFFFFFF_FFFFFFFF_00B61D00_00B4DC02_00
    fn parse_gather_point(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let unknown_u32_1 = self.parse_u32();
        let x = self.parse_f32();
        let y = self.parse_f32();
        let unknown_u32_2 = self.parse_u32_opt();
        let unknown_u32_3 = self.parse_u32_opt();
        let unknown_u8_3 = self.parse_u8();
        let unknown_u32_4 = self.parse_u32();
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::GatherPoint(GatherPoint {
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            x,
            y,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u8_3,
            unknown_u32_4,
            unit_ids,
        }))
    }

    fn parse_guard(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        self.skip_u8s(2);
        let guarded_unit_id = self.parse_u32();
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::Guard(Guard {
            guarded_unit_id,
            unit_ids,
        }))
    }

    // Examples:
    // 01000000_1C000000_00021800_BC1D0000_0000C242_00009642_01000000_0100FFFF_C71D0000_FE280700
    // 01000000_1C000000_00011800_40370000_00008F42_00002E42_01000000_0101FFFF_8F370000_9A3E0700
    // 01000000_20000000_00011C00_BD360000_00008542_00009C41_02000000_0101FFFF_C51D0000_7D370000_BF780700
    // 01000000_24000000_00012000_CA1D0000_5BA19242_33D72942_03000000_0101FFFF_C41D0000_54370000_5B370000_DD0C0800
    // 01000000_28000000_00012400_F7360000_AFE25342_7C2DAD41_04000001_0101FFFF_C41D0000_C51D0000_C31D0000_C91D0000_5D390000
    // 01000000_18000000_00011400_01370000_00004242_0000B040_FFFF0000_0101FFFF_C24C0000
    fn parse_interact(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let target_id = self.parse_u32();
        let x = self.parse_f32();
        let y = self.parse_f32();
        let selected = self.parse_usize_u16_opt();
        let flags = self.parse_flags(4);
        let unknown_u16_1 = self.parse_u16();
        let unknown_u32_1 = self.parse_u32();
        let unit_ids = selected.map(|selected| self.parse_u32s(selected));

        Operation::Action(Action::Interact(Interact {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            target_id,
            x,
            y,
            flags,
            unknown_u16_1,
            unknown_u32_1,
            unit_ids,
        }))
    }

    // Examples:
    // 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_859C0200
    // 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_59140200
    // 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_1E010200
    fn parse_instant_build(&mut self) -> Game {
        let unknown_u16_1 = self.parse_u16();
        let unknown_u32_1 = self.parse_u32();
        let unknown_u16_2 = self.parse_u16();
        let unknown_u16_3 = self.parse_u16();
        let unknown_u32_2 = self.parse_u32();
        let unknown_u32_3 = self.parse_u32();
        let unknown_u16_4 = self.parse_u16();
        let unknown_u16_5 = self.parse_u16();

        Game::InstantBuild(InstantBuild {
            unknown_u16_1,
            unknown_u32_1,
            unknown_u16_2,
            unknown_u16_3,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u16_4,
            unknown_u16_5,
        })
    }

    // Examples:
    // 01000000_28000000_03012400_FFFFFFFF_55154942_55D5C741_04000000_01010000_F9360000_F7360000_F8360000_F6360000_0E190000
    fn parse_move(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let constant = self.parse_u32();

        assert_eq!(
            constant, 0xffff_ffff,
            "expected constant 0xffff_ffff, cursor: {}",
            self.cursor
        );

        let x = self.parse_f32();
        let y = self.parse_f32();
        let selected = self.parse_usize_u8_opt();
        let flags = self.parse_flags(4);
        let unknown_u32_1 = self.parse_u32();
        let unit_ids = selected.map(|selected| self.parse_u32s(selected));

        Operation::Action(Action::Move(Move {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            x,
            y,
            flags,
            unit_ids,
            unknown_u32_1,
        }))
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

    fn parse_order_type(&mut self) -> OrderType {
        let order_type_u8 = self.parse_u8();

        OrderType::from(order_type_u8)
    }

    // Examples:
    // 01000000_3D000000_75023900_07000000_FFFFFFFF_000080BF_000080BF_FFFFFFFF_00000000_07000001_00481D00_00491D00_004A1D00_00243800_002E3800_00283800_003B3800_00C88D03_00
    fn parse_order(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let selected = self.parse_usize_u8();
        let unknown_u32_1 = self.parse_u32_opt();
        let x = self.parse_f32();
        let y = self.parse_f32();
        let unknown_u32_2 = self.parse_u32_opt();
        let unknown_u8_3 = self.parse_u8();
        let unknown_u8_4 = self.parse_u8();
        let unknown_u8_5 = self.parse_u8();
        let order_type = self.parse_order_type();
        let unknown_u8_7 = self.parse_u8();
        let unit_ids = self.parse_u32s(selected);
        let unknown_u32_3 = self.parse_u32();

        Operation::Action(Action::Order(Order {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            x,
            y,
            unknown_u32_2,
            unknown_u8_3,
            unknown_u8_4,
            unknown_u8_5,
            order_type,
            unknown_u8_7,
            unit_ids,
            unknown_u32_3,
        }))
    }

    fn parse_patrol(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        let waypoints = self.parse_u16();
        let xs = self.parse_f32s(10);
        let ys = self.parse_f32s(10);
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::Patrol(Patrol {
            waypoints,
            xs,
            ys,
            unit_ids,
        }))
    }

    fn parse_release(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        self.skip_u8s(2);
        let x = self.parse_f32();
        let y = self.parse_f32();
        let release_type_u8 = self.parse_u8();
        let release_type = ReleaseType::from(release_type_u8);
        self.skip_u8s(3);
        let release_id = self.parse_u32_opt();
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::Release(Release {
            x,
            y,
            release_type,
            release_id,
            unit_ids,
        }))
    }

    fn parse_repair(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        self.skip_u8s(2);
        let repaired_id = self.parse_u32();
        let flags = self.parse_flags(4);
        let unit_ids = self.parse_u32s(selected);

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
            let selected = self.parse_usize_u8();
            self.skip_u8();
            let technology_type = self.parse_u32();
            let selected_ids = self.parse_u32s(selected);

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

    // Examples:
    // 01000000_14000000_67011000_10000000_01000000_00000000_00000000_1B000000
    // 01000000_14000000_67011000_13000000_01000000_00000000_00000000_1B000000
    fn parse_speed(&mut self) -> Game {
        let unknown_u16_1 = self.parse_u16();
        let unknown_u32_1 = self.parse_u32();
        let unknown_u32_2 = self.parse_u32();
        let unknown_u32_3 = self.parse_u32();
        let unknown_u32_4 = self.parse_u32();
        let unknown_u32_5 = self.parse_u32();

        Game::Speed(Speed {
            unknown_u16_1,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u32_4,
            unknown_u32_5,
        })
    }

    fn parse_stance_type(&mut self) -> StanceType {
        let stance_type_u8 = self.parse_u8();

        StanceType::from(stance_type_u8)
    }

    fn parse_stance(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        let stance_type = self.parse_stance_type();
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::Stance(Stance {
            stance_type,
            unit_ids,
        }))
    }

    fn parse_stop(&mut self) -> Operation {
        println!("> {}", self.cursor);
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let selected = self.parse_usize_u8();
        let unknown_u32_1 = self.parse_u32();
        let target_id = self.parse_u32();
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::Stop(Stop {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            target_id,
            unit_ids,
        }))
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

    fn parse_toggle_gate(&mut self) -> Operation {
        self.skip_u8s(3);
        let gate_id = self.parse_u32();

        Operation::Action(Action::ToggleGate(ToggleGate { gate_id }))
    }

    // Examples:
    // 01000000_54000000_23025000_3F1D0000_0000AC42_0000C841_0F000000_01000000_A6380000_BA380000_C1380000_C6380000_CF380000_D6380000_DD380000_F5380000_FC380000_0B390000_0C390000_12390000_13390000_1E390000_1F390000_30AA1100
    fn parse_unknown_0x23(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let unknown_u32_1 = self.parse_u32_opt();
        let unknown_u32_2 = self.parse_u32();
        let unknown_u8_3 = self.parse_u8();

        Operation::Action(Action::Unknown0x23(Unknown0x23 {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u8_3,
        }))
    }

    // Examples:
    // 01000000_09000000_25020500_FFFFFFFF_0080390F_00
    // 01000000_09000000_25020500_FFFFFFFF_0330AA11_00
    fn parse_unknown_0x25(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let unknown_u32_1 = self.parse_u32_opt();
        let unknown_u32_2 = self.parse_u32();
        let unknown_u8_3 = self.parse_u8();

        Operation::Action(Action::Unknown0x25(Unknown0x25 {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u8_3,
        }))
    }

    fn parse_unknown_0x29(&mut self) -> Operation {
        self.skip_u8s(3);

        Operation::Action(Action::Unknown0x29(Unknown0x29 {}))
    }

    // Examples:
    // 01000000_0D000000_83020900_01000000_02573700_0016A400_00
    fn parse_unknown_0x83(&mut self) -> Operation {
        let selected = self.parse_usize_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let unknown_u32_1 = self.parse_u32();
        let unknown_u8_3 = self.parse_u8();
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::Unknown0x83(Unknown0x83 {
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u8_3,
            unit_ids,
        }))
    }

    // Examples:
    // 01000000 11000000 82020D00 02000000 01C73700 00C83700 002DF60A 00
    // 01000000_0D000000_82020900_01000000_00BC1D00_00859C02_00
    // 01000000_0D000000_82020900_01000000_00BC1D00_007A0A00_00
    fn parse_unknown_0x82(&mut self) -> Operation {
        let player_id = self.parse_u8();
        let unknown_u8_1 = self.parse_u8();
        let unknown_u8_2 = self.parse_u8();
        let selected = self.parse_usize_u32();
        let unknown_u8_3 = self.parse_u8();
        let unknown_u32_1 = self.parse_u32();
        let unit_ids = self.parse_u32s(selected);

        Operation::Action(Action::Unknown0x82(Unknown0x82 {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u8_3,
            unknown_u32_1,
            unit_ids,
        }))
    }

    fn parse_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes(self.take_2());

        self.advance_cursor(2);

        value
    }

    fn parse_u16_opt(&mut self) -> Option<u16> {
        match self.parse_u16() {
            0xffff => None,
            value => Some(value),
        }
    }

    fn parse_u32(&mut self) -> u32 {
        let value = self.peek_u32();

        self.advance_cursor(4);

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

        self.advance_cursor(1);

        value
    }

    fn parse_u8_opt(&mut self) -> Option<u8> {
        match self.parse_u8() {
            0xff => None,
            value => Some(value),
        }
    }

    fn parse_u8s(&mut self, count: usize) -> Vec<u8> {
        let cursor_next = self.cursor + count;
        let value = self.raw[self.cursor..cursor_next].to_vec();

        self.set_cursor(cursor_next);

        value
    }

    fn parse_usize_u16_opt(&mut self) -> Option<usize> {
        self.parse_u16_opt().map(|value| value as usize)
    }

    fn parse_usize_u32(&mut self) -> usize {
        self.parse_u32() as usize
    }

    fn parse_usize_u32_opt(&mut self) -> Option<usize> {
        self.parse_u32_opt().map(|value| value as usize)
    }

    fn parse_usize_u8(&mut self) -> usize {
        self.parse_u8() as usize
    }

    fn parse_usize_u8_opt(&mut self) -> Option<usize> {
        self.parse_u8_opt().map(|value| value as usize)
    }

    fn parse_view_lock(&mut self) -> Operation {
        let x = self.parse_f32();
        let y = self.parse_f32();
        let player_id = self.parse_u8();
        self.skip_u8s(3);

        Operation::ViewLock(ViewLock { x, y, player_id })
    }

    fn parse_wall(&mut self, length: usize) -> Operation {
        let selected = self.parse_usize_u8();
        let player_id = self.parse_u8();

        let (start_x, start_y, end_x, end_y, building_id, flags) =
            if length - 16 - (selected * 4) == 8 {
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

        let unit_ids = self.parse_u32s(selected);

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
        let selected = self.parse_usize_u8_opt();
        let x = self.parse_f32();
        let y = self.parse_f32();
        let selected_ids = selected.map(|selected| self.parse_u32s(selected));

        Operation::Action(Action::Waypoint(Waypoint {
            player_id,
            x,
            y,
            selected_ids,
        }))
    }

    fn skip_u8(&mut self) {
        self.advance_cursor(1);
    }

    fn skip_u8s(&mut self, count: usize) {
        self.advance_cursor(count);
    }

    fn take_2(&self) -> [u8; 2] {
        [self.raw[self.cursor], self.raw[self.cursor + 1]]
    }

    fn take_4(&self) -> [u8; 4] {
        [
            self.raw[self.cursor],
            self.raw[self.cursor + 1],
            self.raw[self.cursor + 2],
            self.raw[self.cursor + 3],
        ]
    }

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

    fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    fn hex(str: &str) -> Vec<u8> {
        str.trim()
            .replace([' ', '\n'], "")
            .chars()
            .collect::<Vec<_>>()
            .chunks_exact(2)
            .map(|byte_chars| {
                u8::from_str_radix(&byte_chars.iter().collect::<String>(), 16)
                    .ok()
                    .unwrap()
            })
            .collect()
    }

    #[test]
    fn test_parse_bool_u8_false() {
        let mut parser = Parser::new(hex("00"));

        assert!(!parser.parse_bool_u8());
    }

    #[test]
    fn test_parse_bool_u8_true() {
        let mut parser = Parser::new(hex("01"));

        assert!(parser.parse_bool_u8());
    }

    #[test]
    #[should_panic(expected = "0x00 or 0x01, found: 2")]
    fn test_parse_bool_u8_panic() {
        let mut parser = Parser::new(hex("02"));
        let _ = parser.parse_bool_u8();
    }

    #[test]
    fn test_parse_f32_0xffff_ffff() {
        let mut parser = Parser::new(hex("FFFFFFFF"));

        assert!(parser.parse_f32().is_nan());
    }

    #[test]
    fn test_parse_f32s() {
        let mut parser = Parser::new(hex("00004841 00004849 0000473C"));

        assert_eq!(parser.parse_f32s(3), vec![12.5, 819_200.0, 0.012_145_996]);
    }

    #[test]
    fn test_parse_f32_min() {
        let mut parser = Parser::new(hex("00000000"));

        assert_eq!(parser.parse_f32(), 0.0);
    }

    #[test]
    fn test_parse_f32_non_min() {
        let mut parser = Parser::new(hex("00004841"));

        assert_eq!(parser.parse_f32(), 12.5);
    }

    #[test]
    fn test_parse_u16_min() {
        let mut parser = Parser::new(hex("0000"));

        assert_eq!(parser.parse_u16(), 0);
    }

    #[test]
    fn test_parse_u16_non_min() {
        let mut parser = Parser::new(hex("0A00"));

        assert_eq!(parser.parse_u16(), 10);
    }

    #[test]
    fn test_parse_u32_min() {
        let mut parser = Parser::new(hex("00000000"));

        assert_eq!(parser.parse_u32(), 0);
    }

    #[test]
    fn test_parse_u32_non_min() {
        let mut parser = Parser::new(hex("0A000000"));

        assert_eq!(parser.parse_u32(), 10);
    }

    #[test]
    fn test_parse_u8_min() {
        let mut parser = Parser::new(hex("00"));

        assert_eq!(parser.parse_u8(), 0);
    }

    #[test]
    fn test_parse_u8_non_min() {
        let mut parser = Parser::new(hex("0A"));

        assert_eq!(parser.parse_u8(), 10);
    }

    #[test]
    fn test_parse_ai_interact_1() {
        // 01000000  u32        1          action
        // 1C000000  u32       28          length
        // 02        u8         2          ai_interact
        // parser starts here
        // 02        u8         2          player_id
        // 18        u8        24          unknown_u8_1
        // 00        u8         0          unknown_u8_2
        // FB360000  u32  3603200          target_id
        // FCE3C042  f32       96.445282   x
        // 2E6A9942  f32       76.7073822  y
        // 01000000  u32        1          flags
        // 01000000  u32        1          unknown_u32_1
        // C21D0000  u32     7618          unit_id
        // 98130000  u32     5016          unit_id
        let mut parser = Parser::new(hex("
              021800 FB360000 FCE3C042 2E6A9942
            01000000 01000000 C21D0000 98130000
        "));

        assert_eq!(
            parser.parse_ai_interact(),
            Operation::Action(Action::AiInteract(AiInteract {
                player_id: 2,
                unknown_u8_1: 24,
                unknown_u8_2: 0,
                target_id: 14075,
                x: 96.44528,
                y: 76.70738,
                flags: Some(vec![1, 0, 0, 0]),
                unknown_u32_1: 7618,
                unit_ids: Some(vec![5016])
            }))
        );
    }

    #[test]
    fn test_parse_ai_interact_2() {
        // 01000000  u32        1          action
        // 1C000000  u32       28          length
        // 02        u8         2          ai_interact
        // parser starts here
        // 02        u8         2          player_id
        // 18        u8        24          unknown_u8_1
        // 00        u8         0          unknown_u8_2
        // A00E0000  u32     3744          target_id
        // 00004242  f32       48.5        x
        // 00009142  f32       72.5        y
        // 01000000  u32        1          flags
        // 01000000  u32        1          unknown_u32_1
        // 993A0000  u32    15001          unit_id?
        // E6572C00  u32  2906086          unit_id?
        let mut parser = Parser::new(hex("
              021800 A00E0000 00004242 00009142
            01000000 01000000 993A0000 E6572C00
        "));

        assert_eq!(
            parser.parse_ai_interact(),
            Operation::Action(Action::AiInteract(AiInteract {
                player_id: 2,
                unknown_u8_1: 24,
                unknown_u8_2: 0,
                target_id: 3744,
                x: 48.5,
                y: 72.5,
                flags: Some(vec![1, 0, 0, 0]),
                unknown_u32_1: 15001,
                unit_ids: Some(vec![2_906_086])
            }))
        );
    }

    #[test]
    fn test_parse_interact_1() {
        // 01000000  u32        1    action
        // 1C000000  u32       28    length
        // 00        u8         2    interact
        // parser starts here
        // 02        u8         2    player_id
        // 18        u8        24    unknown_u8_1
        // 00        u8         0    unknown_u8_2
        // A9370000  u32    14249    target_id
        // 0000AA42  f32       85.0  x
        // 0000A242  f32       81.0  y
        // 0100      u16        1    selected
        // 00000100  u32        1    flags
        // FFFF      u16    65535    unknown_u16_1
        // 9C3D0000  u32    15772    unknown_u32_1
        // E30A2C00  u32  2886371    unit_ids
        let mut parser = Parser::new(hex("
              021800 A9370000 0000AA42 0000A242
            01000000 0100FFFF 9C3D0000 E30A2C00
        "));

        assert_eq!(
            parser.parse_interact(),
            Operation::Action(Action::Interact(Interact {
                player_id: 2,
                unknown_u8_1: 24,
                unknown_u8_2: 0,
                target_id: 14249,
                x: 85.0,
                y: 81.0,
                flags: None,
                unknown_u16_1: 0xffff,
                unknown_u32_1: 4_294_901_761,
                unit_ids: Some(vec![15772])
            }))
        );
    }

    #[test]
    fn test_parse_interact_2() {
        // 01000000  u32        1    action
        // 1C000000  u32       28    length
        // 00        u8         2    interact
        // parser starts here
        // 02        u8         2    player_id
        // 18        u8        24    unknown_u8_1
        // 00        u8         0    unknown_u8_2
        // BC1D0000  u32     7612    target_id
        // 0000C242  f32       97.0  x
        // 00009642  f32       75.0  y
        // 0100      u16        1    selected
        // 00000100  u32        1    flags
        // FFFF      u16    65535    unknown_u16_1
        // C71D0000  u32     7623    unknown_u32_1
        // FE280700  u32   469246    unit_ids
        let mut parser = Parser::new(hex("
              021800 BC1D0000 0000C242 00009642
            01000000 0100FFFF C71D0000 FE280700
        "));

        assert_eq!(
            parser.parse_interact(),
            Operation::Action(Action::Interact(Interact {
                player_id: 2,
                unknown_u8_1: 24,
                unknown_u8_2: 0,
                target_id: 7612,
                x: 97.0,
                y: 75.0,
                flags: None,
                unknown_u16_1: 0xffff,
                unknown_u32_1: 4_294_901_761,
                unit_ids: Some(vec![7623])
            }))
        );
    }

    #[test]
    fn test_parse_move_1() {
        // 01000000  u32        1    action
        // 1C000000  u32       28    length
        // 00        u8         2    interact
        // parser starts here
        // 02        u8         2    player_id
        // 18        u8        24    unknown_u8_1
        // 00        u8         0    unknown_u8_2
        // A9370000  u32    14249    target_id
        // 0000AA42  f32       85.0  x
        // 0000A242  f32       81.0  y
        // 0100      u16        1    selected
        // 00000100  u32        1    flags
        // FFFF      u16    65535    unknown_u16_1
        // 9C3D0000  u32    15772    unknown_u32_1
        // E30A2C00  u32  2886371    unit_ids
        let mut parser = Parser::new(hex("
              021800 A9370000 0000AA42 0000A242
            01000000 0100FFFF 9C3D0000 E30A2C00
        "));

        assert_eq!(
            parser.parse_interact(),
            Operation::Action(Action::Interact(Interact {
                player_id: 2,
                unknown_u8_1: 24,
                unknown_u8_2: 0,
                target_id: 14249,
                x: 85.0,
                y: 81.0,
                flags: None,
                unknown_u16_1: 0xffff,
                unknown_u32_1: 4_294_901_761,
                unit_ids: Some(vec![15772])
            }))
        );
    }

    // 01000000_18000000_00011400_01370000_00004242_0000B040_FFFF0000_0101FFFF_C24C0000
    #[test]
    fn test_parse_interact_3() {
        let mut parser = Parser::new(hex("
              021800 01370000 00004242 0000B040
            FFFF0000 0101FFFF C24C0000
        "));

        assert_eq!(
            parser.parse_interact(),
            Operation::Action(Action::Interact(Interact {
                player_id: 2,
                unknown_u8_1: 24,
                unknown_u8_2: 0,
                target_id: 14081,
                x: 48.5,
                y: 5.5,
                flags: Some(vec![0, 0, 1, 1]),
                unknown_u16_1: 0xffff,
                unknown_u32_1: 19650,
                unit_ids: None
            }))
        );
    }
}
