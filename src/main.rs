use std::{env, fs, io};

use jackc::jack::Tokenizer;

fn main() -> io::Result<()> {
    let filename = env::args().skip(1).next().expect("missing filename");
    let source = fs::read_to_string(filename)?;
    let tokenizer = Tokenizer::new(&source);

    for token in tokenizer {
        println!("{:?}", token);
    }

    Ok(())
}
