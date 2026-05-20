# AthenISA Specification

###### Version: 0.1

AthenISA is a compact 16-bit instruction set architecture designed for the Tydeus-16 core. It uses fixed-width 16-bit instructions and a small architectural register set.

AthenISA defines the programmer-visible behavior of the processor:

- architectural registers
- instruction formats
- instruction semantics
- opcode allocation
- memory model
- stack behavior
- control-flow behavior

Implementation details such as the multicycle datapath, internal control unit, FSM states, and stage timing are not part of AthenISA. Those are documented in the Tydeus-16 core microarchitecture document.

## 1. Architectural Summary

| Property | Value |
| --- | --- |
| Data width | 16 bits |
| Instruction width | 16 bits |
| Instruction encoding | Fixed-width, 16-bit |
| General-purpose registers | 7, named R1-R7 |
| Zero register | R0 |
| Program counter width | 11 bits |
| Stack pointer width | 16 bits |
| Flags register width | 4 bits |
| Instruction memory model | Separate instruction address space |
| Data memory model | Separate data address space |

## 2. Registers

The processor exposes a compact architectural register set composed of one zero register,
seven general-purpose registers, the program counter, the stack pointer, and a flags register.

### Zero register

`R0` is a 16-bit constant zero register. Its value is permanently fixed to `0x0000`, and any write
to `R0` is ignored.

### General-Purpose registers

The processor provides seven 16-bit general-purpose registers, `R1` to `R7`. These registers are
used as operands and destinations for the instructions.

### Program counter register

The program counter, `PC`, is an 11-bit register that holds the address of the current instruction
in instruction memory.

> [!NOTE]
> `PC` register is 11-bit wide due to the instruction memory size being `2^11 = 2048` words. This allows absolute jumps to reach all the instruction memory adressess with the 11 bits encoded in the instruction.

### Stack pointer register

The stack pointer, SP, is a 16-bit register that points to the element currently located at the
top of the stack.

### Flags register

The `FLAGS` register is a 4-bit register that stores the processor’s ALU status flags. It contains
the following four 1-bit elements:

| Flag | Meaning |
| --- | --- |
| Z | Zero flag |
| C | Carry flag |
| N | Negative flag |
| V | Overflow flag |

## 3. Instruction Encoding Model

All AthenISA instructions are 16 bits wide. Depending on the instruction, the encoded fields may represent registers, immediates, absolute addresses, or relative offsets.

The top 5 bits encode the primary opcode field. Some instruction groups use a secondary `func` field to distinguish operations sharing the same primary opcode.

## 4. Instruction Formats

### No-operand

`NOP`, `RET`

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `reserved` | `10:0` | Reserved bits, should be encoded as zero by assemblers |

### Register-register-register (RRR)

`ADD`, `SUB`, `AND`, `OR`, `XOR`

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `rd` | `10:8` | Destination register |
| `rs1` | `7:5` | First source register |
| `rs2` | `4:2` | Second source register |
| `func` | `1:0` | Secondary function selector |

### Register-register (RR)

`MOV`, `NOT`, `CMP`

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `rd` | `10:8` | Destination or first operand register |
| `rs` | `7:5` | Source or second operand register |
| `reserved` | `4:2` | Reserved bits, should be encoded as zero by assemblers |
| `func` | `1:0` | Secondary function selector |

### Register-immediate (RI)

`LI`, `LIH`, `ADDI`, `SUBI`, `CMPI`

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `rd` | `10:8` | Destination or operand register |
| `imm(8)` | `7:0` | 8-bit immediate |

### Register-register-immediate (RRI)

`SLL`, `SRL`, `SRA`

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `rd` | `10:8` | Destination register |
| `rs` | `7:5` | Source register |
| `reserved` | `4` | Reserved bit, should be encoded as zero by assemblers |
| `imm(4)` | `3:0` | 4-bit shift amount |

> [!NOTE]
> A 4-bit shift immediate is sufficient for a 16-bit architecture because meaningful shift amounts range from 0 to 15.

### Unconditional jump

`JMP`, `CALL`

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `addr(11)` | `10:0` | Absolute 11-bit instruction address |

> [!NOTE]
> The 11 bits available for the absolute instruction address constrain instruction memory to `2^11` words. This ensures that an absolute jump can reach any instruction in memory.

### Conditional branch

`BEQ`, `BNE`, `BLT`, `BGT`, `BLE`, `BGE`

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `off(11)` | `10:0` | Signed 11-bit PC-relative offset |

