use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Build {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub x: f32,
    pub y: f32,
    pub building_type: u32,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
    pub sprite_id: u32,
    pub unit_ids: Vec<u32>,
}

// Examples:
// 01000000_28000000_66012400_02000000_00005842_00007041_46000000_FFFFFFFF_FFFFFFFF_00000001_C41D0000_C51D0000_C9150000
// 01000000_24000000_66022000_01000000_0000C442_00008A42_46000000_00000000_FFFFFFFF_00000001_C81D0000_49280000
// 01000000_24000000_66022000_01000000_00003042_0000D242_46000000_FFFFFFFF_FFFFFFFF_02000001_090C0000_18060000
// 01000000_38000000_66023400_06000000_00004442_0000AE42_46000000_FFFFFFFF_FFFFFFFF_00000001_39100000_990E0000_880E0000_69130000_CF0E0000_30120000_0B951A00
impl Parse for Build {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let x = parser.f32();
        let y = parser.f32();
        let building_type = parser.u32();
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32();
        let unknown_u32_3 = parser.u32();
        let sprite_id = parser.u32();
        let unit_ids = parser.u32s(selected);

        Self {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{
            Build,
            *,
        },
        crate::hex::hex,
    };

    #[test]
    fn test_parse_1() {
        let mut parser = Parser::new(hex("
              022000 01000000 00003042 0000D242 46000000 FFFFFFFF FFFFFFFF
            02000001 090C0000 18060000
        "));

        assert_eq!(
            Build::parse(&mut parser),
            Build {
                player_id: 2,
                unknown_u8_1: 32,
                unknown_u8_2: 0,
                x: 44.0,
                y: 105.0,
                building_type: 70,
                unknown_u32_1: 4_294_967_295,
                unknown_u32_2: 4_294_967_295,
                unknown_u32_3: 16_777_218,
                sprite_id: 3081,
                unit_ids: vec![1560]
            }
        );
    }

    #[test]
    fn test_parse_2() {
        let mut parser = Parser::new(hex("
              022000 01000000 00004442 0000AE42 46000000
            FFFFFFFF FFFFFFFF 00000001 39100000 990E0000
            880E0000 69130000 CF0E0000 30120000 0B951A00
        "));

        assert_eq!(
            Build::parse(&mut parser),
            Build {
                player_id: 2,
                unknown_u8_1: 52,
                unknown_u8_2: 0,
                x: 49.0,
                y: 87.0,
                building_type: 70,
                unknown_u32_1: 4_294_967_295,
                unknown_u32_2: 4_294_967_295,
                unknown_u32_3: 16_777_216,
                sprite_id: 4153,
                unit_ids: vec![3737, 3720, 4969, 3791, 4656, 1_742_091]
            }
        );
    }
}
