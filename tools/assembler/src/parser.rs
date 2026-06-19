use anyhow::{Context, Result, bail};
use std::collections::HashMap;

use crate::isa::{Instruction, Register};

const INSTR_MEM_SIZE: i32 = 2048;

pub fn parse_program(source: &str) -> Result<Vec<Instruction>> {
    let symbols = collect_symbols(source)?;

    parse_instructions(source, &symbols)
}

fn collect_symbols(source: &str) -> Result<HashMap<String, i32>> {
    let mut symbols = HashMap::new();
    let mut pc = 0;

    for (line_idx, raw_line) in source.lines().enumerate() {
        let line_num = line_idx + 1;
        let line = strip_comment(raw_line).trim();

        if line.is_empty() {
            continue;
        }

        if is_label(line) {
            let (name, value) = parse_symbol(line, pc, &symbols)
                .with_context(|| format!("line {line_num}: {line}"))?;

            if symbols.contains_key(&name) {
                bail!("line {line_num}: symbol '{name}' is already defined");
            }

            symbols.insert(name, value);

            continue;
        }

        if pc >= INSTR_MEM_SIZE {
            bail!(
                "line {line_num}: program exceeds instruction memory size of {INSTR_MEM_SIZE} words"
            );
        } else {
            pc += 1;
        }
    }

    Ok(symbols)
}

fn parse_instructions(source: &str, symbols: &HashMap<String, i32>) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let mut pc = 0;

    for (line_idx, raw_line) in source.lines().enumerate() {
        let line_num = line_idx + 1;
        let line = strip_comment(raw_line).trim();

        if line.is_empty() {
            continue;
        }

        if is_label(line) {
            continue;
        }

        let instruction = parse_instruction(line, pc, symbols)
            .with_context(|| format!("line {line_num}: {line}"))?;
        instructions.push(instruction);
        pc += 1;
    }

    Ok(instructions)
}

fn is_label(line: &str) -> bool {
    line.contains(':')
}

fn strip_comment(line: &str) -> &str {
    line.split(';').next().unwrap()
}

fn parse_symbol(line: &str, pc: i32, symbols: &HashMap<String, i32>) -> Result<(String, i32)> {
    let colon_idx = line.find(':').unwrap();
    let name = line[..colon_idx].trim();
    let value_text = line[colon_idx + 1..].trim();

    if name.is_empty() {
        bail!("symbol name cannot be empty");
    }

    if !is_symbol_name(name) {
        bail!("invalid symbol name '{}'", name);
    }

    let value = parse_symbol_value(value_text, pc, symbols)?;

    Ok((name.to_string(), value))
}

fn is_symbol_name(name: &str) -> bool {
    let mut chars = name.chars();

    let Some(first_char) = chars.next() else {
        return false;
    };

    if !first_char.is_ascii_alphabetic() && first_char != '_' {
        return false;
    }

    for current_char in chars {
        if !current_char.is_ascii_alphanumeric() && current_char != '_' {
            return false;
        }
    }

    true
}

fn parse_symbol_value(text: &str, pc: i32, symbols: &HashMap<String, i32>) -> Result<i32> {
    if text.is_empty() {
        return Ok(pc);
    }

    let parts: Vec<&str> = text.split_whitespace().collect();

    if parts.len() != 1 {
        bail!("symbol value must contain exactly one value");
    }

    parse_value(parts[0], symbols)
}

