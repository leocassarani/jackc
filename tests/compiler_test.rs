use jackc::jack::{Compiler, Parser, Tokenizer};
use jackc::vm::{Command, Segment};
use std::path::PathBuf;
use std::{env, fs};

fn read_test_file(filename: &str) -> String {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let path = dir.join("tests").join("testdata").join(filename);
    fs::read_to_string(path).expect("couldn't load test file")
}

#[test]
fn seven_jack_test() {
    let source = read_test_file("Seven.jack");
    let tokenizer = Tokenizer::new(&source);
    let class = Parser::new(tokenizer).parse().expect("parsing error");
    let compiler = Compiler::new(class);

    assert_eq!(
        compiler.compile(),
        vec![
            Command::Function("main.main".into(), 0),
            Command::Push(Segment::Constant, 1),
            Command::Push(Segment::Constant, 2),
            Command::Push(Segment::Constant, 3),
            Command::Call("math.multiply".into(), 2),
            Command::Add,
            Command::Call("output.printint".into(), 1),
            Command::Pop(Segment::Temp, 0),
            Command::Push(Segment::Constant, 0),
            Command::Return,
        ]
    );
}
