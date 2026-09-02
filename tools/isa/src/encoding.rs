use anyhow::{Result, bail};

use crate::instruction::*;

pub fn encode_program(program: &[Instruction]) -> Vec<u16> {
    let mut words = Vec::new();

    for &instruction in program {
        words.push(encode_instruction(instruction));
    }

    words
}

pub fn encode_instruction(instruction: Instruction) -> u16 {
    match instruction {
        Instruction::Nop | Instruction::Ret => opcode(instruction),

        Instruction::Mov { rd, rs } => encode_rr(instruction, rd, rs),

        Instruction::Li { rd, imm8 }
        | Instruction::Lih { rd, imm8 }
        | Instruction::Addi { rd, imm8 }
        | Instruction::Subi { rd, imm8 }
        | Instruction::Cmpi { rd, imm8 } => encode_ri(instruction, rd, imm8),

        Instruction::Add { rd, rs1, rs2 }
        | Instruction::Sub { rd, rs1, rs2 }
        | Instruction::And { rd, rs1, rs2 }
        | Instruction::Or { rd, rs1, rs2 }
        | Instruction::Xor { rd, rs1, rs2 } => encode_rrr(instruction, rd, rs1, rs2),

        Instruction::Cmp { rd, rs } | Instruction::Not { rd, rs } => encode_rr(instruction, rd, rs),

        Instruction::Sll { rd, rs, imm4 }
        | Instruction::Srl { rd, rs, imm4 }
        | Instruction::Sra { rd, rs, imm4 } => encode_rri(instruction, rd, rs, imm4),

        Instruction::Load { rd, rb, imm5 } => encode_load(instruction, rd, rb, imm5),

        Instruction::Store { rb, imm5, rs } => encode_store(instruction, rb, imm5, rs),

        Instruction::Jmp { imm11 } | Instruction::Call { imm11 } => {
            encode_unsigned_imm11(instruction, imm11)
        }

        Instruction::Beq { imm11 }
        | Instruction::Bne { imm11 }
        | Instruction::Blt { imm11 }
        | Instruction::Bgt { imm11 }
        | Instruction::Ble { imm11 }
        | Instruction::Bge { imm11 } => encode_signed_imm11(instruction, imm11),
    }
}

pub fn decode_program(words: &[u16]) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();

    for &word in words {
        instructions.push(decode_instruction(word)?);
    }

    Ok(instructions)
}

pub fn decode_instruction(word: u16) -> Result<Instruction> {
    let opcode = word >> 11;

    match opcode {
        OP_NOP => decode_no_operand(word, Instruction::Nop),
        OP_ARITM => decode_arithmetic(word),
        OP_LOGIC => decode_logic(word),
        OP_LI => Ok(Instruction::Li {
            rd: rd(word),
            imm8: imm8(word),
        }),
        OP_LIH => Ok(Instruction::Lih {
            rd: rd(word),
            imm8: imm8(word),
        }),
        OP_SLL => {
            ensure_zero(word, 0x0010, "reserved bit of SLL is not zero")?;
            Ok(Instruction::Sll {
                rd: rd(word),
                rs: rs1(word),
                imm4: (word & 0x000F) as u8,
            })
        }
        OP_SRL => {
            ensure_zero(word, 0x0010, "reserved bit of SRL is not zero")?;
            Ok(Instruction::Srl {
                rd: rd(word),
                rs: rs1(word),
                imm4: (word & 0x000F) as u8,
            })
        }
        OP_SRA => {
            ensure_zero(word, 0x0010, "reserved bit of SRA is not zero")?;
            Ok(Instruction::Sra {
                rd: rd(word),
                rs: rs1(word),
                imm4: (word & 0x000F) as u8,
            })
        }
        OP_JMP => Ok(Instruction::Jmp {
            imm11: word & 0x07FF,
        }),
        OP_BEQ => Ok(Instruction::Beq {
            imm11: signed_field(word & 0x07FF, 11),
        }),
        OP_BNE => Ok(Instruction::Bne {
            imm11: signed_field(word & 0x07FF, 11),
        }),
        OP_BLT => Ok(Instruction::Blt {
            imm11: signed_field(word & 0x07FF, 11),
        }),
        OP_BGT => Ok(Instruction::Bgt {
            imm11: signed_field(word & 0x07FF, 11),
        }),
        OP_BLE => Ok(Instruction::Ble {
            imm11: signed_field(word & 0x07FF, 11),
        }),
        OP_BGE => Ok(Instruction::Bge {
            imm11: signed_field(word & 0x07FF, 11),
        }),
        OP_CALL => Ok(Instruction::Call {
            imm11: word & 0x07FF,
        }),
        OP_RET => decode_no_operand(word, Instruction::Ret),
        OP_LOAD => Ok(Instruction::Load {
            rd: rd(word),
            rb: rs1(word),
            imm5: signed_field(word & 0x001F, 5) as i8,
        }),
        OP_STORE => Ok(Instruction::Store {
            rb: rs1(word),
            imm5: signed_field(word & 0x001F, 5) as i8,
            rs: rd(word),
        }),
        OP_ADDI => Ok(Instruction::Addi {
            rd: rd(word),
            imm8: imm8(word),
        }),
        OP_SUBI => Ok(Instruction::Subi {
            rd: rd(word),
            imm8: imm8(word),
        }),
        OP_CMPI => Ok(Instruction::Cmpi {
            rd: rd(word),
            imm8: imm8(word),
        }),
        _ => bail!("cannot decode 0x{word:04X}: reserved opcode"),
    }
}

