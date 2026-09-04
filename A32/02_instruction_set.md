# A32 Instruction Set

This chapter defines the architectural behavior of every real A32 instruction. Assembly-only pseudo-instructions will be defined separately in the [A32 assembly reference](../asm/A32.md).

In the instruction notation, `rd` denotes a destination register, `rs`, `rs1`, and `rs2` denote source registers, and `rb` denotes a base-address register. An `imm` operand is an immediate value, and its numeric suffix gives the number of meaningful bits for that instruction. Each instruction defines how the value is encoded and interpreted. Detailed bit layouts are defined in [03_instruction_formats.md](03_instruction_formats.md).

By default the `FLAGS` register is unchanged.

## No-operation instructions

### NOP

`NOP` performs no operation. Its `imm26` field is zero.

```text
NOP                         // does nothing
```

## Data movement instructions

### MOV

`MOV` copies the source register into the destination register.

```text
MOV rd, rs                  // rd <- rs
```

### LI

`LI` loads a 16-bit immediate into the lower half of the destination register and clears the upper half to zero. Bits `20:16` of the encoded `imm21` field must be zero.

```text
LI rd, imm16                // rd[15:0] <- imm16
                            // rd[31:16] <- 0x0000
```

### LIH

`LIH` loads a 16-bit immediate into the upper half of the destination register while leaving the lower half unchanged. Bits `20:16` of the encoded `imm21` field must be zero.

```text
LIH rd, imm16               // rd[31:16] <- imm16
```

`LI` followed by `LIH` can construct any 32-bit value. `LI` provides the low 16 bits and `LIH` provides the high 16 bits.

## Arithmetic and logic instructions

### ADD

`ADD` adds two source registers and writes the low 32 bits of the result to the destination register.

```text
ADD rd, rs1, rs2            // rd <- rs1 + rs2
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `1` if the addition produces a carry out of bit 31; otherwise `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `1` if the addition produces signed overflow; otherwise `0` |

### ADDI

`ADDI` adds a zero-extended 16-bit immediate to a source register and writes the result to a separate destination register.

```text
ADDI rd, rs, imm16          // rd <- rs + zext(imm16)
```

> [!NOTE]
> `ADDI` uses `zext(imm16)` rather than `sext(imm16)` because adding a negative value is equivalent to using `SUBI` with its positive magnitude, while zero extension provides the full immediate range from 0 to 65,535.

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `1` if the addition produces a carry out of bit 31; otherwise `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `1` if the addition produces signed overflow; otherwise `0` |

### SUB

`SUB` subtracts the second source register from the first and writes the low 32 bits of the result to the destination register.

```text
SUB rd, rs1, rs2            // rd <- rs1 - rs2
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `1` if the subtraction requires no unsigned borrow; otherwise `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `1` if the subtraction produces signed overflow; otherwise `0` |

### SUBI

`SUBI` subtracts a zero-extended 16-bit immediate from a source register and writes the result to a separate destination register.

```text
SUBI rd, rs, imm16          // rd <- rs - zext(imm16)
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `1` if the subtraction requires no unsigned borrow; otherwise `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `1` if the subtraction produces signed overflow; otherwise `0` |

### AND

`AND` performs a bitwise AND between two source registers.

```text
AND rd, rs1, rs2            // rd <- rs1 & rs2
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

### ANDI

`ANDI` performs a bitwise AND between a source register and a zero-extended 16-bit immediate.

```text
ANDI rd, rs, imm16          // rd <- rs & zext(imm16)
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

### OR

`OR` performs a bitwise OR between two source registers.

```text
OR rd, rs1, rs2             // rd <- rs1 | rs2
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

### ORI

`ORI` performs a bitwise OR between a source register and a zero-extended 16-bit immediate.

```text
ORI rd, rs, imm16           // rd <- rs | zext(imm16)
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

### XOR

`XOR` performs a bitwise XOR between two source registers.

```text
XOR rd, rs1, rs2            // rd <- rs1 ^ rs2
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

### XORI

`XORI` performs a bitwise XOR between a source register and a zero-extended 16-bit immediate.

```text
XORI rd, rs, imm16          // rd <- rs ^ zext(imm16)
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

### NOT

`NOT` inverts every bit of the source register.

```text
NOT rd, rs                  // rd <- ~rs
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

### CMP

`CMP` subtracts the second operand from the first only to update the flags. The result is not written to the register file.

