use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Interact {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub target_id: u32,
    pub x: f32,
    pub y: f32,
    pub flags: Option<Vec<u8>>,
    pub unknown_u16_1: u16,
    pub unit_ids: Option<Vec<u32>>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_1C000000_00021800_BC1D0000_0000C242_00009642_01000000_0100FFFF_C71D0000_FE280700
// 01000000_1C000000_00011800_40370000_00008F42_00002E42_01000000_0101FFFF_8F370000_9A3E0700
// 01000000_20000000_00011C00_BD360000_00008542_00009C41_02000000_0101FFFF_C51D0000_7D370000_BF780700
// 01000000_24000000_00012000_CA1D0000_5BA19242_33D72942_03000000_0101FFFF_C41D0000_54370000_5B370000_DD0C0800
// 01000000_28000000_00012400_F7360000_AFE25342_7C2DAD41_04000001_0101FFFF_C41D0000_C51D0000_C31D0000_C91D0000_5D390000
// 01000000_18000000_00011400_01370000_00004242_0000B040_FFFF0000_0101FFFF_C24C0000
impl Parse for Interact {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let target_id = parser.u32();
        let x = parser.f32();
        let y = parser.f32();
        let selected = parser.usize16_opt();
        let flags = parser.flags(4);
        let unknown_u16_1 = parser.u16();
        let unit_ids = selected.map(|selected| parser.u32s(selected));
        let unknown_u32_1 = parser.u32();

        Interact {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            target_id,
            x,
            y,
            flags,
            unknown_u16_1,
            unit_ids,
            unknown_u32_1,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            hex::hex,
            parser::Parser,
        },
    };

    #[test]
    fn test_parse_interact_1() {
        let mut parser = Parser::new(hex("
              021800 A9370000 0000AA42 0000A242
            01000000 0100FFFF 9C3D0000 E30A2C00
        "));

        assert_eq!(
            Interact::parse(&mut parser),
            Interact {
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
            }
        );
    }

    #[test]
    fn test_parse_interact_2() {
        let mut parser = Parser::new(hex("
              021800 BC1D0000 0000C242 00009642
            01000000 0100FFFF C71D0000 FE280700
        "));

        assert_eq!(
            Interact::parse(&mut parser),
            Interact {
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
            }
        );
    }

    #[test]
    fn test_parse_interact_3() {
        let mut parser = Parser::new(hex("
              021800 01370000 00004242 0000B040
            FFFF0000 0101FFFF C24C0000
        "));

        assert_eq!(
            Interact::parse(&mut parser),
            Interact {
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
            }
        );
    }
}
