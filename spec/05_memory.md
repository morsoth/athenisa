# AthenISA Memory

AthenISA uses separate instruction and data address spaces. Both are word-addressed and use 16-bit words.

## Instruction memory

Instruction memory stores program code executed by the processor.

| Property | Value |
| --- | --- |
| Address width | 11 bits |
| Number of words | `2^11 = 2048` |
| Word width | 16 bits |
| Total size | 4096 bytes / 4 KB |
| Writable by ISA instructions | No |

The `PC` register constrains the instruction memory size to `2^11 = 2048` bits.

- `JMP` and `CALL` use absolute 11-bit instruction addresses.
- Conditional branches use signed 11-bit offsets relative to `PC+1`.

## Data memory

Data memory stores program data, stack contents, and memory-resident values.

| Property | Value |
| --- | --- |
| Address width | 16 bits |
| Number of words | `2^16 = 65536` |
| Word width | 16 bits |
| Total size | 131072 bytes / 128 KB |
| Access granularity | Word |

## Endianness

The system is little-endian.

Since the base ISA accesses memory at word granularity, byte-level endianness only becomes externally visible when memory images, tooling, or future byte-addressed features are introduced.

## Data-memory addressing mode

AthenISA supports base + offset addressing for data memory.

```athe
LOAD  rd, off5[rb]
STORE off5[rb], rs
```

The effective address is computed as:

```text
address <- rb + sext(off5)
```

The 5-bit offset is signed, allowing forward and backward accesses relative to the base register.

## Stack

The stack resides in the upper region of data memory.

The stack grows toward lower addresses.

`SP` points to the element currently located at the top of the stack.

After reset, a processor implementation is expected to initialize `SP` to the end of data memory unless a different boot convention is specified by the platform.

`CALL` saves the return address on the stack and transfers control to an absolute instruction-memory address.

```text
SP      <- SP - 1
MEM[SP] <- PC + 1
PC      <- addr11
```

`RET` loads the return address from the stack and restores execution at that address.

```text
PC <- MEM[SP]
SP <- SP + 1
```