```text
CMP rs1, rs2                // rs1 - rs2
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the comparison result is zero; otherwise `0` |
| `C` | `1` if the subtraction requires no unsigned borrow; otherwise `0` |
| `N` | `1` if comparison result bit 31 is set; otherwise `0` |
| `V` | `1` if the subtraction produces signed overflow; otherwise `0` |

### CMPI

`CMPI` compares a register with a zero-extended 21-bit immediate. The result is not written to the register file.

```text
CMPI rs, imm21              // rs - zext(imm21)
```

> [!NOTE]
> `CMPI` uses `zext(imm21)` because comparisons against negative immediate values are expected to be less common. Zero extension provides the full immediate range from 0 to 2,097,151. A negative value can still be loaded into a register and compared using `CMP`.

| Flag | Value |
| --- | --- |
| `Z` | `1` if the comparison result is zero; otherwise `0` |
| `C` | `1` if the subtraction requires no unsigned borrow; otherwise `0` |
| `N` | `1` if comparison result bit 31 is set; otherwise `0` |
| `V` | `1` if the subtraction produces signed overflow; otherwise `0` |

## Shift instructions

The shift amount occupies bits `4:0` of the encoded `imm16` field. Bits `15:5` must be zero, giving every shift instruction an effective `imm5` operand from 0 to 31.

### SLL

`SLL` performs a logical left shift of the source register by an immediate amount.

```text
SLL rd, rs, imm5            // rd <- rs << imm5
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

### SRL

`SRL` performs a logical right shift of the source register by an immediate amount.

```text
SRL rd, rs, imm5            // rd <- zext(rs) >> imm5
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

### SRA

`SRA` performs an arithmetic right shift of the source register by an immediate amount.

```text
SRA rd, rs, imm5            // rd <- sext(rs) >> imm5
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

## Jump instructions

### JMP

`JMP` always branches by a signed offset relative to the instruction that follows it.

```text
JMP imm26                   // PC <- PC + 4 + (sext(imm26) << 2)
```

### JMPR

`JMPR` transfers execution to the instruction address stored in a register.

```text
JMPR rs                     // PC <- rs
```

### BEQ

`BEQ` branches when the zero flag is set.

```text
BEQ imm26                   // if Z = 1
                            // then PC <- PC + 4 + (sext(imm26) << 2)
```

### BNE

`BNE` branches when the zero flag is clear.

```text
BNE imm26                   // if Z = 0
                            // then PC <- PC + 4 + (sext(imm26) << 2)
```

### BLT

`BLT` branches when the previous comparison indicates signed less than.

```text
BLT imm26                   // if (N xor V) = 1
                            // then PC <- PC + 4 + (sext(imm26) << 2)
```

### BGE

`BGE` branches when the previous comparison indicates signed greater than or equal.

```text
BGE imm26                   // if (N xor V) = 0
                            // then PC <- PC + 4 + (sext(imm26) << 2)
```

### BLTU

`BLTU` branches when the previous comparison indicates unsigned less than.

```text
BLTU imm26                  // if C = 0
                            // then PC <- PC + 4 + (sext(imm26) << 2)
```

### BGEU

`BGEU` branches when the previous comparison indicates unsigned greater than or equal.

```text
BGEU imm26                  // if C = 1
                            // then PC <- PC + 4 + (sext(imm26) << 2)
```

For these instructions, `imm26` is interpreted as a signed two's-complement offset measured in instructions and is shifted left by two before being added to the byte-addressed `PC`. `BGT`, `BLE`, `BGTU`, and `BLEU` are not provided because their conditions can be expressed by reversing the operands of `CMP`.

## Stack instructions

### CALL

`CALL` stores the sequential return address on the stack and transfers execution by a signed offset relative to the instruction that follows it.

```text
CALL imm26                  // SP <- SP - 4
                            // MEM32[SP] <- PC + 4
                            // PC <- PC + 4 + (sext(imm26) << 2)
```

### CALLR

`CALLR` stores the sequential return address on the stack and transfers execution to the instruction address stored in a register. The target is read before `SP` is modified.

```text
CALLR rs                    // SP <- SP - 4
                            // MEM32[SP] <- PC + 4
                            // PC <- rs
```

### RET

`RET` restores the program counter from the top stack word and then removes that entry. Its `imm26` field is zero.

```text
RET                         // PC <- MEM32[SP]
                            // SP <- SP + 4
```

### PUSH

`PUSH` adds one register value to the stack.

```text
PUSH rs                     // SP <- SP - 4
                            // MEM32[SP] <- rs
```

### POP

`POP` removes the top value from the stack and writes it to a register.

```text
POP rd                      // rd <- MEM32[SP]
                            // SP <- SP + 4
```

The register operand of `PUSH` or `POP` must be one of `R0` through `R30`. `PUSH SP` and `POP SP` are illegal because these instructions already modify `SP` implicitly. Software must initialize `SP` and keep every stack access within its assigned memory region.

## Memory instructions

### LDW

`LDW` reads one 32-bit word from a four-byte-aligned memory address.

```text
LDW rd, imm16[rb]           // rd <- MEM32[rb + sext(imm16)]
```

### STW

`STW` writes one 32-bit source-register value to a four-byte-aligned memory address.

```text
STW imm16[rb], rs           // MEM32[rb + sext(imm16)] <- rs
```

### LDB

`LDB` reads one byte from memory and zero-extends it to 32 bits.

```text
LDB rd, imm16[rb]           // rd <- zext(MEM8[rb + sext(imm16)])
```

### STB

`STB` writes the low eight bits of a source register to memory.

```text
STB imm16[rb], rs           // MEM8[rb + sext(imm16)] <- rs[7:0]
```

Memory offsets are measured in bytes. Byte accesses accept any address, while `LDW` and `STW` are illegal when their effective address is not a multiple of four.
