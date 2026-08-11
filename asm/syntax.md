# AthenISA Assembly Syntax

This document defines the source language accepted by the reference AthenISA assembler. Source files conventionally use the `.athe` extension.

For instruction behavior and binary encoding, see the [instruction set](../spec/02_instruction_set.md) and [encoding table](../spec/04_instruction_encoding.md).

## Source structure

An AthenISA source file is made of statements. Each non-empty line contains only one statement, and the assembler reads them from top to bottom. There are five statement types:

| Statement | Example | Purpose |
| --- | --- | --- |
| Section | `.code` | Selects whether following statements describe code or data |
| Instruction | `ADD R1, R2, R3` | Requests an operation to be executed by the processor |
| Label | `loop:` | Gives a name to an instruction address |
| Constant | `const: 10` | Gives a name to a numeric value |
| Data declaration | `data[1] 5` | Reserves and initializes data-memory words |

Blank lines and comments are ignored. A data declaration is the only statement that may continue onto another line.

## Sections

AthenISA keeps instructions and data in separate address spaces. Assembly source represents these spaces with the `.code` and `.data` sections.

| Section | Accepted statements | Initial address |
| --- | --- | --- |
| `.code` | Instructions, labels, and constants | `0x000` |
| `.data` | Data declarations | `0x0000` |

A section directive changes the active section for the statements that follow it. It must appear alone on its line and does not occupy memory:

```athe
.code
; Instructions, labels, and constants go here.

.data
; Data declarations go here.
```

By default the active section is `.code` when assembly begins. Programs that contain no data therefore do not need to write `.code` explicitly.

Sections may be selected any number of times. Code and data have independent address counters, and each counter resumes from its previous value when its section is selected again:

```athe
.code
NOP             ; code address 0

.data
value[1] 5      ; data address 0

.code
RET             ; code address 1
```

Using a statement in the wrong section is an error.

## General syntax rules

### Comments

A semicolon starts a comment that continues to the end of the line:

```athe
; Full-line comment
LI R1, 42                 ; End-of-line comment
```

### Whitespaces

Blank lines are ignored. Spaces and tabs separate tokens. Commas between instruction operands are optional, so these forms are equivalent:

```athe
ADD R1, R2, R3
ADD R1 R2 R3
ADD   R1,   R2,   R3
```

Commas are required between multiple data initializers:

```athe
vector[3] 1, 2, 3
```

### Case sensitivity

Instruction mnemonics, register names, and section directives are case-insensitive:

```athe
Add r1, R2, r3
.DATA
```

Symbol names are case-sensitive. `loop`, `Loop`, and `LOOP` are three different symbols.

## Instructions

An instruction starts with a mnemonic that identifies an operation. Any values needed by that operation follow as operands:

```athe
LI R1, 0xF0
```

