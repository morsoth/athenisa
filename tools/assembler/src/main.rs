mod encoder;
mod isa;
mod parser;

use anyhow::Result;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    input: String,

    #[arg(short, long)]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let source = fs::read_to_string(&args.input)?;
    let program = parser::parse_program(&source)?;
    let words = encoder::encode_program(&program)?;
    let hex = encoder::to_hex(&words);

    fs::write(&args.output, hex)?;

    Ok(())
}
