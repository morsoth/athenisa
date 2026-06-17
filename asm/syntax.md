# AthenISA Assembly Syntax

AthenISA assembly source files use the `.athe` extension.

## Comments

Comments start with `;` and continue until the end of the line.

```athe
; this is a comment
LI R1, 42   ; this is also a comment
```

## Whitespace

Whitespace is not significant except as a separator between tokens.

These are equivalent:

```athe
ADD R1,R2,R3
ADD R1, R2, R3
ADD   R1,   R2,   R3
ADD R1 R2 R3
```

## Registers

General registers are written as:

```athe
R0
R1
R2
R3
R4
R5
R6
R7
```

Register names are case-insensitive in the reference assembler.

## Immediates

Decimal:

```athe
LI R1, 42
```

Hexadecimal:

```athe
LI R1, 0x2A
```

Binary:

```athe
LI R1, 0b00101010
```

Negative decimal immediates are allowed for signed operands such as branch offsets and memory offsets:

```athe
STORE -1[R7], R1
```

If a numeric value does not fit in the destination field, the assembler emits a warning and truncates the value to the encoded bit width.

For example, `LI R1, 0x123` warns and encodes the low 8 bits, `0x23`.

Symbols may also be used wherever a numeric operand is accepted:

```athe
limit: 15
ADDI R1, limit
```

## Memory operands

Memory operands use base + offset syntax:

```athe
LOAD  R1, 0[R2]
STORE -1[R7], R3
```

The offset is a signed 5-bit value.

When the offset is zero, it may be omitted:

```athe
LOAD  R1, [R2]    ; equivalent to LOAD R1, 0[R2]
STORE [R7], R3    ; equivalent to STORE 0[R7], R3
```

## Labels

Symbols are defined with `:`.

Each source line may contain only one kind of item: a label, a constant, or an instruction.

When no value follows the symbol name, the symbol receives the current instruction address:

```athe
loop:
    SUBI R1, 1
    CMPI R1, 0
    BNE loop
```

When a value follows the symbol name, the symbol receives that value and does not consume instruction memory:

```athe
limit: 0x0F
mask: 0b11110000
```

All symbols are numeric. The assembler does not distinguish labels from constants after the first pass.

For branches, symbolic operands are assembled as signed offsets relative to `PC + 1`.

Numeric branch operands are treated as raw signed offsets.

For `JMP` and `CALL`, label operands are assembled as absolute instruction addresses.

## Planned pseudo-instructions

Pseudo-instructions are planned for the assembler but are not real AthenISA instructions. They will expand into one or more real instructions.

### `LDI`

Loads a full 16-bit immediate.

```athe
LDI R1, 0x1234
```

Expands to:

```athe
LI  R1, 0x34
LIH R1, 0x12
```

### `CLR`

Clears a register.

```athe
CLR R1
```

Expands to:

```athe
MOV R1, R0
```

### `INC`

Increments a register by one.

```athe
INC R1
```

Expands to:

```athe
ADDI R1, 1
```

### `DEC`

Decrements a register by one.

```athe
DEC R1
```

Expands to:

```athe
SUBI R1, 1
```

## Instruction syntax summary

```athe
NOP
RET

MOV  rd, rs
ADD  rd, rs1, rs2
SUB  rd, rs1, rs2
CMP  rd, rs
AND  rd, rs1, rs2
OR   rd, rs1, rs2
XOR  rd, rs1, rs2
NOT  rd, rs

LI   rd, imm8
LIH  rd, imm8
ADDI rd, imm8
SUBI rd, imm8
CMPI rd, imm8

SLL  rd, rs, imm4
SRL  rd, rs, imm4
SRA  rd, rs, imm4

JMP  addr11
CALL addr11
BEQ  off11
BNE  off11
BLT  off11
BGT  off11
BLE  off11
BGE  off11

LOAD  rd, off5[rb]
LOAD  rd, [rb]
STORE off5[rb], rs
STORE [rb], rs
```
