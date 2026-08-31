# AthenISA Assembly Syntax

This document defines the source-language rules shared by AthenISA architecture profiles. Source files conventionally use the `.athe` extension.

Registers, instruction forms, encoded ranges, and pseudo-instructions are defined by the selected architecture profile:

- [A16 assembly reference](A16.md)
- [A32 assembly reference](A32.md)

## Source structure

An AthenISA source file is made of statements. Each non-empty line contains only one statement, and the assembler reads them from top to bottom. There are five statement types:

| Statement | Example | Purpose |
| --- | --- | --- |
| Section | `.code` | Selects whether following statements describe code or data |
| Instruction | `ADD R1, R2, R3` | Requests an operation from the selected architecture profile |
| Label | `loop:` | Gives a name to an instruction address |
| Constant | `limit: 10` | Gives a name to a numeric value |
| Data declaration | `data[1] 5` | Reserves and initializes data-memory words |

Blank lines and comments are ignored. A data declaration is the only statement that may continue onto another line.

## Sections

AthenISA source represents code and data with the `.code` and `.data` sections.

| Section | Accepted statements | Initial address |
| --- | --- | --- |
| `.code` | Instructions, labels, and constants | `0` |
| `.data` | Data declarations | `0` |

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

### Whitespace

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

In this example, `LI` is the mnemonic, `R1` is a register operand, and `0xF0` is an immediate value. The selected architecture profile defines which mnemonics, registers, operand forms, and values are accepted. Instruction behavior is defined by that architecture's specification.

## Numeric literals

Numeric values may be written in decimal, hexadecimal, or binary:

```athe
decimal: 42
hexadecimal: 0x2A
binary: 0b00101010
```

An optional `+` or `-` sign may precede a literal in any base:

```athe
positive: +10
negative: -0x4
```

Hexadecimal and binary prefixes may use either case (`0x`/`0X`, `0b`/`0B`). Digit separators are not supported. The selected architecture profile defines the evaluation range and how a value is converted to its destination field or data word.

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

A name followed by `:` with no value defines a label. Its value is the address of the next emitted instruction:

```athe
loop:
    SUBI R1, 1
    CMPI R1, 0
    BNE loop
```

Labels do not emit an instruction and may only be defined in `.code`. Their addresses include every real instruction emitted before them, including the expansion of pseudo-instructions.

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

Constants do not emit an instruction and may only be defined in `.code`. Their value may be an arithmetic expression:

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

Operators at the same precedence are evaluated from left to right. Parentheses may override the normal order. Division truncates toward zero. Overflow, division by zero, and remainder by zero are errors.

## Data declarations

A data declaration reserves and optionally initializes a block of consecutive data words. It may only appear in `.data` and uses the following form:

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

The selected architecture profile defines the width of each data word and the size of data memory.

### Size

The size is measured in words, must be greater than zero, and must fit together with preceding declarations in the target data memory. It may be a literal or an expression:

```athe
.code
rows: 2
columns: 3

.data
matrix[rows * columns] 0
```

Forward references are not accepted in data sizes.

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

Every initial value is stored as one target data word. The selected architecture profile defines how out-of-range values are handled.

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

The index is not restricted to the declared size, which permits calculations such as the address immediately after an array. The resulting address must fit the target data address space. The complete `name(index)` expression must remain contiguous when used as an instruction operand.

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
