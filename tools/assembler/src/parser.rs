use anyhow::{Result, bail};
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
            let (name, value) = parse_symbol(line, line_num, pc)?;

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

fn parse_instructions(source: &str, _symbols: &HashMap<String, i32>) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();

    for (line_idx, raw_line) in source.lines().enumerate() {
        let line_num = line_idx + 1;
        let line = strip_comment(raw_line).trim();

        if line.is_empty() {
            continue;
        }

        if is_label(line) {
            continue;
        }

        let instruction = parse_instruction(line, line_num)?;
        instructions.push(instruction);
    }

    Ok(instructions)
}

fn is_label(line: &str) -> bool {
    line.contains(':')
}

fn strip_comment(line: &str) -> &str {
    line.split(';').next().unwrap()
}

fn parse_symbol(line: &str, line_num: usize, pc: i32) -> Result<(String, i32)> {
    let colon_idx = line.find(':').unwrap();
    let name = line[..colon_idx].trim();
    let value_text = line[colon_idx + 1..].trim();

    if name.is_empty() {
        bail!("line {line_num}: symbol name cannot be empty");
    }

    let value = if value_text.is_empty() {
        pc
    } else {
        parse_i32(value_text, line_num)?
    };

    Ok((name.to_string(), value))
}

fn parse_instruction(line: &str, line_num: usize) -> Result<Instruction> {
    let clean = line.replace(",", " ");
    let parts: Vec<&str> = clean.split_whitespace().collect();

    if parts.is_empty() {
        bail!("line {line_num}: empty line");
    }

    let op = parts[0].to_ascii_uppercase();

    match op.as_str() {
        "NOP" => {
            expect_len(&parts, 1, line_num)?;
            Ok(Instruction::Nop)
        }

        "RET" => {
            expect_len(&parts, 1, line_num)?;
            Ok(Instruction::Ret)
        }

        "MOV" => {
            expect_len(&parts, 3, line_num)?;

            Ok(Instruction::Mov {
                rd: parse_reg(parts[1], line_num)?,
                rs: parse_reg(parts[2], line_num)?,
            })
        }

        "LI" => {
            expect_len(&parts, 3, line_num)?;

            Ok(Instruction::Li {
                rd: parse_reg(parts[1], line_num)?,
                imm8: parse_imm8(parts[2], line_num)?,
            })
        }

        "LIH" => {
            expect_len(&parts, 3, line_num)?;

            Ok(Instruction::Lih {
                rd: parse_reg(parts[1], line_num)?,
                imm8: parse_imm8(parts[2], line_num)?,
            })
        }

        "ADD" => {
            expect_len(&parts, 4, line_num)?;

            Ok(Instruction::Add {
                rd: parse_reg(parts[1], line_num)?,
                rs1: parse_reg(parts[2], line_num)?,
                rs2: parse_reg(parts[3], line_num)?,
            })
        }

        "SUB" => {
            expect_len(&parts, 4, line_num)?;

            Ok(Instruction::Sub {
                rd: parse_reg(parts[1], line_num)?,
                rs1: parse_reg(parts[2], line_num)?,
                rs2: parse_reg(parts[3], line_num)?,
            })
        }

        "CMP" => {
            expect_len(&parts, 3, line_num)?;

            Ok(Instruction::Cmp {
                rd: parse_reg(parts[1], line_num)?,
                rs: parse_reg(parts[2], line_num)?,
            })
        }

        "AND" => {
            expect_len(&parts, 4, line_num)?;

            Ok(Instruction::And {
                rd: parse_reg(parts[1], line_num)?,
                rs1: parse_reg(parts[2], line_num)?,
                rs2: parse_reg(parts[3], line_num)?,
            })
        }

        "OR" => {
            expect_len(&parts, 4, line_num)?;

            Ok(Instruction::Or {
                rd: parse_reg(parts[1], line_num)?,
                rs1: parse_reg(parts[2], line_num)?,
                rs2: parse_reg(parts[3], line_num)?,
            })
        }

        "XOR" => {
            expect_len(&parts, 4, line_num)?;

            Ok(Instruction::Xor {
                rd: parse_reg(parts[1], line_num)?,
                rs1: parse_reg(parts[2], line_num)?,
                rs2: parse_reg(parts[3], line_num)?,
            })
        }

        "NOT" => {
            expect_len(&parts, 3, line_num)?;

            Ok(Instruction::Not {
                rd: parse_reg(parts[1], line_num)?,
                rs: parse_reg(parts[2], line_num)?,
            })
        }

        "ADDI" => {
            expect_len(&parts, 3, line_num)?;

            Ok(Instruction::Addi {
                rd: parse_reg(parts[1], line_num)?,
                imm8: parse_imm8(parts[2], line_num)?,
            })
        }

        "SUBI" => {
            expect_len(&parts, 3, line_num)?;

            Ok(Instruction::Subi {
                rd: parse_reg(parts[1], line_num)?,
                imm8: parse_imm8(parts[2], line_num)?,
            })
        }

        "CMPI" => {
            expect_len(&parts, 3, line_num)?;

            Ok(Instruction::Cmpi {
                rd: parse_reg(parts[1], line_num)?,
                imm8: parse_imm8(parts[2], line_num)?,
            })
        }

        "SLL" => {
            expect_len(&parts, 4, line_num)?;

            Ok(Instruction::Sll {
                rd: parse_reg(parts[1], line_num)?,
                rs: parse_reg(parts[2], line_num)?,
                imm4: parse_imm4(parts[3], line_num)?,
            })
        }

        "SRL" => {
            expect_len(&parts, 4, line_num)?;

            Ok(Instruction::Srl {
                rd: parse_reg(parts[1], line_num)?,
                rs: parse_reg(parts[2], line_num)?,
                imm4: parse_imm4(parts[3], line_num)?,
            })
        }

        "SRA" => {
            expect_len(&parts, 4, line_num)?;

            Ok(Instruction::Sra {
                rd: parse_reg(parts[1], line_num)?,
                rs: parse_reg(parts[2], line_num)?,
                imm4: parse_imm4(parts[3], line_num)?,
            })
        }

        "LOAD" => {
            expect_len(&parts, 3, line_num)?;
            let (rb, off5) = parse_mem_operand(parts[2], line_num)?;

            Ok(Instruction::Load {
                rd: parse_reg(parts[1], line_num)?,
                rb,
                off5,
            })
        }

        "STORE" => {
            expect_len(&parts, 3, line_num)?;
            let (rb, off5) = parse_mem_operand(parts[1], line_num)?;

            Ok(Instruction::Store {
                rb,
                off5,
                rs: parse_reg(parts[2], line_num)?,
            })
        }

        _ => bail!("line {line_num}: unknown instruction '{}'", parts[0]),
    }
}

