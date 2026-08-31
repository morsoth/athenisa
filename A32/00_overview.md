# A32 Overview

This document introduces the architectural contract of A32. The following chapters define the complete programmer-visible state, binary encoding, instruction behavior, and memory model.

## Architectural profile

| Property | Definition |
| --- | --- |
| Instruction width | 32 bits |
| Data width | 32 bits |
| Instruction addresses | 26 bits, word-addressed |
| Data addresses | 32 bits, word-addressed |
| Register operands | `R0` to `R30` and `SP` |
| Instruction byte order | Little-endian |

A32 uses separate instruction and data address spaces. An instruction address selects one 32-bit instruction word; a data address selects one 32-bit data word. The base architecture has no byte load, byte store, or unaligned access.

## Implementation independence

A32 does not prescribe how many cycles an instruction takes, whether the processor is pipelined or multicycle, how RAM is implemented, or when an external memory returns data. Such details do not change the architectural result of an instruction and belong to the processor implementation.
