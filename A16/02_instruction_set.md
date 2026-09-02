# A16 Instruction Set

This chapter defines the architectural behavior of every real A16 instruction. Assembly-only pseudo-instructions are defined separately in the [A16 assembly reference](../asm/A16.md#pseudo-instructions).

In the instruction notation, `rd` denotes a destination register, `rs`, `rs1`, and `rs2` denote source registers, and `rb` denotes a base-address register. `imm` identifies an encoded immediate field, while its numeric suffix gives the field width. Each instruction defines whether that field represents a value, an address, or an offset. Detailed bit layouts are defined in [03_instruction_formats.md](03_instruction_formats.md).

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

`LI` loads an 8-bit immediate into the lower half of the destination register and clears the upper half to zero. This instruction is used to efficiently load small constants.

```text
LI rd, imm8                 // rd[7:0] <- imm8
                            // rd[15:8] <- 0x00
```

### LIH

`LIH` loads an 8-bit immediate into the upper half of the destination register while leaving the lower half unchanged. Combined with LI, it allows constructing a full 16-bit constant.

```text
LIH rd, imm8                // rd[15:8] <- imm8
```

> [!NOTE]
> To successfully load a 16-bit immediate into a register it is mandatory to use `LI` first and then `LIH`.

## Arithmetic and logic instructions

### ADD

`ADD` adds two source registers and writes the result to the destination register.

```text
ADD rd, rs1, rs2            // rd <- rs1 + rs2
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `1` if the addition produces a carry out of bit 15; otherwise `0` |
| `N` | `1` if result bit 15 is set; otherwise `0` |
| `V` | `1` if the addition produces signed overflow; otherwise `0` |

### ADDI

`ADDI` adds a zero-extended 8-bit immediate to the destination register.

```text
ADDI rd, imm8               // rd <- rd + zext(imm8)
```

> [!NOTE]
> `ADDI` uses `zext(imm8)` rather than `sext(imm8)` because adding a negative value would be equivalent to using `SUBI` with its positive magnitude, while zero extension allows `ADDI` to use the full unsigned immediate range from 0 to 255.

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `1` if the addition produces a carry out of bit 15; otherwise `0` |
| `N` | `1` if result bit 15 is set; otherwise `0` |
| `V` | `1` if the addition produces signed overflow; otherwise `0` |

### SUB

`SUB` subtracts the second source register from the first and writes the result to the destination register.

```text
SUB rd, rs1, rs2            // rd <- rs1 - rs2
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `1` if the subtraction requires no unsigned borrow; otherwise `0` |
| `N` | `1` if result bit 15 is set; otherwise `0` |
| `V` | `1` if the subtraction produces signed overflow; otherwise `0` |

### SUBI

`SUBI` subtracts a zero-extended 8-bit immediate from the destination register.

```text
SUBI rd, imm8               // rd <- rd - zext(imm8)
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `1` if the subtraction requires no unsigned borrow; otherwise `0` |
| `N` | `1` if result bit 15 is set; otherwise `0` |
| `V` | `1` if the subtraction produces signed overflow; otherwise `0` |

### AND

`AND` performs a bitwise AND between two source registers.

```text
AND rd, rs1, rs2            // rd <- rs1 & rs2
```

> [!NOTE]
> Logical operations (`AND`, `OR`, `XOR`, `NOT`) do not have immediate versions because the 8-bit immediate available in [`RI` instructions](./03_instruction_formats.md#register-immediate-ri) is not especially useful in a 16-bit architecture, particularly for full-width bit masks.

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 15 is set; otherwise `0` |
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
| `N` | `1` if result bit 15 is set; otherwise `0` |
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
| `N` | `1` if result bit 15 is set; otherwise `0` |
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
| `N` | `1` if result bit 15 is set; otherwise `0` |
| `V` | `0` |

### CMP

`CMP` performs a register subtraction only to update the flags. The arithmetic result is not written to the register file.

```text
CMP rd, rs                  // rd - rs
```

| Flag | Value |
| --- | --- |
| `Z` | `1` if the comparison result is zero; otherwise `0` |
| `C` | `1` if the subtraction requires no unsigned borrow; otherwise `0` |
| `N` | `1` if comparison result bit 15 is set; otherwise `0` |
| `V` | `1` if the subtraction produces signed overflow; otherwise `0` |

### CMPI

`CMPI` compares a register with a zero-extended 8-bit immediate. The arithmetic result is not written to the register file.

```text
CMPI rd, imm8               // rd - zext(imm8)
```

> [!NOTE]
> `CMPI` uses `zext(imm8)` because comparisons against negative immediate values are expected to be less common. Zero extension gives the instruction the full unsigned immediate range from 0 to 255. A negative value can still be loaded into a register and compared using `CMP`.

| Flag | Value |
| --- | --- |
| `Z` | `1` if the comparison result is zero; otherwise `0` |
| `C` | `1` if the subtraction requires no unsigned borrow; otherwise `0` |
| `N` | `1` if comparison result bit 15 is set; otherwise `0` |
| `V` | `1` if the subtraction produces signed overflow; otherwise `0` |

## Shift instructions

### SLL

`SLL` performs a logical left shift of the source register by an immediate amount.

```text
SLL rd, rs, imm5            // rd <- rs << imm5
```

> [!NOTE]
> Shift instructions use a 5-bit immediate with a range from 0 to 31. For shift amounts from 16 to 31, `SLL` and `SRL` produce zero, while `SRA` produces either `0x0000` or `0xFFFF` according to the original sign bit.

| Flag | Value |
| --- | --- |
| `Z` | `1` if the result is zero; otherwise `0` |
| `C` | `0` |
| `N` | `1` if result bit 15 is set; otherwise `0` |
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
| `N` | `1` if result bit 15 is set; otherwise `0` |
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
| `N` | `1` if result bit 15 is set; otherwise `0` |
| `V` | `0` |

## Jump instructions

### JMP

`JMP` transfers execution to an absolute 11-bit instruction address.

```text
JMP imm11                   // PC <- imm11
```

### JMPR

`JMPR` transfers execution to the instruction address stored in a register. Only the low 11 bits of the register are used.

```text
JMPR rs                     // PC <- rs[10:0]
```

### BRA

`BRA` always branches by a signed offset relative to the instruction that follows it.

```text
BRA imm11                   // PC <- PC + 1 + sext(imm11)
```

### BEQ

`BEQ` branches when the zero flag is set.

```text
BEQ imm11                   // if Z = 1
                            // then PC <- PC + 1 + sext(imm11)
```

### BNE

`BNE` branches when the zero flag is clear.

```text
BNE imm11                   // if Z = 0
                            // then PC <- PC + 1 + sext(imm11)
```

### BLT

`BLT` branches when the previous comparison indicates signed less than.

```text
BLT imm11                   // if (N xor V) = 1
                            // then PC <- PC + 1 + sext(imm11)
```

### BGE

`BGE` branches when the previous comparison indicates signed greater than or equal.

```text
BGE imm11                   // if (N xor V) = 0
                            // then PC <- PC + 1 + sext(imm11)
```

### BLTU

`BLTU` branches when the previous comparison indicates unsigned less than.

```text
BLTU imm11                  // if C = 0
                            // then PC <- PC + 1 + sext(imm11)
```

### BGEU

`BGEU` branches when the previous comparison indicates unsigned greater than or equal.

```text
BGEU imm11                  // if C = 1
                            // then PC <- PC + 1 + sext(imm11)
```

> [!NOTE]
> `BGT`, `BLE`, `BGTU`, and `BLEU` are not provided because their conditions can be expressed by reversing the operands of `CMP`. For example, `CMP R2, R1` followed by `BLT` tests whether `R1 > R2`, while the same comparison followed by `BGE` tests whether `R1 <= R2`. `BLTU` and `BGEU` provide the equivalent unsigned cases.

## Stack instructions

### CALL

`CALL` stores the sequential return address on the stack and transfers execution to an absolute 11-bit instruction address.

```text
CALL imm11                  // SP <- SP - 1
                            // MEM[SP] <- PC + 1
                            // PC <- imm11
```

### CALLR

`CALLR` stores the sequential return address on the stack and transfers execution to the instruction address stored in a register. The target is read before `SP` is modified, and only its low 11 bits are used.

```text
CALLR rs                    // target <- rs[10:0]
                            // SP <- SP - 1
                            // MEM[SP] <- PC + 1
                            // PC <- target
```

### RET

`RET` restores the program counter from the stack and then removes that entry.

```text
RET                         // PC <- MEM[SP]
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

The register operand of `PUSH` or `POP` must be one of `R0` through `R6`. `PUSH SP` and `POP SP` are illegal because these instructions already modify `SP` implicitly.

Software must initialize `SP` and keep every stack access within its assigned memory region. A16 does not detect an empty stack, stack underflow, or stack overflow.

## Memory instructions

### LOAD

`LOAD` reads one 16-bit word from data memory into the destination register.

```text
LOAD rd, imm5[rb]           // rd <- MEM[rb + sext(imm5)]
```

### STORE

`STORE` writes one 16-bit source-register value to data memory.

```text
STORE imm5[rb], rs          // MEM[rb + sext(imm5)] <- rs
```
