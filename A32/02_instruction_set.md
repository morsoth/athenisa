# A32 Instruction Set

This chapter defines the architectural behavior of every real A32 instruction. Assembly-only pseudo-instructions will be defined separately in the [A32 assembly reference](../asm/A32.md).

In the instruction notation, `rd` denotes a destination register, `rs`, `rs1`, and `rs2` denote source registers, and `rb` denotes a base-address register. The prefixes `imm`, `addr`, and `off` identify immediate values, absolute addresses, and offsets, while the numeric suffix gives the field width. Detailed bit layouts are defined in [03_instruction_formats.md](03_instruction_formats.md).

By default the `FLAGS` register is unchanged.

## No-operation instructions

### NOP

`NOP` performs no operation.

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

`LI` loads a 16-bit immediate into the lower half of the destination register and clears the upper half to zero.

```text
LI rd, imm16                // rd[15:0] <- imm16
                            // rd[31:16] <- 0x0000
```

### LIH

`LIH` loads a 16-bit immediate into the upper half of the destination register while leaving the lower half unchanged.

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

`CMPI` compares a register with a zero-extended 16-bit immediate. The result is not written to the register file.

```text
CMPI rs, imm16              // rs - zext(imm16)
```

> [!NOTE]
> `CMPI` uses `zext(imm16)` because comparisons against negative immediate values are expected to be less common. Zero extension provides the full immediate range from 0 to 65,535. A negative value can still be loaded into a register and compared using `CMP`.

| Flag | Value |
| --- | --- |
| `Z` | `1` if the comparison result is zero; otherwise `0` |
| `C` | `1` if the subtraction requires no unsigned borrow; otherwise `0` |
| `N` | `1` if comparison result bit 31 is set; otherwise `0` |
| `V` | `1` if the subtraction produces signed overflow; otherwise `0` |

## Shift instructions

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

A five-bit shift amount represents every useful shift from 0 to 31 positions.

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 31 is set; otherwise `0` |
| `V` | `0` |

## Jump instructions

### JMP

`JMP` transfers execution to an absolute 26-bit instruction address.

```text
JMP addr26                  // PC <- addr26
```

### JMPR

`JMPR` transfers execution to the instruction address stored in a register. Only the low 26 bits of the register are used.

```text
JMPR rs                     // PC <- rs[25:0]
```

### BRA

`BRA` always branches by a signed offset relative to the instruction that follows it.

```text
BRA off26                   // PC <- PC + 1 + off26
```

### BEQ

`BEQ` branches when the zero flag is set.

```text
BEQ off26                   // if Z = 1
                            // then PC <- PC + 1 + off26
```

### BNE

`BNE` branches when the zero flag is clear.

```text
BNE off26                   // if Z = 0
                            // then PC <- PC + 1 + off26
```

### BLT

`BLT` branches when the previous comparison indicates signed less than.

```text
BLT off26                   // if (N xor V) = 1
                            // then PC <- PC + 1 + off26
```

### BGE

`BGE` branches when the previous comparison indicates signed greater than or equal.

```text
BGE off26                   // if (N xor V) = 0
                            // then PC <- PC + 1 + off26
```

### BLTU

`BLTU` branches when the previous comparison indicates unsigned less than.

```text
BLTU off26                  // if C = 0
                            // then PC <- PC + 1 + off26
```

### BGEU

`BGEU` branches when the previous comparison indicates unsigned greater than or equal.

```text
BGEU off26                  // if C = 1
                            // then PC <- PC + 1 + off26
```

The `off26` field is interpreted as a signed two's-complement value. `BGT`, `BLE`, `BGTU`, and `BLEU` are not provided because their conditions can be expressed by reversing the operands of `CMP`.

## Stack instructions

### CALL

`CALL` stores the sequential return address on the stack and transfers execution to an absolute 26-bit instruction address.

```text
CALL addr26                 // SP <- SP - 1
                            // MEM[SP] <- zext(PC + 1)
                            // PC <- addr26
```

### CALLR

`CALLR` stores the sequential return address on the stack and transfers execution to the instruction address stored in a register. The target is read before `SP` is modified.

```text
CALLR rs                    // target <- rs[25:0]
                            // SP <- SP - 1
                            // MEM[SP] <- zext(PC + 1)
                            // PC <- target
```

### RET

`RET` restores the program counter from the low 26 bits of the top stack word and then removes that entry.

```text
RET                         // PC <- MEM[SP][25:0]
                            // SP <- SP + 1
```

### PUSH

`PUSH` adds one register value to the stack.

```text
PUSH rs                     // SP <- SP - 1
                            // MEM[SP] <- rs
```

### POP

`POP` removes the top value from the stack and writes it to a register.

```text
POP rd                      // rd <- MEM[SP]
                            // SP <- SP + 1
```

The register operand of `PUSH` or `POP` must be one of `R0` through `R30`. `PUSH SP` and `POP SP` are illegal because these instructions already modify `SP` implicitly. Software must initialize `SP` and keep every stack access within its assigned memory region.

## Memory instructions

### LOAD

`LOAD` reads one 32-bit word from data memory into the destination register.

```text
LOAD rd, off16[rb]          // rd <- MEM[rb + sext(off16)]
```

### STORE

`STORE` writes one 32-bit source-register value to data memory.

```text
STORE off16[rb], rs         // MEM[rb + sext(off16)] <- rs
```
