use jackc::asm;
use jackc::jack::{Compiler, Parser, Tokenizer};
use jackc::vm::Translator;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let filename = env::args().skip(1).next().expect("missing filename");
    let source = fs::read_to_string(filename)?;

    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer);
    let class = parser.parse().expect("parsing error");

    let mut compiler = Compiler::new(&class);
    let modules = &[compiler.compile()];

    let mut translator = Translator::new(modules);
    for instr in asm::assemble(translator.translate()) {
        println!("{}", instr);
    }

    Ok(())
}
