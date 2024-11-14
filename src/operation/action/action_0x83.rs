use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Action0x83 {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u8_3: u8,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_0D000000_83020900_01000000_02573700_0016A400_00
// 01000000_0D000000_83020900_01000000_02573700_00F0A300_00
impl Parse for Action0x83 {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let unknown_u8_3 = parser.u8();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u8_3,
            unit_ids,
            unknown_u32_1,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::hex::hex,
    };

    #[test]
    fn test_action_0x83() {
        let mut parser = Parser::new(hex("
            020900 01000000 02573700 0016A400 00
        "));

        assert_eq!(
            Action0x83::parse(&mut parser),
            Action0x83 {
                player_id: 2,
                unknown_u8_1: 9,
                unknown_u8_2: 0,
                unknown_u8_3: 2,
                unit_ids: vec![14167],
                unknown_u32_1: 42006
            },
        );
    }
}
