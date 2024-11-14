use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttackGround {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub x: f32,
    pub y: f32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_20000000_6B051C00_03000000_5515A742_ABEA2C43_00000100_433D0000_923F0000_C7440000_BCBA1600
// 01000000_18000000_6B021400_01000000_0040AE42_ABAA4542_00000100_68130000_E0641800
// 01000000_18000000_6B021400_01000000_00D0A942_00205E42_00000100_68130000_E8841900
// 01000000_18000000_6B021400_01000000_AB2AB042_55D55A42_00000100_68130000_81CF1900
// 01000000_18000000_6B021400_01000000_0080A042_55D58042_00000100_68130000_2BB71A00
// 01000000_18000000_6B021400_01000000_5525AF42_ABCA5C42_00000100_68130000_CCCF1A00
impl Parse for AttackGround {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let x = parser.f32();
        let y = parser.f32();
        let flags = parser.flags(4);
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            x,
            y,
            flags,
            unit_ids,
            unknown_u32_1,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{
            AttackGround,
            *,
        },
        crate::hex::hex,
    };

    #[test]
    fn test_parse_1() {
        let mut parser = Parser::new(hex("
            021400 01000000 0040AE42 ABAA4542 00000100 68130000 E0641800
        "));

        assert_eq!(
            AttackGround::parse(&mut parser),
            AttackGround {
                player_id: 2,
                unknown_u8_1: 20,
                unknown_u8_2: 0,
                x: 87.125,
                y: 49.416_668,
                flags: Some(vec![0, 0, 1, 0]),
                unit_ids: vec![4968],
                unknown_u32_1: 1_598_688
            }
        );
    }

    #[test]
    fn test_parse_2() {
        let mut parser = Parser::new(hex("
            021400 01000000 00D0A942 00205E42 00000100 68130000 E8841900
        "));

        assert_eq!(
            AttackGround::parse(&mut parser),
            AttackGround {
                player_id: 2,
                unknown_u8_1: 20,
                unknown_u8_2: 0,
                x: 84.90625,
                y: 55.53125,
                flags: Some(vec![0, 0, 1, 0]),
                unit_ids: vec![4968],
                unknown_u32_1: 1_672_424
            }
        );
    }

    #[test]
    fn test_parse_3() {
        let mut parser = Parser::new(hex("
            021400 01000000 AB2AB042 55D55A42 00000100 68130000 81CF1900
        "));

        assert_eq!(
            AttackGround::parse(&mut parser),
            AttackGround {
                player_id: 2,
                unknown_u8_1: 20,
                unknown_u8_2: 0,
                x: 88.083_336,
                y: 54.708_332,
                flags: Some(vec![0, 0, 1, 0]),
                unit_ids: vec![4968],
                unknown_u32_1: 1_691_521
            }
        );
    }

    #[test]
    fn test_parse_4() {
        let mut parser = Parser::new(hex("
            021400 01000000 0080A042 55D58042 00000100 68130000 2BB71A00
        "));

        assert_eq!(
            AttackGround::parse(&mut parser),
            AttackGround {
                player_id: 2,
                unknown_u8_1: 20,
                unknown_u8_2: 0,
                x: 80.25,
                y: 64.416_664,
                flags: Some(vec![0, 0, 1, 0]),
                unit_ids: vec![4968],
                unknown_u32_1: 1_750_827
            }
        );
    }

    #[test]
    fn test_parse_5() {
        let mut parser = Parser::new(hex("
            021400 01000000 5525AF42 ABCA5C42 00000100 68130000 CCCF1A00
        "));

        assert_eq!(
            AttackGround::parse(&mut parser),
            AttackGround {
                player_id: 2,
                unknown_u8_1: 20,
                unknown_u8_2: 0,
                x: 87.572_914,
                y: 55.197_918,
                flags: Some(vec![0, 0, 1, 0]),
                unit_ids: vec![4968],
                unknown_u32_1: 1_757_132
            }
        );
    }
}
