# A32 Memory and Addressing

A32 uses separate instruction and data address spaces. Both spaces are word-addressed and contain 32-bit words.

| Property | Instruction memory | Data memory |
| --- | --- | --- |
| Address width | 26 bits | 32 bits |
| Address range | `0x0000000` to `0x3FFFFFF` | `0x00000000` to `0xFFFFFFFF` |
| Number of words | `2^26` | `2^32` |
| Word width | 32 bits | 32 bits |
| Byte capacity | 256 MiB | 16 GiB |
| ISA access | Instruction fetch | `LOAD`, `STORE`, `CALL`, `CALLR`, `RET`, `PUSH`, `POP` |

Each address selects one complete 32-bit word. Address `0` selects the first word and address `1` selects the following word; neither address identifies an individual byte.

The instruction space is read-only from the perspective of A32 instructions. Loading a program into it is a platform responsibility performed before normal execution or through an external programming interface. An implementation may provide less physical memory than the architectural address space, but the platform must define which address ranges are available.

## Data memory addressing

`LOAD` and `STORE` add the signed `off16` field to the base register `rb`:

```text
d_addr = rb + sext(off16)
```

The offset is measured in 32-bit words and has a range from `-32,768` to `+32,767`. The calculation uses 32-bit wrapping arithmetic.

Assembly represents these operands as follows:

```athe
LOAD  R1, 4[R2]             ; load from address R2 + 4
STORE -1[R30], R3           ; store at address R30 - 1
```

## Instruction memory addressing

The program counter contains the 26-bit word address of the current instruction. Sequential execution advances to `PC + 1`, wrapping from `0x3FFFFFF` to `0x0000000`.

### Absolute targets

`JMP` and `CALL` replace the program counter with their `addr26` field:

```text
PC = addr26
```

The field can target any word in instruction memory.

### Register targets

`JMPR` and `CALLR` take their target from the low 26 bits of a source register:

```text
PC = rs[25:0]
```

The upper six bits of the source register do not affect the target address.

### Relative branch targets

Relative branches add a signed 26-bit offset to the address of the following instruction:

```text
PC = PC + 1 + off26
```

The `off26` field has a signed range from `-33,554,432` to `+33,554,431` instructions. The calculation uses 26-bit wrapping arithmetic.

> [!WARNING]
> An `off26` value of `-1` targets the branch instruction itself (`PC + 1 - 1 = PC`). `BRA -1` therefore repeats indefinitely. A taken conditional branch with the same offset also repeats while its condition remains satisfied.

## Stack

The stack resides in data memory and grows toward lower addresses. `SP` identifies the most recently stored stack word, which may contain a return address or a value saved by software. Register encoding `11111` allows software to read and write `SP` directly.

Software chooses the initial value and valid memory region of each stack. After reset, `SP` contains `0x00000000`, but this value does not indicate an empty or initialized stack.

`CALL` and `CALLR` decrement `SP` before storing the zero-extended return address:

```text
SP       = SP - 1
DMEM[SP] = zext(PC + 1)
```

`RET` reads the current top entry and then increments `SP`:

```text
PC = DMEM[SP][25:0]
SP = SP + 1
```

`PUSH` and `POP` use the same convention:

```text
SP       = SP - 1           // PUSH
DMEM[SP] = rs

rd = DMEM[SP]               // POP
SP = SP + 1
```

Stack entries are untyped. Software must balance values placed above a return address before executing `RET`. A32 does not track a stack base, limit, depth, or empty state, so software is responsible for preventing stack underflow and overflow.

## Byte order and serialized programs

A32 uses little-endian byte order. Base-architecture memory accesses transfer complete 32-bit words, so byte order is not observable through `LOAD` or `STORE`.

When an assembler generates a raw binary image, each 32-bit instruction is serialized from its least significant byte to its most significant byte. Text representations containing complete instruction words have no byte-order ambiguity.

## Timing and physical memory

The ISA does not require combinational or synchronous reads, a particular RAM primitive, or a fixed access latency. A conforming implementation may use FPGA block RAM, external memory, or another storage system as long as completed instructions produce the behavior defined by the ISA.
