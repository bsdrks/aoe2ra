use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Stop {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_20000000_01011C00_06000000_B30E0000_410F0000_7F0F0000_890F0000_9D0F0000_05100000_E8CD0E00
// 01000000_0C000000_01020800_01000000_C81D0000_16A60000
impl Parse for Stop {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
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
    fn test_stop() {
        let mut parser = Parser::new(hex("
            020800 01000000 C81D0000 16A60000
        "));

        assert_eq!(
            Stop::parse(&mut parser),
            Stop {
                player_id: 2,
                unknown_u8_1: 8,
                unknown_u8_2: 0,
                unit_ids: vec![7624],
                unknown_u32_1: 42518
            }
        );
    }
}