> [!NOTE]
> Conditional branch targets are computed relative to `PC + 1`.

### Load

`LOAD`

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `rd` | `10:8` | Destination register |
| `rb` | `7:5` | Base register |
| `off(5)` | `4:0` | Signed 5-bit offset |

### Store

`STORE`

| Field | Bits | Description |
| --- | --- | --- |
| `opcode` | `15:11` | Primary opcode |
| `rs` | `10:8` | Source data register |
| `rb` | `7:5` | Base register |
| `off(5)` | `4:0` | Signed 5-bit offset |

## 5. Instruction Set

### No-operation instructions

#### NOP

The `NOP` instruction performs no operation and does not modify the architectural state of the processor.

```
NOP                     // does nothing
```

### Data movement instructions

#### MOV

The `MOV` instruction copies the contents of a source register into a destination register.

```
MOV rd, rs              // rd <- rs
```

#### LI

The `LI` instruction loads an 8-bit immediate into the lower half of the destination register and clears the upper half to zero. This instruction is used to efficiently load small constants.

```
LI rd, imm(8)           // rd[7:0] <- imm(8), rd[15:8] <- 0x00
```

#### LIH

The `LIH` instruction loads an 8-bit immediate into the upper half of the destination register while leaving the lower half unchanged. Combined with `LI`, it allows constructing a full 16-bit constant.

> [!NOTE]
> To successfully load a 16-bit immediate into a register it is mandatory to use `LI` first and then `LIH`.

```
LIH rd, imm(8)          // rd[15:8] <- imm(8)
```

### Arithmetic and logic instructions

#### ADD

The `ADD` instruction adds two source registers and writes the result to the destination register.

```
ADD rd, rs1, rs2        // rd <- rs1 + rs2
```

#### ADDI

The `ADDI` instruction adds a zero-extended 8-bit immediate to the destination register.

```
ADDI rd, imm(8)         // rd <- rd + zext(imm(8))
```

#### SUB

The `SUB` instruction subtracts the second source register from the first source register and writes the result to the destination register.

```
SUB rd, rs1, rs2        // rd <- rs1 - rs2
```

#### SUBI

The `SUBI` instruction subtracts a zero-extended 8-bit immediate from the destination register.

```
SUBI rd, imm(8)         // rd <- rd - zext(imm(8))
```

#### AND

The `AND` instruction performs a bitwise AND between two source registers.

```
AND rd, rs1, rs2        // rd <- rs1 & rs2
```

> [!NOTE]
> Logical operations (`AND`, `OR`, `XOR`, `NOT`) do not have immediate versions because an 8-bit immediate is not especially useful in a 16-bit architecture, particularly for full-width bit masks. These instructions may be added in the future if free opcodes remain available.

#### OR

The `OR` instruction performs a bitwise OR between two source registers.

```
OR rd, rs1, rs2         // rd <- rs1 | rs2
```

#### XOR

The `XOR` instruction performs a bitwise XOR between two source registers.

```
XOR rd, rs1, rs2        // rd <- rs1 ^ rs2
```

#### NOT

The `NOT` instruction performs a bitwise negation of the source register.

```
NOT rd, rs              // rd <- ~rs
```

#### CMP

The `CMP` instruction behaves like a subtraction operation, but the result is not written back to the register file. Instead, only the status flags are updated.

```
CMP rd, rs              // rd - rs
```

#### CMPI

The `CMPI` compares the register operand with a zero-extended 8-bit immediate and acts as the `CMP` instruction.

```
CMPI rd, imm(8)         // rd - zext(imm(8))
```

### Shift instructions

#### SLL

The `SLL` instruction performs a logical left shift of the source register by an immediate amount.

```
SLL rd, rs, imm(4)      // rd <- rs << imm(4)
```

> [!NOTE]
> Shift instructions use only a 4-bit immediate because this is sufficient to encode all meaningful shift amounts in a 16-bit architecture. A 4-bit field allows values from 0 to 15, which covers the full useful shift range for a 16-bit operand.

#### SRL

The `SRL` instruction performs a logical right shift of the source register by an immediate amount.

```
SRL rd, rs, imm(4)      // rd <- rs >> imm(4)
```

#### SRA

The `SRA` instruction performs an arithmetic right shift of the source register by an immediate amount.

```
SRA rd, rs, imm(4)      // rd <- rs >> imm(4) ; arithmetic
```

### Jump instructions

#### JMP

