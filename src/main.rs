use jackc::asm;
use jackc::jack::{self, Compiler, Tokenizer};
use jackc::vm::{self, Module, Translator};
use std::{
    env, fs,
    io::{self, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    let filename = env::args().skip(1).next().expect("missing filename");
    let path = Path::new(&filename);

    let modules: Vec<_> = if path.is_dir() {
        compile_dir(path)?
    } else {
        let module = compile_file(path).expect("unexpected file extension")?;
        vec![module]
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let insts = Translator::new(&modules).translate();
    for inst in asm::assemble(insts) {
        writeln!(out, "{:016b}", inst)?;
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