fn decode_no_operand(word: u16, instruction: Instruction) -> Result<Instruction> {
    ensure_zero(
        word,
        0x07FF,
        "reserved bits of no-operand instruction are not zero",
    )?;

    Ok(instruction)
}

fn decode_arithmetic(word: u16) -> Result<Instruction> {
    match func_bits(word) {
        FUNC_MOV => {
            ensure_zero(word, 0x001C, "reserved bits of MOV are not zero")?;
            Ok(Instruction::Mov {
                rd: rd(word),
                rs: rs1(word),
            })
        }
        FUNC_ADD => Ok(Instruction::Add {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        }),
        FUNC_SUB => Ok(Instruction::Sub {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        }),
        FUNC_CMP => {
            ensure_zero(word, 0x001C, "reserved bits of CMP are not zero")?;
            Ok(Instruction::Cmp {
                rd: rd(word),
                rs: rs1(word),
            })
        }
        _ => unreachable!(),
    }
}

fn decode_logic(word: u16) -> Result<Instruction> {
    match func_bits(word) {
        FUNC_AND => Ok(Instruction::And {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        }),
        FUNC_OR => Ok(Instruction::Or {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        }),
        FUNC_XOR => Ok(Instruction::Xor {
            rd: rd(word),
            rs1: rs1(word),
            rs2: rs2(word),
        }),
        FUNC_NOT => {
            ensure_zero(word, 0x001C, "reserved bits of NOT are not zero")?;
            Ok(Instruction::Not {
                rd: rd(word),
                rs: rs1(word),
            })
        }
        _ => unreachable!(),
    }
}

fn ensure_zero(word: u16, mask: u16, message: &str) -> Result<()> {
    if word & mask != 0 {
        bail!("cannot decode 0x{word:04X}: {message}");
    }

    Ok(())
}

fn opcode(instruction: Instruction) -> u16 {
    instruction.opcode() << 11
}

fn instruction_func(instruction: Instruction) -> u16 {
    instruction
        .func()
        .expect("encoding bug: instruction format requires a func field")
}

fn encode_rr(instruction: Instruction, rd: Register, rs: Register) -> u16 {
    opcode(instruction) | (rd.encode() << 8) | (rs.encode() << 5) | instruction_func(instruction)
}

fn encode_rrr(instruction: Instruction, rd: Register, rs1: Register, rs2: Register) -> u16 {
    opcode(instruction)
        | (rd.encode() << 8)
        | (rs1.encode() << 5)
        | (rs2.encode() << 2)
        | instruction_func(instruction)
}

fn encode_ri(instruction: Instruction, rd: Register, imm8: u8) -> u16 {
    opcode(instruction) | (rd.encode() << 8) | imm8 as u16
}

fn encode_rri(instruction: Instruction, rd: Register, rs: Register, imm4: u8) -> u16 {
    opcode(instruction) | (rd.encode() << 8) | (rs.encode() << 5) | imm4 as u16
}

fn encode_load(instruction: Instruction, rd: Register, rb: Register, imm5: i8) -> u16 {
    opcode(instruction) | (rd.encode() << 8) | (rb.encode() << 5) | encode_signed_imm5(imm5)
}

fn encode_store(instruction: Instruction, rb: Register, imm5: i8, rs: Register) -> u16 {
    opcode(instruction) | (rs.encode() << 8) | (rb.encode() << 5) | encode_signed_imm5(imm5)
}

fn encode_signed_imm5(imm5: i8) -> u16 {
    (imm5 as i16 as u16) & 0x1F
}

fn encode_unsigned_imm11(instruction: Instruction, imm11: u16) -> u16 {
    opcode(instruction) | (imm11 & 0x07FF)
}

fn encode_signed_imm11(instruction: Instruction, imm11: i16) -> u16 {
    opcode(instruction) | ((imm11 as u16) & 0x07FF)
}

fn rd(word: u16) -> Register {
    Register::decode(word >> 8)
}

fn rs1(word: u16) -> Register {
    Register::decode(word >> 5)
}

fn rs2(word: u16) -> Register {
    Register::decode(word >> 2)
}

fn func_bits(word: u16) -> u16 {
    word & 0b11
}

fn imm8(word: u16) -> u8 {
    (word & 0x00FF) as u8
}

fn signed_field(value: u16, bits: u8) -> i16 {
    let shift = 16 - bits;
    ((value << shift) as i16) >> shift
}
