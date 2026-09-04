# A16 Instruction Formats

All A16 instructions are 16-bit wide. Bits are numbered from 15, the most significant bit, to 0, the least significant bit. The primary `opcode` always occupies bits `15:11`.

Fields named `reserved` or `res.` must be zero in a canonical encoding. The shorter `res.` label is used when a reserved field is three bits wide or less.

## No operand

Used by `NOP` and `RET`.

![No-operand instruction format](imgs/no_op.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `reserved` | `10:0` | Reserved bits |

## Register-register-register (RRR)

Used by `ADD`, `SUB`, `AND`, `OR`, and `XOR`.

![Register-register-register instruction format](imgs/rrr.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `func` | `10:9` | Secondary operation selector |
| `rd` | `8:6` | Destination register |
| `rs1` | `5:3` | First source register |
| `rs2` | `2:0` | Second source register |

## Register-register (RR)

Used by `MOV`, `CMP`, and `NOT`.

![Register-register instruction format](imgs/rr.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `func` | `10:9` | Secondary operation selector |
| `rd` | `8:6` | Destination or first operand register |
| `rs` | `5:3` | Source or second operand register |
| `res.` | `2:0` | Reserved bits |

For `CMP`, `rd` is the first comparison operand and no register is written.

## Register (R)

Used by `JMPR`, `CALLR`, `PUSH`, and `POP`.

![Register instruction format](imgs/r.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `func` | `10:9` | Secondary operation selector |
| `r` | `8:6` | Register operand |
| `reserved` | `5:0` | Reserved bits |

For `JMPR`, `CALLR`, and `PUSH`, `r` is a source register. For `POP`, `r` is a destination register. The `111` encoding is illegal for `PUSH` and `POP` because both instructions modify `SP` implicitly.

## Register-immediate (RI)

Used by `LI`, `LIH`, `ADDI`, `SUBI`, and `CMPI`.

![Register-immediate instruction format](imgs/ri.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `r` | `10:8` | Register operand |
| `imm8` | `7:0` | Immediate field |

For `LI` and `LIH`, `r` is the destination register. For `ADDI` and `SUBI`, it is both the source and destination. For `CMPI`, it is a source register and no register is written.

## Register-register-immediate (RRI)

Used by `SLL`, `SRL`, and `SRA`.

![Register-register-immediate instruction format](imgs/rri.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `r1` | `10:8` | First register operand |
| `r2` | `7:5` | Second register operand |
| `imm5` | `4:0` | Immediate field |

For all three shift instructions, `r1` is the destination, `r2` is the source, and `imm5` is the shift amount.

## Load format

Used by `LDW` and `LDB`.

![Load instruction format](imgs/load.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `rd` | `10:8` | Destination register |
| `rb` | `7:5` | Base-address register |
| `imm5` | `4:0` | Signed byte offset |

## Store format

Used by `STW` and `STB`.

![Store instruction format](imgs/store.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `rs` | `10:8` | Source register |
| `rb` | `7:5` | Base-address register |
| `imm5` | `4:0` | Signed byte offset |

## Jump format

Used by `JMP`, `CALL`, `BEQ`, `BNE`, `BLT`, `BGE`, `BLTU`, and `BGEU`.

![Jump instruction format](imgs/jump.png)

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `off11` | `10:0` | Signed instruction offset |

For `JMP`, `CALL`, and conditional branches, `off11` is measured from the instruction that follows the control-flow instruction. The encoded value is shifted left by one when added to `PC`.

---

`opcode` and `func` assignments are listed in [04_instruction_encoding.md](04_instruction_encoding.md).
