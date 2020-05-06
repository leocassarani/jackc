use byteorder::{BigEndian, WriteBytesExt};
use clap::{crate_version, App, Arg};
use jackc::asm;
use jackc::jack::{self, Compiler, Tokenizer};
use jackc::vm::{self, Module, Translator};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    let matches = App::new("jackc")
        .version(crate_version!())
        .about("A compiler for the Jack programming language")
        .arg(
            Arg::with_name("file")
                .help("File or directory to be compiled")
                .required(true),
        )
        .arg(
            Arg::with_name("asm")
                .long("asm")
                .help("Outputs an assembly file")
                .conflicts_with_all(&["bin", "hack"]),
        )
        .arg(
            Arg::with_name("bin")
                .long("bin")
                .help("Outputs a binary file")
                .conflicts_with_all(&["asm", "hack"]),
        )
        .arg(
            Arg::with_name("hack")
                .long("hack")
                .help("Outputs a Hack file")
                .conflicts_with_all(&["asm", "bin"]),
        )
        .get_matches();

    let path = matches.value_of("file").map(Path::new).unwrap();

    let modules = if path.is_dir() {
        compile_dir(path)?
    } else {
        vec![compile_file(path).expect("unexpected file extension")?]
    };

    let insts = Translator::new(&modules).translate();

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if matches.is_present("asm") {
        for inst in insts {
            writeln!(out, "{}", inst)?;
        }
    } else if matches.is_present("bin") {
        for inst in asm::assemble(insts) {
            out.write_u16::<BigEndian>(inst)?;
        }
    } else {
        for inst in asm::assemble(insts) {
            writeln!(out, "{:016b}", inst)?;
        }
    }

    Ok(())
}

fn compile_dir(dir: &Path) -> io::Result<Vec<Module>> {
    let mut mods = Vec::new();

    for entry in dir.read_dir()? {
        if let Some(module) = compile_file(&entry?.path()) {
            mods.push(module?);
        }
    }

    Ok(mods)
}

fn compile_file(path: &Path) -> Option<io::Result<Module>> {
    path.extension().and_then(|ext| {
        if ext == "jack" {
            Some(compile_jack(path))
        } else if ext == "vm" {
            Some(compile_vm(path))
        } else {
            None
        }
    })
}

fn compile_jack(path: &Path) -> io::Result<Module> {
    let source = fs::read_to_string(path)?;
    let tokenizer = Tokenizer::new(&source);

    let mut parser = jack::Parser::new(tokenizer);
    let class = parser.parse().expect("parsing error");

    let mut compiler = Compiler::new(&class);
    Ok(compiler.compile())
}

fn compile_vm(path: &Path) -> io::Result<Module> {
    let name = path
        .file_stem()
        .expect("invalid file name")
        .to_string_lossy()
        .into();

    let source = fs::read_to_string(path)?;
    let cmds = vm::parse(&source).expect("parsing error");

    Ok(Module::new(name, cmds))
}
