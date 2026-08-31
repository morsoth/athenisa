# A16 Overview

This document introduces the architectural contract of A16. The following chapters define the complete programmer-visible state, binary encoding, instruction behavior, and memory model.

## Architectural profile

| Property | Definition |
| --- | --- |
| Instruction width | 16 bits |
| Data width | 16 bits |
| Instruction addresses | 11 bits, word-addressed |
| Data addresses | 16 bits, word-addressed |
| Register operands | `R0` to `R6` and `SP` |
| Instruction byte order | Little-endian |

A16 uses separate instruction and data address spaces. An instruction address selects one 16-bit instruction word; a data address selects one 16-bit data word. The base architecture has no byte load, byte store, or unaligned access.

## Specification chapters

1. [Registers and flags](01_registers.md)
2. [Instruction semantics](02_instruction_set.md)
3. [Instruction formats](03_instruction_formats.md)
4. [Opcode assignments](04_instruction_encoding.md)
5. [Memory and stack model](05_memory.md)

## Implementation independence

A16 does not prescribe how many cycles an instruction takes, whether the processor is pipelined or multicycle, how RAM is inferred, or when an external memory returns data. Such details do not change the architectural result of an instruction and belong to the processor implementation.
