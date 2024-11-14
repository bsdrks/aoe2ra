use {
    aoe2ra::{
        body::Body,
        operation::{
            Action,
            Operation,
        },
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

    println!("{:?}", body.operations.len());

    let actions = body
        .operations
        .iter()
        .filter(|operation| {
            matches!(operation, Operation::Action(Action::Build(_)))
        })
        .collect::<Vec<_>>();

    for action in actions {
        println!("{:?}", action);
    }
}