fn parse_instruction(line: &str, pc: i32, symbols: &HashMap<String, i32>) -> Result<Instruction> {
    let clean = line.replace(",", " ");
    let parts: Vec<&str> = clean.split_whitespace().collect();

    if parts.is_empty() {
        bail!("empty line");
    }

    let op = parts[0].to_ascii_uppercase();

    match op.as_str() {
        "NOP" => {
            expect_tokens(&parts, 1)?;
            Ok(Instruction::Nop)
        }

        "RET" => {
            expect_tokens(&parts, 1)?;
            Ok(Instruction::Ret)
        }

        "MOV" => {
            expect_tokens(&parts, 3)?;

            Ok(Instruction::Mov {
                rd: parse_reg(parts[1])?,
                rs: parse_reg(parts[2])?,
            })
        }

        "LI" => {
            expect_tokens(&parts, 3)?;

            Ok(Instruction::Li {
                rd: parse_reg(parts[1])?,
                imm8: parse_imm8(parts[2], symbols)?,
            })
        }

        "LIH" => {
            expect_tokens(&parts, 3)?;

            Ok(Instruction::Lih {
                rd: parse_reg(parts[1])?,
                imm8: parse_imm8(parts[2], symbols)?,
            })
        }

        "ADD" => {
            expect_tokens(&parts, 4)?;

            Ok(Instruction::Add {
                rd: parse_reg(parts[1])?,
                rs1: parse_reg(parts[2])?,
                rs2: parse_reg(parts[3])?,
            })
        }

        "SUB" => {
            expect_tokens(&parts, 4)?;

            Ok(Instruction::Sub {
                rd: parse_reg(parts[1])?,
                rs1: parse_reg(parts[2])?,
                rs2: parse_reg(parts[3])?,
            })
        }

        "CMP" => {
            expect_tokens(&parts, 3)?;

            Ok(Instruction::Cmp {
                rd: parse_reg(parts[1])?,
                rs: parse_reg(parts[2])?,
            })
        }

        "AND" => {
            expect_tokens(&parts, 4)?;

            Ok(Instruction::And {
                rd: parse_reg(parts[1])?,
                rs1: parse_reg(parts[2])?,
                rs2: parse_reg(parts[3])?,
            })
        }

        "OR" => {
            expect_tokens(&parts, 4)?;

            Ok(Instruction::Or {
                rd: parse_reg(parts[1])?,
                rs1: parse_reg(parts[2])?,
                rs2: parse_reg(parts[3])?,
            })
        }

        "XOR" => {
            expect_tokens(&parts, 4)?;

            Ok(Instruction::Xor {
                rd: parse_reg(parts[1])?,
                rs1: parse_reg(parts[2])?,
                rs2: parse_reg(parts[3])?,
            })
        }

        "NOT" => {
            expect_tokens(&parts, 3)?;

            Ok(Instruction::Not {
                rd: parse_reg(parts[1])?,
                rs: parse_reg(parts[2])?,
            })
        }

        "ADDI" => {
            expect_tokens(&parts, 3)?;

            Ok(Instruction::Addi {
                rd: parse_reg(parts[1])?,
                imm8: parse_imm8(parts[2], symbols)?,
            })
        }

        "SUBI" => {
            expect_tokens(&parts, 3)?;

            Ok(Instruction::Subi {
                rd: parse_reg(parts[1])?,
                imm8: parse_imm8(parts[2], symbols)?,
            })
        }

        "CMPI" => {
            expect_tokens(&parts, 3)?;

            Ok(Instruction::Cmpi {
                rd: parse_reg(parts[1])?,
                imm8: parse_imm8(parts[2], symbols)?,
            })
        }

        "SLL" => {
            expect_tokens(&parts, 4)?;

            Ok(Instruction::Sll {
                rd: parse_reg(parts[1])?,
                rs: parse_reg(parts[2])?,
                imm4: parse_imm4(parts[3], symbols)?,
            })
        }

        "SRL" => {
            expect_tokens(&parts, 4)?;

            Ok(Instruction::Srl {
                rd: parse_reg(parts[1])?,
                rs: parse_reg(parts[2])?,
                imm4: parse_imm4(parts[3], symbols)?,
            })
        }

        "SRA" => {
            expect_tokens(&parts, 4)?;

            Ok(Instruction::Sra {
                rd: parse_reg(parts[1])?,
                rs: parse_reg(parts[2])?,
                imm4: parse_imm4(parts[3], symbols)?,
            })
        }

        "LOAD" => {
            expect_tokens(&parts, 3)?;
            let (rb, off5) = parse_mem_operand(parts[2], symbols)?;

            Ok(Instruction::Load {
                rd: parse_reg(parts[1])?,
                rb,
                off5,
            })
        }

        "STORE" => {
            expect_tokens(&parts, 3)?;
            let (rb, off5) = parse_mem_operand(parts[1], symbols)?;

            Ok(Instruction::Store {
                rb,
                off5,
                rs: parse_reg(parts[2])?,
            })
        }

        "JMP" => {
            expect_tokens(&parts, 2)?;

            Ok(Instruction::Jmp {
                addr11: parse_addr11(parts[1], symbols)?,
            })
        }

        "CALL" => {
            expect_tokens(&parts, 2)?;

            Ok(Instruction::Call {
                addr11: parse_addr11(parts[1], symbols)?,
            })
        }

        "BEQ" => {
            expect_tokens(&parts, 2)?;

            Ok(Instruction::Beq {
                off11: parse_branch_off11(parts[1], pc, symbols)?,
            })
        }

        "BNE" => {
            expect_tokens(&parts, 2)?;

            Ok(Instruction::Bne {
                off11: parse_branch_off11(parts[1], pc, symbols)?,
            })
        }

        "BLT" => {
            expect_tokens(&parts, 2)?;

            Ok(Instruction::Blt {
                off11: parse_branch_off11(parts[1], pc, symbols)?,
            })
        }

        "BGT" => {
            expect_tokens(&parts, 2)?;

            Ok(Instruction::Bgt {
                off11: parse_branch_off11(parts[1], pc, symbols)?,
            })
        }

        "BLE" => {
            expect_tokens(&parts, 2)?;

            Ok(Instruction::Ble {
                off11: parse_branch_off11(parts[1], pc, symbols)?,
            })
        }

        "BGE" => {
            expect_tokens(&parts, 2)?;

            Ok(Instruction::Bge {
                off11: parse_branch_off11(parts[1], pc, symbols)?,
            })
        }

        _ => bail!("unknown instruction '{}'", parts[0]),
    }
}

fn expect_tokens(parts: &[&str], expected: usize) -> Result<()> {
    if parts.len() != expected {
        bail!("expected {expected} tokens, got {}", parts.len());
    }

    Ok(())
}

