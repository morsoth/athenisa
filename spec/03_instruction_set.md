# AthenISA Instruction Set

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
> Logical operations (`AND`, `OR`, `XOR`, `NOT`) do not have immediate versions because the 8-bit immediate available in `RI` instructions is not especially useful in a 16-bit architecture, particularly for full-width bit masks.

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