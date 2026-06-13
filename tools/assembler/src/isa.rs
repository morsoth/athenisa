#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
}

const OP_NOP: u16 = 0b00000;
const OP_ARITM: u16 = 0b00001;
const OP_LOGIC: u16 = 0b00010;
const OP_LI: u16 = 0b00011;
const OP_LIH: u16 = 0b00100;
const OP_SLL: u16 = 0b00101;
const OP_SRL: u16 = 0b00110;
const OP_SRA: u16 = 0b00111;
const OP_RET: u16 = 0b10000;
const OP_LOAD: u16 = 0b10010;
const OP_STORE: u16 = 0b10011;
const OP_ADDI: u16 = 0b10100;
const OP_SUBI: u16 = 0b10101;
const OP_CMPI: u16 = 0b10110;

const FUNC_MOV: u16 = 0b00;
const FUNC_ADD: u16 = 0b01;
const FUNC_SUB: u16 = 0b10;
const FUNC_CMP: u16 = 0b11;

const FUNC_AND: u16 = 0b00;
const FUNC_OR: u16 = 0b01;
const FUNC_XOR: u16 = 0b10;
const FUNC_NOT: u16 = 0b11;

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
}
