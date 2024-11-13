pub trait Parse {
    fn parse(parser: &mut Parser) -> Self
    where
        Self: Sized;
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

pub struct Parser {
    pub cursor: usize,
    pub raw: Vec<u8>,
}

impl Parser {
    #[must_use]
    pub const fn new(raw: Vec<u8>) -> Self {
        Self { cursor: 0, raw }
    }

    pub fn skip(&mut self, count: usize) {
        self.cursor += count;
    }

    pub fn bool32(&mut self) -> bool {
        match self.u32() {
            0x00 => false,
            0x01 => true,
            n => panic!("0x00 or 0x01, found: {n}"),
        }
    }

    pub fn bool8(&mut self) -> bool {
        match self.u8() {
            0x00 => false,
            0x01 => true,
            n => panic!("0x00 or 0x01, found: {n}"),
        }
    }

    pub fn f32(&mut self) -> f32 {
        let value = f32::from_le_bytes(self.take_4());

        self.skip(4);

        value
    }

    pub fn f32s(&mut self, count: usize) -> Vec<f32> {
        (0..count).map(|_| self.f32()).collect()
    }

    pub fn flags(&mut self, count: usize) -> Option<Vec<u8>> {
        self.peek_u8s(count)
            .iter()
            .all(|&byte| byte == 0 || byte == 1)
            .then(|| self.u8s(count))
    }

    pub fn u16(&mut self) -> u16 {
        let value = u16::from_le_bytes(self.take_2());

        self.skip(2);

        value
    }

    pub fn u16_opt(&mut self) -> Option<u16> {
        match self.u16() {
            0xffff => None,
            value => Some(value),
        }
    }

    pub fn u32(&mut self) -> u32 {
        let value = self.peek_u32();

        self.skip(4);

        value
    }

    pub fn u32_opt(&mut self) -> Option<u32> {
        match self.u32() {
            0xffff_ffff => None,
            n => Some(n),
        }
    }

    pub fn u32s(&mut self, count: usize) -> Vec<u32> {
        (0..count).map(|_| self.u32()).collect()
    }

    pub fn u8(&mut self) -> u8 {
        let value = self.raw[self.cursor];

        self.skip(1);

        value
    }

    pub fn u8_opt(&mut self) -> Option<u8> {
        match self.u8() {
            0xff => None,
            value => Some(value),
        }
    }

    pub fn u8s(&mut self, count: usize) -> Vec<u8> {
        let cursor_next = self.cursor + count;
        let value = self.raw[self.cursor..cursor_next].to_vec();

        self.set_cursor(cursor_next);

        value
    }

    pub fn usize16_opt(&mut self) -> Option<usize> {
        self.u16_opt().map(|value| value as usize)
    }

    pub fn usize32(&mut self) -> usize {
        self.u32() as usize
    }

    pub fn usize32_opt(&mut self) -> Option<usize> {
        self.u32_opt().map(|value| value as usize)
    }

    pub fn usize8(&mut self) -> usize {
        self.u8() as usize
    }

    pub fn usize8_opt(&mut self) -> Option<usize> {
        self.u8_opt().map(|value| value as usize)
    }

    fn take_2(&self) -> [u8; 2] {
        [self.raw[self.cursor], self.raw[self.cursor + 1]]
    }

    fn take_4(&self) -> [u8; 4] {
        [
            self.raw[self.cursor],
            self.raw[self.cursor + 1],
            self.raw[self.cursor + 2],
            self.raw[self.cursor + 3],
        ]
    }

    pub fn peek_bool_u32(&self) -> bool {
        self.peek_u32() != 0
    }

    pub fn peek_u32(&self) -> u32 {
        u32::from_le_bytes(self.take_4())
    }

    pub fn peek_u8s(&self, count: usize) -> Vec<u8> {
        self.raw[self.cursor..self.cursor + count].to_vec()
    }

    pub fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use {
        super::*,
        crate::hex::hex,
    };

    #[test]
    fn test_parse_bool_u8_false() {
        let mut parser = Parser::new(hex("00"));

        assert!(!parser.bool8());
    }

    #[test]
    fn test_parse_bool_u8_true() {
        let mut parser = Parser::new(hex("01"));

        assert!(parser.bool8());
    }

    #[test]
    #[should_panic(expected = "0x00 or 0x01, found: 2")]
    fn test_parse_bool_u8_panic() {
        let mut parser = Parser::new(hex("02"));
        let _ = parser.bool8();
    }

    #[test]
    fn test_parse_f32_0xffff_ffff() {
        let mut parser = Parser::new(hex("FFFFFFFF"));

        assert!(parser.f32().is_nan());
    }

    #[test]
    fn test_parse_f32s() {
        let mut parser = Parser::new(hex("00004841 00004849 0000473C"));

        assert_eq!(parser.f32s(3), vec![12.5, 819_200.0, 0.012_145_996]);
    }

    #[test]
    fn test_parse_f32_min() {
        let mut parser = Parser::new(hex("00000000"));

        assert_eq!(parser.f32(), 0.0);
    }

    #[test]
    fn test_parse_f32_non_min() {
        let mut parser = Parser::new(hex("00004841"));

        assert_eq!(parser.f32(), 12.5);
    }

    #[test]
    fn test_parse_u16_min() {
        let mut parser = Parser::new(hex("0000"));

        assert_eq!(parser.u16(), 0);
    }

    #[test]
    fn test_parse_u16_non_min() {
        let mut parser = Parser::new(hex("0A00"));

        assert_eq!(parser.u16(), 10);
    }

    #[test]
    fn test_parse_u32_min() {
        let mut parser = Parser::new(hex("00000000"));

        assert_eq!(parser.u32(), 0);
    }

    #[test]
    fn test_parse_u32_non_min() {
        let mut parser = Parser::new(hex("0A000000"));

        assert_eq!(parser.u32(), 10);
    }

    #[test]
    fn test_parse_u8_min() {
        let mut parser = Parser::new(hex("00"));

        assert_eq!(parser.u8(), 0);
    }

    #[test]
    fn test_parse_u8_non_min() {
        let mut parser = Parser::new(hex("0A"));

        assert_eq!(parser.u8(), 10);
    }
}
