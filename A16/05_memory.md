# A16 Memory and Addressing

A16 uses one unified, byte-addressable memory space for instructions, data, and the stack.

| Property | Definition |
| --- | --- |
| Address width | 16 bits |
| Address range | `0x0000` to `0xFFFF` |
| Capacity | 64 KiB |
| Instruction size | 2 bytes |
| Data word size | 2 bytes |
| Byte order | Little-endian |

Every address identifies one byte. Instructions and data share the same addresses, so the base architecture does not prevent a store from modifying memory that contains instructions. A platform may add access permissions, but they are outside the A16 base specification.

## Memory accesses

`LDW`, `STW`, `LDB`, and `STB` calculate an effective address by adding the signed `imm5` field to the base register `rb`:

```text
address = rb + sext(imm5)
```

The offset is measured in bytes and has a range from `-16` to `+15`. Address arithmetic wraps to 16 bits.

| Instruction | Access | Result |
| --- | --- | --- |
| `LDW` | Read 2 bytes | Load one 16-bit word |
| `STW` | Write 2 bytes | Store all 16 bits of `rs` |
| `LDB` | Read 1 byte | Zero-extend the byte to 16 bits |
| `STB` | Write 1 byte | Store `rs[7:0]` |

Byte accesses accept any address. `LDW` and `STW` require an address divisible by two; a misaligned word access is illegal.

```athe
LDW R1, 4[R2]              ; read a word at R2 + 4 bytes
LDB R3, 1[R2]              ; read one byte at R2 + 1 byte
STW -2[R6], R4             ; write a word at R6 - 2 bytes
STB [R5], R0               ; write the low byte of R0 at R5
```

## Instruction addressing

`PC` contains the 16-bit byte address of the current instruction. Every instruction occupies two bytes, so sequential execution advances from `PC` to `PC + 2`. A valid instruction address must be divisible by two; fetching from or transferring control to any other address is illegal.

### Register targets

`JMPR` and `CALLR` load `PC` from a source register:

```text
PC = rs
```

The register value must be divisible by two.

### Relative targets

`JMP`, `CALL`, and conditional branches encode a signed `off11` measured in instructions. A16 shifts the offset left by one to convert it to a byte displacement:

```text
PC = PC + 2 + (sext(off11) << 1)
```

The encoded range is `-1024` to `+1023` instructions, equivalent to byte displacements from `-2048` to `+2046`. The calculation uses 16-bit wrapping arithmetic.

> [!WARNING]
> An `off11` value of `-1` targets the control-flow instruction itself because `PC + 2 + (-1 << 1) = PC`. `JMP -1` therefore repeats indefinitely. A taken conditional branch with the same value also repeats while its condition remains satisfied.

## Stack

The stack shares the unified memory space and grows toward lower addresses. `SP` identifies the first byte of the most recently stored 16-bit stack word and must remain divisible by two. Register encoding `111` allows software to read and write `SP` directly.

Software chooses the initial value and valid memory region of each stack. After reset, `SP` contains `0x0000`, but this value does not indicate an empty or initialized stack.

`CALL` and `CALLR` decrement `SP` by two before storing the return address:

```text
SP        = SP - 2
MEM16[SP] = PC + 2
```

`RET` reads the current top entry and then increments `SP` by two:

```text
PC = MEM16[SP]
SP = SP + 2
```

`PUSH` and `POP` use the same convention:

```text
SP        = SP - 2          // PUSH
MEM16[SP] = rs

rd = MEM16[SP]              // POP
SP = SP + 2
```

Stack entries are untyped. Software must balance values placed above a return address before executing `RET`. A16 does not track a stack base, limit, depth, or empty state, so software is responsible for alignment and for preventing stack underflow and overflow.

## Program layout

The standard source layout places code at address `0x0000`. Data follows the complete code region at the next two-byte-aligned address. Selecting `.code` and `.data` multiple times changes source interpretation but does not create separate address spaces.

The complete program, including code, initialized data, and reserved data, must fit in the 64 KiB address space.

## Byte order and serialized programs

A16 uses little-endian byte order. For a 16-bit word beginning at address `A`, address `A` contains bits `7:0` and address `A + 1` contains bits `15:8`. `LDB` can observe each byte independently, and `STB` changes only the selected byte.

When the assembler generates a raw binary image, it serializes each 16-bit instruction or data word from its least significant byte to its most significant byte. Text hexadecimal representations contain complete words and therefore have no byte-order ambiguity.

## Timing and physical memory

The ISA does not require combinational or synchronous reads, a particular RAM primitive, or a fixed access latency. A conforming implementation may use FPGA block RAM, external memory, or another storage system as long as completed instructions produce the behavior defined by the ISA.
