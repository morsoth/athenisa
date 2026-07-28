use athenisa_isa::encoding::decode_instruction;
use athenisa_isa::format::format_instruction;
use athenisa_isa::image::{bytes_to_words, hex_to_words};

use anyhow::{Context, Result, bail};
use clap::Parser;

use std::fs;
use std::io::{self, Write};
use std::path::Path;

const INSTR_MEM_SIZE: usize = 2048;

#[derive(Parser)]
#[command(version, about = "Disassembler for the AthenISA instruction set")]
struct Args {
    /// Raw binary or hexadecimal memory image to disassemble
    input: String,

    /// Write the generated assembly to a file instead of standard output
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let words = read_words(&args.input)?;
    let source = disassemble(&words)?;

    match &args.output {
        Some(output) => fs::write(output, source)?,
        None => io::stdout().write_all(source.as_bytes())?,
    }

    Ok(())
}

fn read_words(input: &str) -> Result<Vec<u16>> {
    let path = Path::new(input);
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase());

    match extension.as_deref() {
        Some("bin") => {
            let bytes = fs::read(path)?;
            bytes_to_words(&bytes)
        }
        Some("hex") => {
            let text = fs::read_to_string(path)?;
            hex_to_words(&text)
        }
        _ => bail!("input file must use the .bin or .hex extension"),
    }
}

fn disassemble(words: &[u16]) -> Result<String> {
    if words.len() > INSTR_MEM_SIZE {
        bail!(
            "input contains {} instructions, but AthenISA instruction memory holds at most {INSTR_MEM_SIZE}",
            words.len()
        );
    }

    let mut source = String::new();

    for (address, &word) in words.iter().enumerate() {
        let instruction = decode_instruction(word)
            .with_context(|| format!("instruction address 0x{address:04X}"))?;

        source.push_str(&format_instruction(instruction));
        source.push('\n');
    }

    Ok(source)
}
