# AthenISA Assembly Syntax

This document defines the source language accepted by the reference AthenISA
assembler. Source files conventionally use the `.athe` extension.

For instruction behavior and binary encoding, see the
[instruction set](../spec/02_instruction_set.md) and
[encoding table](../spec/04_instruction_encoding.md).

## Source lines

After removing comments and surrounding whitespace, each non-empty source line
must contain exactly one of the following:

```text
label:
constant: value
instruction operands
```

A label or constant definition cannot share a line with an instruction.

```athe
start:
    LI R1, 10
```

The following form is not accepted:

```athe
start: LI R1, 10
```

The first emitted instruction is placed at address 0 and following instructions
are laid out consecutively in source order. The assembler does not insert startup
code or an implicit prologue. Sections, origin directives such as `.org`, include
files, and data-emission directives are not supported in the current source
language.

## Comments and whitespace

A semicolon starts a comment that continues to the end of the line:

```athe
; Full-line comment
LI R1, 42                 ; End-of-line comment
```

Blank lines are ignored. Spaces and tabs separate tokens. Commas between
instruction operands are optional, so these forms are equivalent:

```athe
ADD R1, R2, R3
ADD R1 R2 R3
ADD   R1,   R2,   R3
```

Memory operands are a single token and cannot contain spaces inside the
`off5[rb]` expression.

## Case sensitivity

Instruction mnemonics and register names are case-insensitive:

```athe
add r1, R2, r3
```

Symbol names are case-sensitive. `loop`, `Loop`, and `LOOP` are three different
symbols.

## Registers

Instructions may name `R0` through `R7`:

```athe
MOV R1, R2
```

`R0` is architecturally hardwired to zero. It is syntactically valid as either
a source or destination; writes to it are discarded by the processor.

## Numeric literals

The assembler accepts decimal, hexadecimal, and binary integers:

```athe
LI R1, 42
LI R2, 0x2A
LI R3, 0b00101010
```

An optional `+` or `-` sign may precede a literal in any base:

```athe
BEQ -1
LOAD R1, -0x4[R2]
ADDI R3, +10
```

Hexadecimal and binary prefixes may use either case (`0x`/`0X`, `0b`/`0B`). Digit separators are not supported. Arithmetic expressions are accepted only when defining [constants](#constants). Instruction operands must contain one literal or symbol. Parsed values must fit in a signed 32-bit integer before they are encoded.

## Encoded field ranges

| Field | Valid range | Used by |
| --- | --- | --- |
| `imm4` | 0 to 15 | Shifts |
| `imm8` | 0 to 255 | `LI`, `LIH`, `ADDI`, `SUBI`, `CMPI` |
| `imm16` | 0 to 65,535 | Pseudo-instruction `LDI` |
| `off5` | -16 to +15 | `LOAD`, `STORE` |
| `off11` | -1024 to +1023 | Conditional branches |
| `addr11` | 0 to 2047 | `JMP`, `CALL` |

If a value does not fit its destination field, the reference assembler prints a
warning and keeps the low field-width bits. It does not reject the instruction.
For example, `LI R1, 0x123` encodes `imm8 = 0x23`.

This truncation rule is mechanical. In particular, arithmetic `imm8` values are
zero-extended by the processor, so writing `ADDI R1, -1` encodes `0xFF` and adds
255; it is not a signed add-immediate operation.

## Symbols

Labels and constants share one numeric symbol table.

### Names

A symbol name must begin with an ASCII letter or `_`. Remaining characters may
be ASCII letters, digits, or `_`.

```athe
loop:
_start:
limit_1: 15
```

Names such as `1loop`, `bad-name`, and `bad$name` are invalid. Defining the same
name more than once is also an error.

### Labels

A definition without an explicit value creates a label at the current emitted
instruction address:

```athe
loop:
    SUBI R1, 1
    CMPI R1, 0
    BNE loop
```

The address counts real instruction words after pseudo-instruction expansion.
For example, `LDI` advances the current address by two words.

### Constants

A definition with a value creates a constant and emits no instruction:

```athe
limit: 0x0F
mask: 0b11110000
alias: limit
size: (limit + 1) * 2
negative_size: -size
```

A constant value may contain literals, previously defined symbols, parentheses, and the following integer operators:

| Precedence | Operators | Meaning |
| --- | --- | --- |
| Highest | unary `+`, unary `-` | Positive or negative value |
| Middle | `*`, `/`, `%` | Multiplication, integer division, remainder |
| Lowest | `+`, `-` | Addition, subtraction |

Operators at the same precedence are evaluated from left to right. Parentheses may override the normal order. Division truncates toward zero. All operations are checked as signed 32-bit integers; overflow, division by zero, and remainder by zero are errors.

Expressions may refer only to symbols defined earlier in the source. Forward references in constant definitions are not supported:

```athe
element_size: 2
array_size: element_size * 16
```

An expression cannot be written directly as an instruction operand. Define a constant first and use its name:

```athe
value: 10 + 4
LI R1, value              ; accepted
LI R1, 10 + 4             ; not accepted
```

### References

Symbols may replace numeric operands:

```athe
limit: 15
ADDI R1, limit
JMP finish

finish:
RET
```

The assembler collects all label addresses before parsing instructions, so
instructions may refer to labels defined later in the source.

## Control-flow operands

`JMP` and `CALL` always interpret their operand as an absolute `addr11`, whether
it is written as a number or a symbol:

```athe
JMP 0x120
CALL function
```

Conditional branches distinguish numeric and symbolic operands:

- a numeric operand is the raw signed `off11` to encode;
- a symbol is a target address, converted with
  `off11 = symbol - (PC + 1)`.

```athe
loop:
    BNE loop             ; assembler computes the relative offset

BEQ -1                   ; raw offset: branch to this instruction
BGE 4                    ; raw offset: four words after PC + 1
```

Every symbol used by a branch is treated as a target address, including symbols
defined with an explicit constant value.

## Memory operands

Loads and stores use a signed word offset followed by a base register in square
brackets:

```athe
LOAD  R1, 4[R2]
STORE -1[R7], R3
```

The offset may be a literal or symbol. A missing offset means zero:

```athe
LOAD  R1, [R2]           ; equivalent to LOAD R1, 0[R2]
STORE [R7], R3           ; equivalent to STORE 0[R7], R3
```

The complete memory expression must remain contiguous. `4 [R2]` is not accepted.

## Pseudo-instructions

Pseudo-instructions are assembler conveniences. They expand before encoding and
do not introduce new hardware operations.

| Pseudo-instruction | Expansion | Emitted words |
| --- | --- | --- |
| `LDI rd, imm16` | `LI rd, imm16[7:0]`<br>`LIH rd, imm16[15:8]` | 2 |
| `CLR rd` | `LI rd, 0` | 1 |
| `INC rd` | `ADDI rd, 1` | 1 |
| `DEC rd` | `SUBI rd, 1` | 1 |

Example:

```athe
LDI R1, 0x1234
```

emits:

```athe
LI  R1, 0x34
LIH R1, 0x12
```

Labels after an `LDI` account for both emitted words. The assembler's `.lst`
output displays these real expanded instructions.

## Instruction syntax reference

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

LDI  rd, imm16
CLR  rd
INC  rd
DEC  rd
```
