use std::{fs, io};

mod tokenizer;

use tokenizer::Tokenizer;

fn main() -> io::Result<()> {
    let source = fs::read_to_string("ArrayTest.jack")?;
    let tokenizer = Tokenizer::new(&source);

    for token in tokenizer {
        println!("{:?}", token);
    }

    Ok(())
}
