use crate::instruction::{Instruction, Register};

pub fn format_instruction(instruction: Instruction) -> String {
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
        Instruction::Load { rd, rb, imm5 } => format_load(rd, rb, imm5),
        Instruction::Store { rb, imm5, rs } => format_store(rb, imm5, rs),
        Instruction::Jmp { imm11 } => format!("JMP 0x{imm11:03X}"),
        Instruction::Call { imm11 } => format!("CALL 0x{imm11:03X}"),
        Instruction::Beq { imm11 } => format!("BEQ {imm11}"),
        Instruction::Bne { imm11 } => format!("BNE {imm11}"),
        Instruction::Blt { imm11 } => format!("BLT {imm11}"),
        Instruction::Bgt { imm11 } => format!("BGT {imm11}"),
        Instruction::Ble { imm11 } => format!("BLE {imm11}"),
        Instruction::Bge { imm11 } => format!("BGE {imm11}"),
    }
}

fn register_to_assembly(register: Register) -> String {
    format!("R{}", register.encode())
}

fn format_rr(op: &str, rd: Register, rs: Register) -> String {
    format!(
        "{op} {}, {}",
        register_to_assembly(rd),
        register_to_assembly(rs)
    )
}

fn format_rrr(op: &str, rd: Register, rs1: Register, rs2: Register) -> String {
    format!(
        "{op} {}, {}, {}",
        register_to_assembly(rd),
        register_to_assembly(rs1),
        register_to_assembly(rs2)
    )
}

fn format_ri8(op: &str, rd: Register, imm8: u8) -> String {
    format!("{op} {}, 0x{imm8:02X}", register_to_assembly(rd))
}

fn format_shift(op: &str, rd: Register, rs: Register, imm4: u8) -> String {
    format!(
        "{op} {}, {}, 0x{imm4:X}",
        register_to_assembly(rd),
        register_to_assembly(rs)
    )
}

fn format_load(rd: Register, rb: Register, imm5: i8) -> String {
    format!(
        "LOAD {}, {}[{}]",
        register_to_assembly(rd),
        imm5,
        register_to_assembly(rb)
    )
}

fn format_store(rb: Register, imm5: i8, rs: Register) -> String {
    format!(
        "STORE {}[{}], {}",
        imm5,
        register_to_assembly(rb),
        register_to_assembly(rs)
    )
}