fn parse_reg(text: &str) -> Result<Register> {
    match text.to_ascii_uppercase().as_str() {
        "R0" => Ok(Register::R0),
        "R1" => Ok(Register::R1),
        "R2" => Ok(Register::R2),
        "R3" => Ok(Register::R3),
        "R4" => Ok(Register::R4),
        "R5" => Ok(Register::R5),
        "R6" => Ok(Register::R6),
        "R7" => Ok(Register::R7),
        _ => bail!("invalid register '{}'", text),
    }
}

fn parse_value(text: &str, symbols: &HashMap<String, i32>) -> Result<i32> {
    if is_number(text) {
        return parse_number(text);
    }

    match symbols.get(text) {
        Some(value) => Ok(*value),
        None => bail!("undefined symbol '{}'", text),
    }
}

fn is_number(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }

    let first_char = text.chars().next().unwrap();

    first_char.is_ascii_digit() || first_char == '-' || first_char == '+'
}

fn parse_number(text: &str) -> Result<i32> {
    let mut number_text = text;
    let mut negative = false;

    if number_text.starts_with('-') {
        negative = true;
        number_text = &number_text[1..];
    } else if number_text.starts_with('+') {
        number_text = &number_text[1..];
    }

    if number_text.is_empty() {
        bail!("invalid number '{}'", text);
    }

    let base;

    if number_text.starts_with("0x") || number_text.starts_with("0X") {
        base = 16;
        number_text = &number_text[2..];
    } else if number_text.starts_with("0b") || number_text.starts_with("0B") {
        base = 2;
        number_text = &number_text[2..];
    } else {
        base = 10;
    }

    if number_text.is_empty() {
        bail!("invalid number '{}'", text);
    }

    let parsed = match i64::from_str_radix(number_text, base) {
        Ok(value) => value,
        Err(_) => bail!("invalid number '{}'", text),
    };

    let value = if negative { -parsed } else { parsed };

    if value < i32::MIN as i64 || value > i32::MAX as i64 {
        bail!("number '{}' is out of range", text);
    }

    Ok(value as i32)
}

fn parse_imm8(text: &str, symbols: &HashMap<String, i32>) -> Result<u8> {
    let value = parse_value(text, symbols)?;

    warn_if_unsigned_truncates(value, 8, "imm8");

    Ok((value & 0xFF) as u8)
}

fn parse_imm4(text: &str, symbols: &HashMap<String, i32>) -> Result<u8> {
    let value = parse_value(text, symbols)?;

    warn_if_unsigned_truncates(value, 4, "imm4");

    Ok((value & 0xF) as u8)
}

fn parse_addr11(text: &str, symbols: &HashMap<String, i32>) -> Result<u16> {
    let value = parse_value(text, symbols)?;

    warn_if_unsigned_truncates(value, 11, "addr11");

    Ok((value & 0x7FF) as u16)
}

fn parse_branch_off11(text: &str, pc: i32, symbols: &HashMap<String, i32>) -> Result<i16> {
    let offset = if is_number(text) {
        parse_number(text)?
    } else {
        let target = parse_value(text, symbols)?;

        target - (pc + 1)
    };

    warn_if_signed_truncates(offset, 11, "off11");

    Ok(cut_signed(offset, 11) as i16)
}

fn parse_mem_operand(text: &str, symbols: &HashMap<String, i32>) -> Result<(Register, i8)> {
    if !text.ends_with(']') {
        bail!("memory operand '{}' must end with ']'", text);
    }

    let Some(bracket_idx) = text.find('[') else {
        bail!("memory operand '{}' must contain '['", text);
    };

    let offset_text = text[..bracket_idx].trim();
    let reg_text = text[bracket_idx + 1..text.len() - 1].trim();

    if reg_text.is_empty() {
        bail!("memory operand '{}' has no base register", text);
    }

    let rb = parse_reg(reg_text)?;
    let offset = if offset_text.is_empty() {
        0
    } else {
        parse_value(offset_text, symbols)?
    };

    warn_if_signed_truncates(offset, 5, "off5");

    Ok((rb, cut_signed(offset, 5) as i8))
}

fn warn_if_unsigned_truncates(value: i32, bits: u32, field: &str) {
    let max_value = (1_i32 << bits) - 1;

    if value < 0 || value > max_value {
        eprintln!("warning: {field} value {value} does not fit in {bits} bits");
    }
}

fn warn_if_signed_truncates(value: i32, bits: u32, field: &str) {
    let min_value = -(1_i32 << (bits - 1));
    let max_value = (1_i32 << (bits - 1)) - 1;

    if value < min_value || value > max_value {
        eprintln!("warning: {field} value {value} does not fit in {bits} bits");
    }
}

fn cut_signed(value: i32, bits: u32) -> i32 {
    let mask = (1_i32 << bits) - 1;
    let sign_bit = 1_i32 << (bits - 1);
    let cut_value = value & mask;

    if cut_value & sign_bit != 0 {
        cut_value - (1_i32 << bits)
    } else {
        cut_value
    }
}
