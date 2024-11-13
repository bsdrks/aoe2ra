use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Research {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u32_1: u32,
    pub unknown_u16_1: u16,
    pub unknown_u16_2: u16,
    pub unknown_u32_2: Option<u32>,
    pub unknown_u8_2: u8,
    pub unit_ids: Vec<u32>,
}

// Examples:
// 01000000_15000000_65011100_8E380000_0100C800_FFFFFFFF_008E3800_00085617_00
// 01000000_19000000_65011500_2B390000_02006400_FFFFFFFF_00993800_002B3900_0061A516_00
// 01000000_11000000_65020D00_1F3A0000_01006200_FFFFFFFF_001BC11B_00
impl Parse for Research {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let selected = (parser.usize8() - 8) / 4;
        let unknown_u8_1 = parser.u8();
        let unknown_u32_1 = parser.u32();
        let unknown_u16_1 = parser.u16();
        let unknown_u16_2 = parser.u16();
        let unknown_u32_2 = parser.u32_opt();
        let unknown_u8_2 = parser.u8();
        let unit_ids = parser.u32s(selected);

        Self {
            player_id,
            unknown_u8_1,
            unknown_u32_1,
            unknown_u16_1,
            unknown_u16_2,
            unknown_u32_2,
            unknown_u8_2,
            unit_ids,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::Research,
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
              011100 8E380000 0100C800 FFFFFFFF
            008E3800 00085617 00
        "));

        assert_eq!(
            Research::parse(&mut parser),
            Research {
                player_id: 1,
                unknown_u8_1: 0,
                unknown_u32_1: 14478,
                unknown_u16_1: 1,
                unknown_u16_2: 200,
                unknown_u32_2: None,
                unknown_u8_2: 0,
                unit_ids: vec![14478, 1_529_352],
            }
        );
    }
}
