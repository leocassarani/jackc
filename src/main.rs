use byteorder::{BigEndian, WriteBytesExt};
use clap::{crate_version, App, Arg};
use jackc::asm::{self, Instruction};
use jackc::jack::{self, Compiler, Tokenizer};
use jackc::vm::{self, Module, Translator};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Format {
    Asm,
    Bin,
    Hack,
}

impl Format {
    fn extension(self) -> &'static str {
        match self {
            Format::Asm => "asm",
            Format::Bin => "bin",
            Format::Hack => "hack",
        }
    }
}

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
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Writes the output to <file>")
                .takes_value(true)
                .conflicts_with("stdout"),
        )
        .arg(
            Arg::with_name("stdout")
                .long("stdout")
                .help("Writes the output to stdout")
                .conflicts_with("output"),
        )
        .get_matches();

    let path = matches
        .value_of("file")
        .map(Path::new)
        .unwrap()
        .canonicalize()?;

    let format = if matches.is_present("asm") {
        Format::Asm
    } else if matches.is_present("bin") {
        Format::Bin
    } else {
        Format::Hack
    };

    let modules = if path.is_dir() {
        compile_dir(&path)?
    } else {
        vec![compile_file(&path).expect("unexpected file extension")?]
    };

    let insts = Translator::new(&modules).translate();

    if matches.is_present("stdout") {
        let stdout = io::stdout();
        let mut out = stdout.lock();
        write_output(&mut out, &insts, format)
    } else {
        let filename = matches
            .value_of("output")
            .map(|s| s.to_owned())
            .unwrap_or_else(|| {
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
                format!("{}.{}", stem, format.extension())
            });

        let mut file = File::create(filename)?;
        write_output(&mut file, &insts, format)
    }
}

fn compile_dir(dir: &Path) -> io::Result<Vec<Module>> {
    let mut mods = Vec::new();

    for entry in dir.read_dir()? {
        if let Some(module) = compile_file(&entry?.path()) {
            mods.push(module?);
        }
    }

    // Sort modules by name to ensure reproducible builds.
    mods.sort_by(|a, b| a.name.cmp(&b.name));

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

fn write_output<W>(out: &mut W, insts: &[Instruction], format: Format) -> io::Result<()>
where
    W: Write,
{
    match format {
        Format::Asm => {
            for inst in insts {
                writeln!(out, "{}", inst)?;
            }
        }
        Format::Bin => {
            for inst in asm::assemble(insts) {
                out.write_u16::<BigEndian>(inst)?;
            }
        }
        Format::Hack => {
            for inst in asm::assemble(insts) {
                writeln!(out, "{:016b}", inst)?;
            }
        }
    };

    Ok(())
}