The `JMP` instruction performs an unconditional jump to an absolute 11-bit address in instruction memory.

```
JMP addr(11)            // PC <- addr(11)
```

#### BEQ

The `BEQ` instruction performs a relative branch if the zero flag is set.

```
BEQ off(11)             // if Z = 1
                        // then PC <- PC+1 + sext(off(11))
```

#### BNE

The `BNE` instruction performs a relative branch if the zero flag is not set.

```
BNE off(11)             // if Z = 0
                        // then PC <- PC+1 + sext(off(11))
```

#### BLT

The `BLT` instruction performs a relative branch if the signed comparison indicates strictly less than.

```
BLT off(11)             // if (N xor V) = 1
                        // then PC <- PC+1 + sext(off(11))
```

#### BGT

The `BGT` instruction performs a relative branch if the signed comparison indicates strictly greater than.

```
BGT off(11)             // if Z = 0 and (N xor V) = 0
                        // then PC <- PC+1 + sext(off(11))
```

#### BLE

The `BLE` instruction performs a relative branch if the signed comparison indicates less than or equal.

```
BLE off(11)             // if Z = 1 or (N xor V) = 1
                        // then PC <- PC+1 + sext(off(11))
```

#### BGE

The `BGE` instruction performs a relative branch if the signed comparison indicates greater than or equal.

```
BGE off(11)             // if (N xor V) = 0
                        // then PC <- PC+1 + sext(off(11))
```

### Flow control instructions

#### CALL

The `CALL` instruction saves the return address on the stack and then transfers control to an absolute 11-bit address in instruction memory.

```
CALL addr(11)           // SP <- SP - 1 ; MEM[SP] <- PC + 1 ; PC <- addr(11)
```

#### RET

The `RET` instruction restores the program counter from the stack and returns execution to the caller.

```
RET                     // PC <- MEM[SP] ; SP <- SP + 1
```

### Memory instructions

#### LOAD

The `LOAD` instruction reads a word from data memory using `BASE + offset` addressing. The effective address is computed as the contents of the base register plus the sign-extended 5-bit offset.

```
LOAD rd, off(5)[rb]     // rd <- MEM[rb + sext(off(5))]
```

#### STORE

The `STORE` instruction writes a word into data memory using `BASE + offset` addressing. The effective address is computed as the contents of the base register plus the sign-extended 5-bit offset.

```
STORE off(5)[rb], rs    // MEM[rb + sext(off(5))] <- rs
```

## 6. Instruction Encoding Table

| Instruction | Opcode | Func | Format |
| --- | --- | --- | --- |
| `NOP` | `00000` | - | No operand |
| `MOV` | `00001` | `00` | Register-register (RR) |
| `ADD` | `00001` | `01` | Register-register-register (RRR) |
| `SUB` | `00001` | `10` | Register-register-register (RRR) |
| `CMP` | `00001` | `11` | Register-register (RR) |
| `AND` | `00010` | `00` | Register-register-register (RRR) |
| `OR` | `00010` | `01` | Register-register-register (RRR) |
| `XOR` | `00010` | `10` | Register-register-register (RRR) |
| `NOT` | `00010` | `11` | Register-register (RR) |
| `LI` | `00011` | - | Register-immediate (RI) |
| `LIH` | `00100` | - | Register-immediate (RI) |
| `SLL` | `00101` | - | Register-register-immediate (RRI) |
| `SRL` | `00110` | - | Register-register-immediate (RRI) |
| `SRA` | `00111` | - | Register-register-immediate (RRI) |
| `JMP` | `01000` | - | Unconditional jump |
| `BEQ` | `01001` | - | Conditional branch |
| `BNE` | `01010` | - | Conditional branch |
| `BLT` | `01011` | - | Conditional branch |
| `BGT` | `01100` | - | Conditional branch |
| `BLE` | `01101` | - | Conditional branch |
| `BGE` | `01110` | - | Conditional branch |
| `CALL` | `01111` | - | Unconditional jump |
| `RET` | `10000` | - | No operand |
| `Reserved` | `10001` | - | - |
| `LOAD` | `10010` | - | Load |
| `STORE` | `10011` | - | Store |
| `ADDI` | `10100` | - | Register-immediate (RI) |
| `SUBI` | `10101` | - | Register-immediate (RI) |
| `CMPI` | `10110` | - | Register-immediate (RI) |
| `Reserved` | `10111` | - | - |
| `Reserved` | `11xxx` | - | - |