fn parse_reg(text: &str, line_num: usize) -> Result<Register> {
    let upper = text.to_ascii_uppercase();

    match upper.as_str() {
        "R0" => Ok(Register::R0),
        "R1" => Ok(Register::R1),
        "R2" => Ok(Register::R2),
        "R3" => Ok(Register::R3),
        "R4" => Ok(Register::R4),
        "R5" => Ok(Register::R5),
        "R6" => Ok(Register::R6),
        "R7" => Ok(Register::R7),
        _ => bail!("line {line_num}: invalid register '{}'", text),
    }
}

fn parse_imm8(text: &str, line_num: usize) -> Result<u8> {
    let value = parse_u16(text, line_num)?;

    if value > 0xFF {
        bail!(
            "line {line_num}: immediate '{}' does not fit in 8 bits",
            text
        );
    }

    Ok(value as u8)
}

fn parse_imm4(text: &str, line_num: usize) -> Result<u8> {
    let value = parse_u16(text, line_num)?;

    if value > 0xF {
        bail!(
            "line {line_num}: immediate '{}' does not fit in 4 bits",
            text
        );
    }

    Ok(value as u8)
}

fn parse_mem_operand(text: &str, line_num: usize) -> Result<(Register, i8)> {
    let open_bracket = match text.find('[') {
        Some(index) => index,
        None => bail!(
            "line {line_num}: memory operand '{}' must use off5[rb]",
            text
        ),
    };

    let close_bracket = match text.find(']') {
        Some(index) => index,
        None => bail!("line {line_num}: memory operand '{}' is missing ']'", text),
    };

    if close_bracket <= open_bracket {
        bail!("line {line_num}: invalid memory operand '{}'", text);
    }

    if close_bracket != text.len() - 1 {
        bail!(
            "line {line_num}: unexpected text after memory operand '{}'",
            text
        );
    }

    let offset_text = &text[..open_bracket];
    let base_text = &text[open_bracket + 1..close_bracket];

    if base_text.is_empty() {
        bail!(
            "line {line_num}: memory operand '{}' is missing base register",
            text
        );
    }

    let rb = parse_reg(base_text, line_num)?;
    let off5 = if offset_text.is_empty() {
        0
    } else {
        parse_off5(offset_text, line_num)?
    };

    Ok((rb, off5))
}

fn parse_off5(text: &str, line_num: usize) -> Result<i8> {
    let value = parse_i32(text, line_num)?;

    if value < -16 || value > 15 {
        bail!(
            "line {line_num}: offset '{}' does not fit in signed 5 bits",
            text
        );
    }

    Ok(value as i8)
}

fn parse_u16(text: &str, line_num: usize) -> Result<u16> {
    let radix;
    let digits;

    if text.starts_with("0x") || text.starts_with("0X") {
        radix = 16;
        digits = &text[2..];
    } else if text.starts_with("0b") || text.starts_with("0B") {
        radix = 2;
        digits = &text[2..];
    } else {
        radix = 10;
        digits = text;
    }

    match u16::from_str_radix(digits, radix) {
        Ok(value) => Ok(value),
        Err(_) => bail!("line {line_num}: invalid immediate '{}'", text),
    }
}

fn parse_i32(text: &str, line_num: usize) -> Result<i32> {
    if text.starts_with('-') {
        let digits = &text[1..];

        return match digits.parse::<i32>() {
            Ok(value) => Ok(-value),
            Err(_) => bail!("line {line_num}: invalid number '{}'", text),
        };
    }

    let radix;
    let digits;

    if text.starts_with("0x") || text.starts_with("0X") {
        radix = 16;
        digits = &text[2..];
    } else if text.starts_with("0b") || text.starts_with("0B") {
        radix = 2;
        digits = &text[2..];
    } else {
        return match text.parse::<i32>() {
            Ok(value) => Ok(value),
            Err(_) => bail!("line {line_num}: invalid number '{}'", text),
        };
    }

    match i32::from_str_radix(digits, radix) {
        Ok(value) => Ok(value),
        Err(_) => bail!("line {line_num}: invalid number '{}'", text),
    }
}

fn expect_len(parts: &[&str], expected: usize, line_num: usize) -> Result<()> {
    if parts.len() != expected {
        bail!(
            "line {line_num}: expected {expected} tokens, got {}",
            parts.len()
        );
    }

    Ok(())
}
