use byteorder::{BigEndian, WriteBytesExt};
use clap::{crate_version, App, Arg};
use failure::{err_msg, Error};
use jackc::asm::{self, Instruction};
use jackc::jack::{self, Compiler, Tokenizer};
use jackc::vm::{self, Module, Translator};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process,
};

type Result<T> = std::result::Result<T, Error>;

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

fn main() {
    process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Error: {}", err);
            1
        }
    });
}

fn run() -> Result<()> {
    let matches = App::new("jackc")
        .version(crate_version!())
        .about("A compiler for the Jack programming language")
        .arg(
            Arg::with_name("file")
                .help("Files or directories to be compiled")
                .multiple(true)
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
            Arg::with_name("init")
                .long("init")
                .help("Name of the program's entry point (default: Sys.init)")
                .takes_value(true)
                .conflicts_with("no-init"),
        )
        .arg(
            Arg::with_name("no-init")
                .long("no-init")
                .help("Program execution does not start from an init function")
                .conflicts_with("init"),
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

    let paths = matches
        .values_of("file")
        .unwrap()
        .map(|p| Path::new(p).canonicalize())
        .collect::<io::Result<Vec<_>>>()?;

    let mut modules = Vec::new();

    for path in &paths {
        if path.is_dir() {
            modules.extend(compile_dir(&path)?);
        } else {
            modules.push(
                compile_file(&path)
                    .unwrap_or_else(|| Err(err_msg("unsupported file extension")))?,
            );
        }
    }

    if modules.is_empty() {
        return Err(err_msg("missing input files"));
    }

    let mut translator = Translator::new(&modules);

    if let Some(init) = matches.value_of("init") {
        translator.init(Some(init.to_owned()));
    } else if matches.is_present("no-init") {
        translator.init(None);
    }

    let insts = translator.translate()?;

    let format = if matches.is_present("asm") {
        Format::Asm
    } else if matches.is_present("bin") {
        Format::Bin
    } else {
        Format::Hack
    };

    if matches.is_present("stdout") {
        let stdout = io::stdout();
        let mut out = stdout.lock();
        write_output(&mut out, &insts, format)
    } else {
        let filename = matches
            .value_of("output")
            .map(|s| s.to_owned())
            .unwrap_or_else(|| {
                // Use the stem of the first input path as the default output filename.
                let stem = paths
                    .get(0)
                    .and_then(|p| p.file_stem())
                    .and_then(|s| s.to_str())
                    .unwrap_or("out");

                format!("{}.{}", stem, format.extension())
            });

        let mut file = File::create(filename)?;
        write_output(&mut file, &insts, format)
    }
}

fn compile_dir(dir: &Path) -> Result<Vec<Module>> {
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

fn compile_file(path: &Path) -> Option<Result<Module>> {
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

fn compile_jack(path: &Path) -> Result<Module> {
    let source = fs::read_to_string(path)?;

    let tokens = Tokenizer::new(&source).tokenize()?;
    let class = jack::Parser::new(tokens).parse()?;

    Compiler::new(&class).compile()
}

fn compile_vm(path: &Path) -> Result<Module> {
    let name = path
        .file_stem()
        .ok_or_else(|| err_msg("invalid file name"))?
        .to_string_lossy()
        .into();

    let source = fs::read_to_string(path)?;
    let cmds = vm::parse(&source)?;

    Ok(Module::new(name, cmds))
}

fn write_output<W>(out: &mut W, insts: &[Instruction], format: Format) -> Result<()>
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
            for inst in asm::assemble(insts)? {
                out.write_u16::<BigEndian>(inst)?;
            }
        }
        Format::Hack => {
            for inst in asm::assemble(insts)? {
                writeln!(out, "{:016b}", inst)?;
            }
        }
    };

    Ok(())
}
