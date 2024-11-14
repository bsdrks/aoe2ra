use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeAttackMove {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub flags: Option<Vec<u8>>,
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_64000000_21026000_02000000_01000101_0000B641_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_55D5D442_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_2C120000_2B120000_C6D61400
// 01000000_64000000_21026000_02000000_01000101_ABAAB441_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_ABEAD442_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_2C120000_2B120000_66D81400
// 01000000_60000000_21025C00_01000000_01000101_55557641_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_5535BA42_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_2B120000_BFDE1400
impl Parse for DeAttackMove {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let flags = parser.flags(4);
        let xs = parser.f32s(10);
        let ys = parser.f32s(10);
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            flags,
            xs,
            ys,
            unit_ids,
            unknown_u32_1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hex::hex;

    use super::{
        DeAttackMove,
        *,
    };

    #[test]
    fn test_parse_1() {
        let mut parser = Parser::new(hex("
              026800 04000000 01000101 55459D42 00000000 00000000 00000000
            00000000 00000000 00000000 00000000 00000000 00000000 AB8A1F42
            00000000 00000000 00000000 00000000 00000000 00000000 00000000
            00000000 00000000 A60A0000 CB0A0000 D30A0000 E40A0000 F88B1000
        "));

        assert_eq!(
            DeAttackMove::parse(&mut parser),
            DeAttackMove {
                player_id: 2,
                unknown_u8_1: 104,
                unknown_u8_2: 0,
                flags: Some(vec![1, 0, 1, 1]),
                xs: vec![
                    78.635_414, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                ],
                ys: vec![
                    39.885_418, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                ],
                unit_ids: vec![2726, 2763, 2771, 2788],
                unknown_u32_1: 1_084_408,
            }
        );
    }
}
