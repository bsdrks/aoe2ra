use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Action0x82 {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u8_3: u8,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000 11000000 82020D00 02000000 01C73700 00C83700 002DF60A 00
// 01000000_0D000000_82020900_01000000_00BC1D00_00859C02_00
// 01000000_0D000000_82020900_01000000_00BC1D00_007A0A00_00
impl Parse for Action0x82 {
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
    fn test_action_0x82() {
        let mut parser = Parser::new(hex("
            020D00 02000000 01C73700 00C83700 002DF60A 00
        "));

        assert_eq!(
            Action0x82::parse(&mut parser),
            Action0x82 {
                player_id: 2,
                unknown_u8_1: 13,
                unknown_u8_2: 0,
                unknown_u8_3: 1,
                unit_ids: vec![14279, 14280],
                unknown_u32_1: 718_381,
            }
        );
    }
}
