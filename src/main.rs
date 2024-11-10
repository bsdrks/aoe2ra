use {
    aoe2ra::parser::Parser,
    std::fs::read,
};

fn main() {
    let raw = read("./recs/rec5.aoe2record").unwrap();
    let mut parser = Parser::new(raw);
    let _body = parser.parse_body();
}
