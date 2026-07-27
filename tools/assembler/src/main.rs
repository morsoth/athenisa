mod encoder;
mod isa;
mod parser;

use anyhow::{Result, bail};
use clap::Parser;

use isa::{Instruction, Register};

use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(long)]
    hex: bool,

    #[arg(long)]
    sym: bool,

    #[arg(long)]
    debug: bool,

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
    let words = encoder::encode_program(&program.instructions)?;

    if !args.no_bin {
        fs::write(output_path(&args.output, "bin"), encoder::to_bin(&words))?;
    }

    if args.hex {
        fs::write(output_path(&args.output, "hex"), encoder::to_hex(&words))?;
    }

    if args.sym {
        fs::write(
            output_path(&args.output, "sym"),
            format_symbols(&program.symbols),
        )?;
    }

    if args.debug {
        fs::write(
            output_path(&args.output, "lst"),
            format_debug(&program.instructions, &words),
        )?;
    }

    Ok(())
}

fn output_path(output: &str, extension: &str) -> PathBuf {
    let mut path = PathBuf::from(output);
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

fn format_instruction(instruction: Instruction) -> String {
    match instruction {
        Instruction::Nop => "NOP".to_string(),
        Instruction::Ret => "RET".to_string(),
        Instruction::Mov { rd, rs } => format_rr("MOV", rd, rs),
        Instruction::Cmp { rd, rs } => format_rr("CMP", rd, rs),
        Instruction::Not { rd, rs } => format_rr("NOT", rd, rs),
        Instruction::Add { rd, rs1, rs2 } => format_rrr("ADD", rd, rs1, rs2),
        Instruction::Sub { rd, rs1, rs2 } => format_rrr("SUB", rd, rs1, rs2),
        Instruction::And { rd, rs1, rs2 } => format_rrr("AND", rd, rs1, rs2),
        Instruction::Or { rd, rs1, rs2 } => format_rrr("OR", rd, rs1, rs2),
        Instruction::Xor { rd, rs1, rs2 } => format_rrr("XOR", rd, rs1, rs2),
        Instruction::Li { rd, imm8 } => format_ri8("LI", rd, imm8),
        Instruction::Lih { rd, imm8 } => format_ri8("LIH", rd, imm8),
        Instruction::Addi { rd, imm8 } => format_ri8("ADDI", rd, imm8),
        Instruction::Subi { rd, imm8 } => format_ri8("SUBI", rd, imm8),
        Instruction::Cmpi { rd, imm8 } => format_ri8("CMPI", rd, imm8),
        Instruction::Sll { rd, rs, imm4 } => format_shift("SLL", rd, rs, imm4),
        Instruction::Srl { rd, rs, imm4 } => format_shift("SRL", rd, rs, imm4),
        Instruction::Sra { rd, rs, imm4 } => format_shift("SRA", rd, rs, imm4),
        Instruction::Load { rd, rb, off5 } => format_load(rd, rb, off5),
        Instruction::Store { rb, off5, rs } => format_store(rb, off5, rs),
        Instruction::Jmp { addr11 } => format!("JMP 0x{addr11:03X}"),
        Instruction::Call { addr11 } => format!("CALL 0x{addr11:03X}"),
        Instruction::Beq { off11 } => format!("BEQ {off11}"),
        Instruction::Bne { off11 } => format!("BNE {off11}"),
        Instruction::Blt { off11 } => format!("BLT {off11}"),
        Instruction::Bgt { off11 } => format!("BGT {off11}"),
        Instruction::Ble { off11 } => format!("BLE {off11}"),
        Instruction::Bge { off11 } => format!("BGE {off11}"),
    }
}

fn format_rr(op: &str, rd: Register, rs: Register) -> String {
    format!("{op} {}, {}", format_reg(rd), format_reg(rs))
}

fn format_rrr(op: &str, rd: Register, rs1: Register, rs2: Register) -> String {
    format!(
        "{op} {}, {}, {}",
        format_reg(rd),
        format_reg(rs1),
        format_reg(rs2)
    )
}

fn format_ri8(op: &str, rd: Register, imm8: u8) -> String {
    format!("{op} {}, 0x{imm8:02X}", format_reg(rd))
}

fn format_shift(op: &str, rd: Register, rs: Register, imm4: u8) -> String {
    format!("{op} {}, {}, 0x{imm4:X}", format_reg(rd), format_reg(rs))
}

fn format_load(rd: Register, rb: Register, off5: i8) -> String {
    format!("LOAD {}, {}[{}]", format_reg(rd), off5, format_reg(rb))
}

fn format_store(rb: Register, off5: i8, rs: Register) -> String {
    format!("STORE {}[{}], {}", off5, format_reg(rb), format_reg(rs))
}

fn format_reg(reg: Register) -> String {
    format!("R{}", reg.encode())
}
