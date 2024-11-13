use crate::{
    parser::{
        Parse,
        Parser,
    },
    r#enum::OrderType,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Order {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub unknown_u32_2: Option<u32>,
    pub order_type: OrderType,
    pub unknown_u8_3: u8,
    pub unknown_u8_4: u8,
    pub unknown_u8_5: u8,
    pub unknown_u8_6: u8,
    pub unknown_u8_7: u8,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_3: u32,
}

// Examples:
// 01000000_3D000000_75023900_07000000_FFFFFFFF_000080BF_000080BF_FFFFFFFF_00000000_07000001_00481D00_00491D00_004A1D00_00243800_002E3800_00283800_003B3800_00C88D03_00
// 01000000_3D000000_75013900_07000000_FFFFFFFF_000080BF_000080BF_FFFFFFFF_00000000_08000001_00690B00_00740D00_007F0D00_00780D00_00610B00_007A0D00_00630B00_008CD001_00
impl Parse for Order {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let unknown_u32_1 = parser.u32_opt();
        let x = parser.f32();
        let y = parser.f32();
        let unknown_u32_2 = parser.u32_opt();
        let order_type = OrderType::parse(parser);
        let unknown_u8_3 = parser.u8();
        let unknown_u8_4 = parser.u8();
        let unknown_u8_5 = parser.u8();
        let unknown_u8_6 = parser.u8();
        let unknown_u8_7 = parser.u8();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_3 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            x,
            y,
            unknown_u32_2,
            order_type,
            unknown_u8_3,
            unknown_u8_4,
            unknown_u8_5,
            unknown_u8_6,
            unknown_u8_7,
            unit_ids,
            unknown_u32_3,
        }
    }
}
