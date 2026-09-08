# AthenISA A32

A32 is the 32-bit base architecture of the AthenISA family. This page provides an index of its specification, extensions, and instruction set.

## Specification

| Chapter | Contents |
| --- | --- |
| [Overview](00_overview.md) | Architectural profile, scope, and relationship with A16 |
| [Registers](01_registers.md) | Register encodings, `PC`, `SP`, flags, and reset state |
| [Instruction set](02_instruction_set.md) | Programmer-visible behavior of every base instruction |
| [Instruction formats](03_instruction_formats.md) | Bit fields used by each instruction format |
| [Instruction encoding](04_instruction_encoding.md) | Opcode and function assignments |
| [Memory](05_memory.md) | Unified address space, memory accesses, control-flow targets, and stack behavior |

## Extensions

| Extension | Name | Instructions |
| --- | --- | --- |
| [`M`](extensions/M/00_overview.md) | Integer multiplication and division | - |
| [`F`](extensions/F/00_overview.md) | Floating-point | - |
| [`V`](extensions/V/00_overview.md) | Vector operations | - |
| [`B`](extensions/B/00_overview.md) | Bit manipulation | - |

An extension only defines the state, instructions, formats, and encodings added to the A32 base architecture. Extension documents are stored under [`extensions/`](extensions/).

## All supported instructions

| Category | Instructions | Extension |
| --- | --- | --- |
| No operation | [`NOP`](02_instruction_set.md#nop) | |
| Data movement | [`MOV`](02_instruction_set.md#mov), [`LI`](02_instruction_set.md#li), [`LIH`](02_instruction_set.md#lih) | |
| Arithmetic and comparison | [`ADD`](02_instruction_set.md#add), [`ADDI`](02_instruction_set.md#addi), [`SUB`](02_instruction_set.md#sub), [`CMP`](02_instruction_set.md#cmp), [`CMPI`](02_instruction_set.md#cmpi) | |
| Logic | [`AND`](02_instruction_set.md#and), [`ANDI`](02_instruction_set.md#andi), [`OR`](02_instruction_set.md#or), [`ORI`](02_instruction_set.md#ori), [`XOR`](02_instruction_set.md#xor), [`XORI`](02_instruction_set.md#xori), [`NOT`](02_instruction_set.md#not) | |
| Shifts | [`SLL`](02_instruction_set.md#sll), [`SLLI`](02_instruction_set.md#slli), [`SRL`](02_instruction_set.md#srl), [`SRLI`](02_instruction_set.md#srli), [`SRA`](02_instruction_set.md#sra), [`SRAI`](02_instruction_set.md#srai) | |
| Jumps and branches | [`JMP`](02_instruction_set.md#jmp), [`JMPR`](02_instruction_set.md#jmpr), [`BEQ`](02_instruction_set.md#beq), [`BNE`](02_instruction_set.md#bne), [`BLT`](02_instruction_set.md#blt), [`BGE`](02_instruction_set.md#bge), [`BLTU`](02_instruction_set.md#bltu), [`BGEU`](02_instruction_set.md#bgeu) | |
| Stack and calls | [`CALL`](02_instruction_set.md#call), [`CALLR`](02_instruction_set.md#callr), [`RET`](02_instruction_set.md#ret), [`PUSH`](02_instruction_set.md#push), [`POP`](02_instruction_set.md#pop) | |
| Memory | [`LDW`](02_instruction_set.md#ldw), [`STW`](02_instruction_set.md#stw), [`LDB`](02_instruction_set.md#ldb), [`STB`](02_instruction_set.md#stb) | |

Pseudo-instructions will be listed separately in the [A32 assembly reference](../asm/A32.md).
