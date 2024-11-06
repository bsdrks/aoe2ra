use {
    aoe2ra::parser::Parser,
    std::fs::read,
};

fn main() {
    let raw = read("./recs/rec9.aoe2record").unwrap();
    let mut parser = Parser::new(raw);
    let _operations = parser.parse().unwrap();
}
