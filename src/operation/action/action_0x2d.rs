use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Action0x2d {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub unknown_u8_3: u8,
    pub unknown_u8_4: u8,
    pub unknown_u8_5: u8,
    pub unknown_u32_2: u32,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_3: u32,
}

// Examples:
// 01000000_1C000000_2D011800_FFFFFFFF_55858D42_00309842_01000001_01000000_130E0000_F7A61600
// 01000000_24000000_2D022000_FFFFFFFF_AB8A6442_00B0BC42_03000001_01000000_630A0000_FD100000_01110000_0CB42100
impl Parse for Action0x2d {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32_opt();
        let x = parser.f32();
        let y = parser.f32();
        let selected = parser.usize8();
        let unknown_u8_3 = parser.u8();
        let unknown_u8_4 = parser.u8();
        let unknown_u8_5 = parser.u8();
        let unknown_u32_2 = parser.u32();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_3 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            x,
            y,
            unknown_u8_3,
            unknown_u8_4,
            unknown_u8_5,
            unknown_u32_2,
            unit_ids,
            unknown_u32_3,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::Action0x2d,
        crate::{
            hex::hex,
            parser::{
                Parse,
                Parser,
            },
        },
    };

    #[test]
    fn test_parse_1() {
        let mut parser = Parser::new(hex("
              011800 FFFFFFFF 55858D42 00309842
            01000010 01000000 130E0000 F7A61600
        "));

        assert_eq!(
            Action0x2d::parse(&mut parser),
            Action0x2d {
                player_id: 1,
                unknown_u8_1: 24,
                unknown_u8_2: 0,
                unknown_u32_1: None,
                x: 70.760_414,
                y: 76.09375,
                unknown_u8_3: 0,
                unknown_u8_4: 0,
                unknown_u8_5: 16,
                unknown_u32_2: 1,
                unit_ids: vec![3603],
                unknown_u32_3: 1_484_535,
            }
        );
    }
}
