# A16 Overview

This document introduces the architectural contract of A16. The following chapters define the complete programmer-visible state, binary encoding, instruction behavior, and memory model.

## Architectural profile

| Property | Definition |
| --- | --- |
| Instruction width | 16 bits |
| Data width | 16 bits |
| Memory addresses | 16 bits, byte-addressed |
| Register operands | `R0` to `R6` and `SP` |
| Byte order | Little-endian |

A16 uses one unified 64 KiB address space for instructions and data. Instructions occupy two bytes and must be aligned to a two-byte boundary. The base architecture supports byte and 16-bit word accesses; word accesses must also be two-byte aligned.

## Implementation independence

A16 does not prescribe how many cycles an instruction takes, whether the processor is pipelined or multicycle, how RAM is inferred, or when an external memory returns data. Such details do not change the architectural result of an instruction and belong to the processor implementation.
