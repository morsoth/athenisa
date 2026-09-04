# AthenISA A32

A32 is the 32-bit base architecture of the AthenISA family. This page provides an index of its specification, extensions, and instruction set.

## Specification

| Chapter | Contents |
| --- | --- |
| [00 - Overview](00_overview.md) | Architectural profile, scope, and relationship with A16 |
| [01 - Registers](01_registers.md) | Register encodings, `PC`, `SP`, flags, and reset state |
| [02 - Instruction set](02_instruction_set.md) | Programmer-visible behavior of every base instruction |
| [03 - Instruction formats](03_instruction_formats.md) | Bit fields used by each instruction format |
| [04 - Instruction encoding](04_instruction_encoding.md) | Opcode and function assignments |
| [05 - Memory](05_memory.md) | Unified address space, memory accesses, control-flow targets, and stack behavior |

## Extensions

| Extension | Name | Instructions |
| --- | --- | --- |
| [`M`](extensions/M/00_overview.md) | Integer multiplication and division | - |
| [`F`](extensions/F/00_overview.md) | Floating-point | - |
| [`V`](extensions/V/00_overview.md) | Vector operations | - |

An extension only defines the state, instructions, formats, and encodings added to the A32 base architecture. Extension documents are stored under [`extensions/`](extensions/).

## All supported instructions

| Category | Instructions | Extension |
| --- | --- | --- |
| No operation | `NOP` | |
| Data movement | `MOV`, `LI`, `LIH` | |
| Arithmetic and comparison | `ADD`, `ADDI`, `SUB`, `SUBI`, `CMP`, `CMPI` | |
| Logic | `AND`, `OR`, `XOR`, `NOT` | |
| Shifts | `SLL`, `SRL`, `SRA` | |
| Jumps and branches | `JMP`, `JMPR`, `BEQ`, `BNE`, `BLT`, `BGE`, `BLTU`, `BGEU` | |
| Stack and calls | `CALL`, `CALLR`, `RET`, `PUSH`, `POP` | |
| Memory | `LDW`, `STW`, `LDB`, `STB` | |

Pseudo-instructions will be listed separately in the [A32 assembly reference](../asm/A32.md).
