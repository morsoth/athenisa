# A32 Instruction Formats

All A32 instructions are 32 bits wide. Bits are numbered from 31, the most significant bit, to 0, the least significant bit. The primary `opcode` always occupies bits `31:26`.

Fields named `reserved` must be zero in a canonical encoding. The `func` field is three bits wide and selects one operation within an opcode group.

## No operand

Used by `NOP` and `RET`.

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `reserved` | `25:0` | Reserved bits |

## Register-register-register (RRR)

Used by `ADD`, `SUB`, `AND`, `OR`, and `XOR`.

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `rd` | `25:21` | Destination register |
| `rs1` | `20:16` | First source register |
| `rs2` | `15:11` | Second source register |
| `reserved` | `10:3` | Reserved bits |
| `func` | `2:0` | Secondary operation selector |

## Register-register (RR)

Used by `MOV`, `CMP`, and `NOT`.

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `rd` or `rs1` | `25:21` | Destination or first operand register |
| `rs` or `rs2` | `20:16` | Source or second operand register |
| `reserved` | `15:3` | Reserved bits |
| `func` | `2:0` | Secondary operation selector |

For `CMP`, both register fields are sources and no register is written.

## Register (R)

Used by `JMPR`, `CALLR`, `PUSH`, and `POP`.

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `r` | `25:21` | Register operand |
| `reserved` | `20:3` | Reserved bits |
| `func` | `2:0` | Secondary operation selector |

For `JMPR`, `CALLR`, and `PUSH`, `r` is a source register. For `POP`, `r` is a destination register. Encoding `11111` is illegal for `PUSH` and `POP` because both instructions modify `SP` implicitly.

## Register-immediate (RI)

Used by `LI`, `LIH`, and `CMPI`.

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `rd` | `25:21` | Destination or comparison register |
| `reserved` | `20:16` | Reserved bits |
| `imm16` | `15:0` | Unsigned 16-bit immediate |

For `CMPI`, `rd` is only used as the first comparison operand and not as a destination register.

## Register-register-immediate (RRI)

Used by `SLL`, `SRL`, `SRA`, `ADDI`, and `SUBI`.

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `rd` | `25:21` | Destination register |
| `rs` | `20:16` | Source register |
| `imm16` | `15:0` | Unsigned 16-bit immediate |

Shift instructions accept values from `0` to `31`. Their `imm16` field therefore has bits `15:5` cleared and stores the shift amount in bits `4:0`.

## Absolute jump

Used by `JMP` and `CALL`.

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `addr26` | `25:0` | Absolute instruction address |

The field spans the complete `2^26`-word instruction address space.

## Relative branch

Used by `BRA`, `BEQ`, `BNE`, `BLT`, `BGE`, `BLTU`, and `BGEU`.

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `off26` | `25:0` | Signed PC-relative instruction offset |

The target is `PC + 1 + off26`, calculated using 26-bit wrapping arithmetic.

## Load

Used by `LOAD`.

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `rd` | `25:21` | Destination register |
| `rb` | `20:16` | Base-address register |
| `off16` | `15:0` | Signed data offset |

## Store

Used by `STORE`.

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `31:26` | Primary opcode |
| `rs` | `25:21` | Source data register |
| `rb` | `20:16` | Base-address register |
| `off16` | `15:0` | Signed data offset |

`opcode` and `func` assignments are listed in [04_instruction_encoding.md](04_instruction_encoding.md).
