#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    Ret,
    Mov {
        rd: Register,
        rs: Register,
    },
    Li {
        rd: Register,
        imm8: u8,
    },
    Lih {
        rd: Register,
        imm8: u8,
    },
    Add {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Sub {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Cmp {
        rd: Register,
        rs: Register,
    },
    And {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Or {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Xor {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Not {
        rd: Register,
        rs: Register,
    },
    Addi {
        rd: Register,
        imm8: u8,
    },
    Subi {
        rd: Register,
        imm8: u8,
    },
    Cmpi {
        rd: Register,
        imm8: u8,
    },
    Sll {
        rd: Register,
        rs: Register,
        imm4: u8,
    },
    Srl {
        rd: Register,
        rs: Register,
        imm4: u8,
    },
    Sra {
        rd: Register,
        rs: Register,
        imm4: u8,
    },
    Load {
        rd: Register,
        rb: Register,
        off5: i8,
    },
    Store {
        rb: Register,
        off5: i8,
        rs: Register,
    },
    Jmp {
        addr11: u16,
    },
    Call {
        addr11: u16,
    },
    Beq {
        off11: i16,
    },
    Bne {
        off11: i16,
    },
    Blt {
        off11: i16,
    },
    Bgt {
        off11: i16,
    },
    Ble {
        off11: i16,
    },
    Bge {
        off11: i16,
    },
}

pub(crate) const OP_NOP: u16 = 0b00000;
pub(crate) const OP_ARITM: u16 = 0b00001;
pub(crate) const OP_LOGIC: u16 = 0b00010;
pub(crate) const OP_LI: u16 = 0b00011;
pub(crate) const OP_LIH: u16 = 0b00100;
pub(crate) const OP_SLL: u16 = 0b00101;
pub(crate) const OP_SRL: u16 = 0b00110;
pub(crate) const OP_SRA: u16 = 0b00111;
pub(crate) const OP_JMP: u16 = 0b01000;
pub(crate) const OP_BEQ: u16 = 0b01001;
pub(crate) const OP_BNE: u16 = 0b01010;
pub(crate) const OP_BLT: u16 = 0b01011;
pub(crate) const OP_BGT: u16 = 0b01100;
pub(crate) const OP_BLE: u16 = 0b01101;
pub(crate) const OP_BGE: u16 = 0b01110;
pub(crate) const OP_CALL: u16 = 0b01111;
pub(crate) const OP_RET: u16 = 0b10000;
pub(crate) const OP_LOAD: u16 = 0b10010;
pub(crate) const OP_STORE: u16 = 0b10011;
pub(crate) const OP_ADDI: u16 = 0b10100;
pub(crate) const OP_SUBI: u16 = 0b10101;
pub(crate) const OP_CMPI: u16 = 0b10110;

pub(crate) const FUNC_MOV: u16 = 0b00;
pub(crate) const FUNC_ADD: u16 = 0b01;
pub(crate) const FUNC_SUB: u16 = 0b10;
pub(crate) const FUNC_CMP: u16 = 0b11;

pub(crate) const FUNC_AND: u16 = 0b00;
pub(crate) const FUNC_OR: u16 = 0b01;
pub(crate) const FUNC_XOR: u16 = 0b10;
pub(crate) const FUNC_NOT: u16 = 0b11;

impl Instruction {
    pub fn opcode(self) -> u16 {
        match self {
            Instruction::Nop => OP_NOP,
            Instruction::Ret => OP_RET,
            Instruction::Mov { .. }
            | Instruction::Add { .. }
            | Instruction::Sub { .. }
            | Instruction::Cmp { .. } => OP_ARITM,
            Instruction::And { .. }
            | Instruction::Or { .. }
            | Instruction::Xor { .. }
            | Instruction::Not { .. } => OP_LOGIC,
            Instruction::Li { .. } => OP_LI,
            Instruction::Lih { .. } => OP_LIH,
            Instruction::Sll { .. } => OP_SLL,
            Instruction::Srl { .. } => OP_SRL,
            Instruction::Sra { .. } => OP_SRA,
            Instruction::Jmp { .. } => OP_JMP,
            Instruction::Beq { .. } => OP_BEQ,
            Instruction::Bne { .. } => OP_BNE,
            Instruction::Blt { .. } => OP_BLT,
            Instruction::Bgt { .. } => OP_BGT,
            Instruction::Ble { .. } => OP_BLE,
            Instruction::Bge { .. } => OP_BGE,
            Instruction::Call { .. } => OP_CALL,
            Instruction::Load { .. } => OP_LOAD,
            Instruction::Store { .. } => OP_STORE,
            Instruction::Addi { .. } => OP_ADDI,
            Instruction::Subi { .. } => OP_SUBI,
            Instruction::Cmpi { .. } => OP_CMPI,
        }
    }

    pub fn func(self) -> Option<u16> {
        match self {
            Instruction::Mov { .. } => Some(FUNC_MOV),
            Instruction::Add { .. } => Some(FUNC_ADD),
            Instruction::Sub { .. } => Some(FUNC_SUB),
            Instruction::Cmp { .. } => Some(FUNC_CMP),
            Instruction::And { .. } => Some(FUNC_AND),
            Instruction::Or { .. } => Some(FUNC_OR),
            Instruction::Xor { .. } => Some(FUNC_XOR),
            Instruction::Not { .. } => Some(FUNC_NOT),
            _ => None,
        }
    }
}

impl Register {
    pub fn encode(self) -> u16 {
        match self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7,
        }
    }

    pub(crate) fn decode(value: u16) -> Register {
        match value & 0b111 {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            _ => Register::R7,
        }
    }
}