In this example, `LI` is the mnemonic, `R1` is the destination register, and `0xF0` is an immediate value. Each instruction defines the type, number, order, and meaning of its own operands. See the [instruction syntax reference](#instruction-syntax-reference) for every instruction operand form. Consult the [AthenISA instruction set](../spec/02_instruction_set.md) for the architectural behavior of each instruction.

### Registers

Instructions may name `R0` through `R7`:

```athe
MOV R1, R2
```

`R0` is architecturally hardwired to zero. It is valid as either a source or destination, but the processor discards any value written to it.

### Numeric literals

Numeric operands may be written in decimal, hexadecimal, or binary:

```athe
LI R1, 42
LI R2, 0x2A
LI R3, 0b00101010
```

An optional `+` or `-` sign may precede a literal in any base:

```athe
BEQ -3
LOAD R1, -0x4[R2]
ADDI R3, +0b10
```

Hexadecimal and binary prefixes may use either case (`0x`/`0X`, `0b`/`0B`). Digit separators are not supported. Parsed values must fit in a signed 32-bit integer before being converted to their destination field or data word.

## Symbols

A symbol is a name associated with a numeric value. Labels, constants, and data names all use the same symbol table, so a name cannot be defined more than once.

A symbol name must begin with an ASCII letter or `_`. Remaining characters may be ASCII letters, digits, or `_`.

```athe
_start:
limit_1: 15
vector[4] 0
```

Names such as `1loop`, `bad-name`, and `bad$name` are invalid.

### Labels

A name followed by `:` with no value defines a label. Its value is the address of the next instruction word:

```athe
loop:
    SUBI R1, 1
    CMPI R1, 0
    BNE loop
```

Labels do not emit a word and may only be defined in `.code`. Their addresses count the real instruction words produced before them. This matters for pseudo-instructions that expand into more than one word.

A label must have its own line. The following form is not accepted:

```athe
loop: SUBI R1, 1
```

### Constants

A name followed by `:` and a value defines a constant:

```athe
limit: 15
mask: 0b11110000
```

Constants do not emit a word and may only be defined in `.code`. Their value may be an arithmetic expression:

```athe
element_size: 2
array_size: element_size * 16
```

Forward references are not supported in constant definitions.

## Expressions

An expression calculates an integer value during assembly. Expressions are accepted in constant definitions, data sizes, data initializers, and data indexes. They are not accepted directly as instruction operands. To use the result of an expression as an instruction operand, first assign it to a constant or use the `name(index)` data-address syntax:

```athe
LI  R1, 10 + 4             ; not accepted

value: 10 + 4
LI  R1, value              ; accepted
```

Expressions may contain parentheses and the following operators:

| Precedence | Operators | Meaning |
| --- | --- | --- |
| Highest | unary `+`, unary `-` | Positive or negative value |
| Middle | `*`, `/`, `%` | Multiplication, integer division, remainder |
| Lowest | `+`, `-` | Addition, subtraction |

Operators at the same precedence are evaluated from left to right. Parentheses may override the normal order. Division truncates toward zero. All operations are checked as signed 32-bit integers; overflow, division by zero, and remainder by zero are errors.

## Data declarations

A data declaration reserves and optionally initializes a block of consecutive 16-bit words. It may only appear in `.data` and uses the following form:

```text
name[size] values
```

| Part | Meaning |
| --- | --- |
| `name` | Defines a data symbol for the address of the first reserved word |
| `size` | Specifies how many words to reserve |
| `values` | Optionally specifies the initial contents of those words |

For example:

```athe
.data
item[1] 9
vector[5] 0, 1, 2, 3, 4
```

### Size

The size is measured in words, must be greater than zero, and must fit together with the preceding declarations in the 65,536-word data memory. It may be a literal or an expression:

```athe
.code
rows: 2
columns: 3

.data
matrix[rows * columns] 0
```

Forward references are not accepted in a data sizes.

### Initialization

The `values` list determines the initial contents of the reserved words:

| Initial values | Result |
| --- | --- |
| None | Every reserved word is initialized to zero |
| One | The value is repeated in every reserved word |
| Exactly `size` | Values are stored in source order |
| Any other count | Error |

```athe
.data

empty[4]                    ; 0, 0, 0, 0
filled[4] 7                 ; 7, 7, 7, 7
vector[4] 1, 2, 3, 4        ; 1, 2, 3, 4
```

Multiple initial values must be separated by commas. A list may continue on the following line when the previous line ends with a comma:

```athe
matrix[6] 0, 1, 2,
          3, 4, 5
```

Each initial value may be an expression containing literals or symbols. Initializers can refer to symbols defined later:

```athe
pointer[1] vector
vector[3] 10, 20, 30
```

Every initial value is stored as one 16-bit word. If a value does not fit, the assembler prints a warning and stores its low 16 bits.

### Addresses and indexes

Every declaration defines a data symbol whose value is the address of its first reserved word. It represents an address, not the value stored at that address:

```athe
.data
first[2] 10, 20
second[1] 30
```

In this example, `first` has value `0` and `second` has value `2` because the two words belonging to `first` occupy addresses 0 and 1.

`name(index)` calculates the address of another word relative to the start of a declaration:

```text
name(index) = name + index
```

The index is an expression evaluated by the assembler and measured in words. This syntax only calculates an address; it does not read data memory:

```athe
.code
LDI  R5, vector(3)
LOAD R1, [R5]
```

The index is not restricted to the declared size, which permits calculations such as the address immediately after an array. The resulting address must fit in the 16-bit data address space. The complete `name(index)` expression must remain contiguous when used as an instruction operand.

## Symbol references

A symbol may generally be used anywhere the assembler accepts a numeric value. Symbols are untyped, so the assembler does not reject a code address used as data or a data address used as code.

There are two exceptions: data sizes may only reference constants, and `name(index)` requires `name` to be a data symbol.

Whether a symbol can be used before its definition depends on the context:

| Context | Forward reference | Example |
| --- | --- | --- |
| Instruction operand | Accepted | `JMP fwd_ref` |
| Data index | Accepted | `LDI R1, data(fwd_ref+1)` |
| Data initializer | Accepted | `data[1] fwd_ref` |
| Data size | Not accepted | `data[fwd_ref] 0` |
| Constant definition | Not accepted | `const: fwd_ref + 1` |

Instruction operands, data indexes, and data initializers are evaluated after the complete symbol table has been collected. Data sizes and constant expressions are evaluated while addresses are being calculated, so they may only use symbols already known at that point.

## Special operands

### Memory operands

`LOAD` and `STORE` address data memory using a base register (`rb`) and a signed word offset (`off5`):

```text
address = rb + off5
```

The assembly syntax writes the offset immediately before the base register (`off5[rb]`):

```athe
LOAD  R1, 4[R2]
STORE -1[R7], R3
```

The offset may be a literal or symbol. Omitting it means zero:

```athe
LOAD  R1, [R2]           ; equivalent to LOAD R1, 0[R2]
STORE [R7], R3           ; equivalent to STORE 0[R7], R3
```

The complete memory operand is one token and cannot contain spaces. `4 [R2]` is not accepted.

An arbitrary 16-bit data address normally has to be loaded into a register because it does not fit in the 5-bit offset:

```athe
LDI  R5, result
LOAD R1, [R5]
```

### Control-flow operands

`JMP` and `CALL` use absolute instruction addresses. A numeric operand is used directly, and a symbol contributes its numeric address:

```athe
JMP 0x120
CALL function
```

Conditional branches use relative offsets measured from the instruction after the branch. Numeric and symbolic operands are interpreted differently:

- A numeric operand is the signed `off11` encoded directly.
- A symbol is a target instruction address, converted with `off11 = symbol - (PC + 1)`.

```athe
loop:
    BNE loop             ; assembler computes the relative offset

BEQ -1                   ; execute this branch again
BGE 4                    ; target is four words after PC + 1
```

Every symbol used by a branch is treated as a target address, including constants and data symbols.

## Encoded field ranges

Instruction fields have fixed widths. The following table shows the values that fit without truncation:

| Field | Valid range | Used by |
| --- | --- | --- |
| `imm4` | 0 to 15 | Shifts |
| `imm8` | 0 to 255 | `LI`, `LIH`, `ADDI`, `SUBI`, `CMPI` |
| `imm16` | 0 to 65,535 | Pseudo-instruction `LDI` |
| `off5` | -16 to +15 | `LOAD`, `STORE` |
| `off11` | -1024 to +1023 | Conditional branches |
| `addr11` | 0 to 2047 | `JMP`, `CALL` |

If a value does not fit its field, the assembler prints a warning, keeps the low field-width bits, and continues. For example, `LI R1, 0x123` encodes `imm8 = 0x23`.

Truncation does not change how the processor interprets the remaining bits. Arithmetic `imm8` values are zero-extended, so `ADDI R1, -1` encodes `0xFF` and adds 255; it does not perform a signed addition of -1.

## Pseudo-instructions

Pseudo-instructions are convenient assembly operations built from real AthenISA instructions. The assembler expands them before generating machine code; they do not require additional processor hardware.

| Pseudo-instruction | Expansion | Emitted words |
| --- | --- | --- |
| `LDI rd, imm16` | `LI rd, imm16[7:0]`<br>`LIH rd, imm16[15:8]` | 2 |
| `CLR rd` | `LI rd, 0` | 1 |
| `INC rd` | `ADDI rd, 1` | 1 |
| `DEC rd` | `SUBI rd, 1` | 1 |

For example:

```athe
LDI R1, 0x1234
```

emits these two real instructions:

```athe
LI  R1, 0x34
LIH R1, 0x12
```

A label after `LDI` accounts for both emitted words. The assembler's `.lst` output displays the expanded real instructions.

## Complete example

See [`tools/examples/original.athe`](../tools/examples/original.athe) for a complete program that combines constants, code and data sections, labels, data addresses, control flow, and pseudo-instructions. The generated assembler and disassembler outputs are available in the same directory.

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
