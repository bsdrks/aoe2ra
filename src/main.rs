use {
    aoe2ra::{
        body::Body,
        parser::{
            Parse,
            Parser,
        },
    },
    std::{
        env::args,
        fs::read,
    },
};

fn main() {
    let record_path = args().nth(1).expect("No record path provided");
    let raw = read(record_path).expect("Failed to read record");
    let mut parser = Parser::new(raw);
    let body = Body::parse(&mut parser);

    println!("{:?}", body);
}
