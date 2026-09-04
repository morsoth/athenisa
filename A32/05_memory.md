# A32 Memory and Addressing

A32 uses one unified, byte-addressable memory space for instructions, data, and the stack.

| Property | Definition |
| --- | --- |
| Address width | 32 bits |
| Address range | `0x00000000` to `0xFFFFFFFF` |
| Capacity | 4 GiB |
| Instruction size | 4 bytes |
| Data word size | 4 bytes |
| Byte order | Little-endian |

Every address identifies one byte. Instructions and data share the same addresses, so the base architecture does not prevent a store from modifying memory that contains instructions. A platform may add access permissions, but they are outside the A32 base specification. An implementation may provide less physical memory than the architectural address space, but the platform must define which address ranges are available.

## Memory accesses

`LDW`, `STW`, `LDB`, and `STB` calculate an effective address by adding the signed `imm16` field to the base register `rb`:

```text
address = rb + sext(imm16)
```

The offset is measured in bytes and has a range from `-32,768` to `+32,767`. Address arithmetic wraps to 32 bits.

| Instruction | Access | Result |
| --- | --- | --- |
| `LDW` | Read 4 bytes | Load one 32-bit word |
| `STW` | Write 4 bytes | Store all 32 bits of `rs` |
| `LDB` | Read 1 byte | Zero-extend the byte to 32 bits |
| `STB` | Write 1 byte | Store `rs[7:0]` |

Byte accesses accept any address. `LDW` and `STW` require an address divisible by four; a misaligned word access is illegal.

```athe
LDW R1, 8[R2]              ; read a word at R2 + 8 bytes
LDB R3, 1[R2]              ; read one byte at R2 + 1 byte
STW -4[R30], R4            ; write a word at R30 - 4 bytes
STB [R5], R0               ; write the low byte of R0 at R5
```

## Instruction addressing

`PC` contains the 32-bit byte address of the current instruction. Every instruction occupies four bytes, so sequential execution advances from `PC` to `PC + 4`. A valid instruction address must be divisible by four; fetching from or transferring control to any other address is illegal.

### Register targets

`JMPR` and `CALLR` load `PC` from a source register:

```text
PC = rs
```

The register value must be divisible by four.

### Relative targets

`JMP`, `CALL`, and conditional branches interpret `imm26` as a signed offset measured in instructions. A32 shifts the immediate left by two to convert it to a byte displacement:

```text
PC = PC + 4 + (sext(imm26) << 2)
```

The encoded range is `-33,554,432` to `+33,554,431` instructions, equivalent to byte displacements from `-134,217,728` to `+134,217,724`. The calculation uses 32-bit wrapping arithmetic.

> [!WARNING]
> An `imm26` value of `-1` targets the control-flow instruction itself because `PC + 4 + (-1 << 2) = PC`. `JMP -1` therefore repeats indefinitely. A taken conditional branch with the same value also repeats while its condition remains satisfied.

## Stack

The stack shares the unified memory space and grows toward lower addresses. `SP` identifies the first byte of the most recently stored 32-bit stack word and must remain divisible by four. Register encoding `11111` allows software to read and write `SP` directly.

Software chooses the initial value and valid memory region of each stack. After reset, `SP` contains `0x00000000`, but this value does not indicate an empty or initialized stack.

`CALL` and `CALLR` decrement `SP` by four before storing the return address:

```text
SP        = SP - 4
MEM32[SP] = PC + 4
```

`RET` reads the current top entry and then increments `SP` by four:

```text
PC = MEM32[SP]
SP = SP + 4
```

`PUSH` and `POP` use the same convention:

```text
SP        = SP - 4          // PUSH
MEM32[SP] = rs

rd = MEM32[SP]              // POP
SP = SP + 4
```

Stack entries are untyped. Software must balance values placed above a return address before executing `RET`. A32 does not track a stack base, limit, depth, or empty state, so software is responsible for alignment and for preventing stack underflow and overflow.

## Program layout

The standard source layout places code at address `0x00000000`. Data follows the complete code region at the next four-byte-aligned address. Selecting `.code` and `.data` multiple times changes source interpretation but does not create separate address spaces.

The complete program must fit in the available platform memory and every referenced address must belong to an implemented region.

## Byte order and serialized programs

A32 uses little-endian byte order. For a 32-bit word beginning at address `A`, addresses `A`, `A + 1`, `A + 2`, and `A + 3` contain bits `7:0`, `15:8`, `23:16`, and `31:24`, respectively. `LDB` can observe each byte independently, and `STB` changes only the selected byte.

When the assembler generates a raw binary image, it serializes each 32-bit instruction or data word from its least significant byte to its most significant byte. Text hexadecimal representations contain complete words and therefore have no byte-order ambiguity.

## Timing and physical memory

The ISA does not require combinational or synchronous reads, a particular RAM primitive, or a fixed access latency. A conforming implementation may use FPGA block RAM, external memory, or another storage system as long as completed instructions produce the behavior defined by the ISA.
