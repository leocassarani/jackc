use jackc::jack::{Compiler, Parser, Tokenizer};
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let filename = env::args().skip(1).next().expect("missing filename");
    let source = fs::read_to_string(filename)?;

    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer);
    let class = parser.parse().expect("parsing error");

    let mut compiler = Compiler::new();
    for cmd in compiler.compile(&class) {
        println!("{}", cmd);
    }

    Ok(())
}
