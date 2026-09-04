# A32 Instruction Formats

All A32 instructions are 32 bits wide. Bits are numbered from 31, the most significant bit, to 0, the least significant bit. The primary `opcode` always occupies bits `31:26`.

A format only defines the position and width of each encoded field. The definition of an instruction specifies how its register and immediate fields are used. Register fields and immediate bits not used by an instruction, along with fields named `reserved`, must be zero in a valid encoding. If any such bit is one, the complete word is an illegal instruction encoding and must not execute as the base instruction. A future exception mechanism may report this condition, but exception handling is outside the current base specification.

## No operand (N)

Used by `NOP` and `RET`.

![No-operand instruction format](imgs/n.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `reserved` | `25:0` | Reserved bits |

## Register (R)

Used by instructions whose operands are all registers.

![Register instruction format](imgs/r.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `rd` | `25:21` | Register field, normally the destination |
| `rs1` | `20:16` | First source-register field |
| `rs2` | `15:11` | Second source-register field |
| `reserved` | `10:2` | Reserved bits |
| `func` | `1:0` | Secondary operation selector |

For three-register operations, including `SLL`, `SRL`, and `SRA`, all register fields are used as named. `MOV` and `NOT` use `rd` and `rs1`. `CMP` uses `rs1` and `rs2`, and does not use `rd`. `JMPR`, `CALLR`, and `PUSH` use `rs1`; `POP` uses `rd`.

## Register-immediate (RI)

Used by `LI`, `LIH`, and `CMPI`.

![Register-immediate instruction format](imgs/ri.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `rd` | `25:21` | Register field |
| `imm21` | `20:0` | Immediate field |

`LI` and `LIH` use `rd` as a destination. `LI` uses the complete `imm21` field. `LIH` stores its `imm16` value in bits `15:0`, while bits `20:16` are unused and must be zero. `CMPI` uses the register field as a source, interprets the complete `imm21` field as signed, and does not write a register.

## Register-register-immediate (RRI)

Used by arithmetic-immediate, logic-immediate, immediate shift, load, and store instructions.

![Register-register-immediate instruction format](imgs/rri.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `rd` | `25:21` | Register field, normally the destination |
| `rs` | `20:16` | Source or base-register field |
| `imm16` | `15:0` | Immediate field |

Arithmetic, logic, immediate shift, and load instructions use `rd` as the destination and `rs` as a source or base register. Store instructions use `rd` as the value source and `rs` as the base register. `SLLI`, `SRLI`, and `SRAI` store `shamt5` in bits `4:0`; bits `15:5` are unused and must be zero. Other instructions define whether `imm16` is a numeric operand or memory offset and how it is extended.

## Immediate (I)

Used by `JMP`, `CALL`, and conditional branches.

![Immediate instruction format](imgs/i.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `imm26` | `25:0` | Immediate field |

Control-flow instructions interpret `imm26` as a signed offset measured in instructions.

`opcode` and `func` assignments are listed in [04_instruction_encoding.md](04_instruction_encoding.md). The interpretation of every immediate field is defined in [02_instruction_set.md](02_instruction_set.md).
