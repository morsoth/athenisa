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

## Memory operands

Memory operands use base-plus-offset syntax:

```athe
LOAD  R1, 0[R2]
STORE -1[R7], R3
```

The offset is a signed 5-bit value.

## Labels

Labels mark instruction addresses.

```athe
loop:
    SUBI R1, 1
    CMPI R1, 0
    BNE loop
```

For branches, label operands are assembled as signed offsets relative to `PC + 1`.

For `JMP` and `CALL`, label operands are assembled as absolute instruction addresses.

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
STORE off5[rb], rs
```

## Pseudo-instructions

Pseudo-instructions are accepted by the assembler but are not real AthenISA instructions. They expand into one or more real instructions.

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