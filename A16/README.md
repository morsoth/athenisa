# AthenISA A16

A16 is the 16-bit base architecture of the AthenISA family. This page provides an index of its specification, extensions, and instruction set.

## Specification

| Chapter | Contents |
| --- | --- |
| [00 - Overview](00_overview.md) | Architectural profile, scope, and implementation boundaries |
| [01 - Registers](01_registers.md) | Register encodings, `PC`, `SP`, flags, and reset state |
| [02 - Instruction set](02_instruction_set.md) | Programmer-visible behavior of every base instruction |
| [03 - Instruction formats](03_instruction_formats.md) | Bit fields used by each instruction format |
| [04 - Instruction encoding](04_instruction_encoding.md) | Opcode and function assignments |
| [05 - Memory](05_memory.md) | Address spaces, control-flow targets, data addressing, and stack behavior |

## Extensions

| Extension | Name | Instructions |
| --- | --- | --- |
| [`M`](extensions/M/00_overview.md) | Integer multiplication and division | `MUL`, `DIV` |
| [`F`](extensions/F/00_overview.md) | Floating-point | - |

An extension only defines the state, instructions, formats, and encodings added to the A16 base architecture. Extension documents are stored under [`extensions/`](extensions/).

## All supported instructions

| Category | Instructions | Extension |
| --- | --- | --- |
| No operation | `NOP` | |
| Data movement | `MOV`, `LI`, `LIH` | |
| Arithmetic and comparison | `ADD`, `ADDI`, `SUB`, `SUBI`, `CMP`, `CMPI` | |
| Logic | `AND`, `OR`, `XOR`, `NOT` | |
| Shifts | `SLL`, `SRL`, `SRA` | |
| Jumps and branches | `JMP`, `JMPR`, `BRA`, `BEQ`, `BNE`, `BLT`, `BGE`, `BLTU`, `BGEU` | |
| Stack and calls | `CALL`, `CALLR`, `RET`, `PUSH`, `POP` | |
| Memory | `LOAD`, `STORE` | |
| Multiplication and division | `MUL`, `DIV` | `M` |

Pseudo-instructions are assembly conveniences and are listed separately in the [A16 assembly reference](../asm/A16.md#pseudo-instructions).
