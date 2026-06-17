use anyhow::Result;

use crate::isa::{Instruction, Register};

pub fn encode_program(program: &[Instruction]) -> Result<Vec<u16>> {
    let mut words = Vec::new();

    for &instruction in program {
        words.push(encode_instruction(instruction));
    }

    Ok(words)
}

pub fn to_hex(words: &[u16]) -> String {
    let mut hex = String::new();

    for word in words {
        hex.push_str(&format!("{word:04X}\n"));
    }

    hex
}

fn encode_instruction(instruction: Instruction) -> u16 {
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

        Instruction::Load { rd, rb, off5 } => encode_load(instruction, rd, rb, off5),

        Instruction::Store { rb, off5, rs } => encode_store(instruction, rb, off5, rs),

        Instruction::Jmp { addr11 } | Instruction::Call { addr11 } => {
            encode_addr11(instruction, addr11)
        }

        Instruction::Beq { off11 }
        | Instruction::Bne { off11 }
        | Instruction::Blt { off11 }
        | Instruction::Bgt { off11 }
        | Instruction::Ble { off11 }
        | Instruction::Bge { off11 } => encode_off11(instruction, off11),
    }
}

fn opcode(instruction: Instruction) -> u16 {
    instruction.opcode() << 11
}

fn func(instruction: Instruction) -> u16 {
    instruction
        .func()
        .expect("encoder bug: instruction format requires a func field")
}

fn encode_rr(instruction: Instruction, rd: Register, rs: Register) -> u16 {
    opcode(instruction) | (rd.encode() << 8) | (rs.encode() << 5) | func(instruction)
}

fn encode_rrr(instruction: Instruction, rd: Register, rs1: Register, rs2: Register) -> u16 {
    opcode(instruction)
        | (rd.encode() << 8)
        | (rs1.encode() << 5)
        | (rs2.encode() << 2)
        | func(instruction)
}

fn encode_ri(instruction: Instruction, rd: Register, imm8: u8) -> u16 {
    opcode(instruction) | (rd.encode() << 8) | imm8 as u16
}

fn encode_rri(instruction: Instruction, rd: Register, rs: Register, imm4: u8) -> u16 {
    opcode(instruction) | (rd.encode() << 8) | (rs.encode() << 5) | imm4 as u16
}

fn encode_load(instruction: Instruction, rd: Register, rb: Register, off5: i8) -> u16 {
    opcode(instruction) | (rd.encode() << 8) | (rb.encode() << 5) | encode_off5(off5)
}

fn encode_store(instruction: Instruction, rb: Register, off5: i8, rs: Register) -> u16 {
    opcode(instruction) | (rs.encode() << 8) | (rb.encode() << 5) | encode_off5(off5)
}

fn encode_off5(off5: i8) -> u16 {
    (off5 as i16 as u16) & 0x1F
}

fn encode_addr11(instruction: Instruction, addr11: u16) -> u16 {
    opcode(instruction) | (addr11 & 0x7FF)
}

fn encode_off11(instruction: Instruction, off11: i16) -> u16 {
    opcode(instruction) | ((off11 as u16) & 0x7FF)
}
