mod parser;

use athenisa_isa::encoding::encode_program;
use athenisa_isa::format::format_instruction;
use athenisa_isa::image::{words_to_bytes, words_to_hex};
use athenisa_isa::instruction::Instruction;

use anyhow::{Result, bail};
use clap::Parser;

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(version, about = "Assembler for the AthenISA instruction set")]
struct Args {
    /// AthenISA source file to assemble
    input: String,

    /// Base path for generated output files
    #[arg(short, long)]
    output: Option<String>,

    /// Generate a hexadecimal memory image
    #[arg(long)]
    hex: bool,

    /// Generate a symbol map
    #[arg(long)]
    sym: bool,

    /// Generate an instruction listing
    #[arg(long)]
    debug: bool,

    /// Do not generate the default binary file
    #[arg(long)]
    no_bin: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.no_bin && !args.hex && !args.sym && !args.debug {
        bail!("no output selected: use --hex, --sym, --debug, or remove --no-bin");
    }

    let source = fs::read_to_string(&args.input)?;
    let program = parser::parse_source(&source)?;
    let words = encode_program(&program.instructions);
    let output = match &args.output {
        Some(output) => PathBuf::from(output),
        None => default_output_path(&args.input),
    };

    if !args.no_bin {
        fs::write(output_path(&output, "bin"), words_to_bytes(&words))?;
    }

    if args.hex {
        fs::write(output_path(&output, "hex"), words_to_hex(&words))?;
    }

    if args.sym {
        fs::write(
            output_path(&output, "sym"),
            format_symbols(&program.symbols),
        )?;
    }

    if args.debug {
        fs::write(
            output_path(&output, "lst"),
            format_debug(&program.instructions, &words),
        )?;
    }

    Ok(())
}

fn default_output_path(input: &str) -> PathBuf {
    let mut output = PathBuf::from(input);
    output.set_extension("");

    output
}

fn output_path(output: &Path, extension: &str) -> PathBuf {
    let mut path = output.to_path_buf();
    path.set_extension(extension);

    path
}

fn format_symbols(symbols: &parser::Symbols) -> String {
    let name_width = symbols
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0);

    let mut text = String::new();

    for (name, value) in symbols {
        text.push_str(&format!("{:<width$} {}\n", name, value, width = name_width));
    }

    text
}

fn format_debug(instructions: &[Instruction], words: &[u16]) -> String {
    let mut text = String::new();

    for (pc, (instruction, word)) in instructions.iter().zip(words.iter()).enumerate() {
        text.push_str(&format!(
            "{pc:04X}  {word:04X}  {}\n",
            format_instruction(*instruction)
        ));
    }

    text
}
