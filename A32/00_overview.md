# A32 Overview

This document introduces the architectural contract of A32. The following chapters define the complete programmer-visible state, binary encoding, instruction behavior, and memory model.

## Architectural profile

| Property | Definition |
| --- | --- |
| Instruction width | 32 bits |
| Data width | 32 bits |
| Memory addresses | 32 bits, byte-addressed |
| Register operands | `R0` to `R30` and `SP` |
| Byte order | Little-endian |

A32 uses one unified 4 GiB address space for instructions and data. Instructions occupy four bytes and must be aligned to a four-byte boundary. The base architecture supports byte and 32-bit word accesses; word accesses must also be four-byte aligned.

## Implementation independence

A32 does not prescribe how many cycles an instruction takes, whether the processor is pipelined or multicycle, how RAM is implemented, or when an external memory returns data. Such details do not change the architectural result of an instruction and belong to the processor implementation.
