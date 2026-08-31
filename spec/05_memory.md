# AthenISA Memory and Addressing

AthenISA uses separate instruction and data address spaces. Both spaces are word-addressed and contain 16-bit words.

| Property | Instruction memory | Data memory |
| --- | --- | --- |
| Address width | 11 bits | 16 bits |
| Address range | `0x000` to `0x7FF` | `0x0000` to `0xFFFF` |
| Number of words | 2048 | 65,536 |
| Word width | 16 bits | 16 bits |
| Byte capacity | 4 KiB | 128 KiB |
| ISA access | Instruction fetch | `LOAD`, `STORE`, `CALL`, `RET`, `PUSH`, `POP` |

Both memory spaces are word-addressed: each address refers to one complete 16-bit word. For example, address `0x0000` selects the first word and address `0x0001` selects the second word; addresses do not identify individual bytes.

The instruction space is read-only from the perspective of AthenISA instructions. Loading a program into it is a platform responsibility performed before normal execution or through an external programming interface.

## Data memory addressing

`LOAD` and `STORE` calculate a data-memory address by adding the signed `off5` field to the base register `rb`:

```text
d_addr = rb + sext(off5)
```

The offset is measured in 16-bit words and has a range of `-16` to `+15`.

The calculation uses 16-bit address arithmetic. If the result exceeds the data-memory address range, it wraps around; for example, `0x0000 - 1` produces address `0xFFFF`.

Assembly represents these operands as follows:

```athe
LOAD  R1, 4[R2]             ; load from address R2 + 4
STORE -1[R7], R3            ; store at address R7 - 1
```

The source-language shorthand for a zero offset is defined in [`asm/syntax.md`](../asm/syntax.md#memory-operands).

## Instruction memory addressing

The program counter (`PC`) contains the 11-bit word address of the current instruction. Sequential execution advances to `PC + 1`, wrapping from `0x7FF` to `0x000`.

### Absolute targets

`JMP` and `CALL` contain an `addr11` field that directly replaces the program counter:

```text
PC = addr11
```

Because `addr11` is 11 bits wide, these instructions can target any word in instruction memory.

### Relative branch targets

Relative branches contain a signed 11-bit offset from the instruction after the branch:

```text
PC = PC + 1 + off11
```

The `off11` field has a signed range of `-1024` to `+1023` instructions. A positive offset branches forward and a negative offset branches backward.

> [!WARNING]
> An `off11` value of `-1` targets the branch instruction itself (`PC + 1 - 1 = PC`). `BRA -1` therefore repeats indefinitely. Conditional branches do not modify the flags, so a conditional branch taken with this offset also repeats while its condition remains satisfied.

The calculation uses 11-bit address arithmetic and therefore wraps at the instruction-memory boundary. For example, advancing beyond `0x7FF` continues from `0x000`.

## Stack

The stack resides in data memory. It starts at the end of the memory and grows toward lower addresses. `SP` identifies the most recently stored stack word, which may contain a return address or a value saved by software. The value `SP = 0x0000` is reserved as the empty-stack marker.

After reset: `SP = 0x0000`.

`CALL` decrements `SP` before storing its return address, so the first stack entry uses the highest data address:

```text
SP       = SP - 1
DMEM[SP] = zext(PC + 1)
```

`RET` reads the current top entry and then increments `SP`:

```text
PC = DMEM[SP]
SP = SP + 1
```

`PUSH` and `POP` use the same stack convention:

```text
SP       = SP - 1           // PUSH
DMEM[SP] = rs

rd = DMEM[SP]               // POP
SP = SP + 1
```

Stack entries are untyped. Software must balance any `PUSH` operations performed after a `CALL` with corresponding `POP` operations before executing `RET`.

`RET` or `POP` with `SP = 0x0000` is a stack-underflow condition. Address `0x0000` must remain outside the valid stack so the empty marker remains unambiguous; therefore the architectural stack can hold at most 65,535 words. AthenISA does not define a stack-overflow exception, so software must avoid exceeding that depth.

## Byte order and serialized programs

AthenISA uses little-endian byte order. Base-ISA memory accesses transfer complete 16-bit words, so byte order is not observable through `LOAD` or `STORE`.

When the assembler generates a raw `.bin` file, it serializes each 16-bit instruction in little-endian order: the low byte is written first, followed by the high byte. Text `.hex` files represent complete 16-bit words and therefore have no byte-order ambiguity. The exact output formats are documented in the [assembler guide](../tools/assembler/README.md#output-files).

## Timing and physical memory

The ISA does not require combinational or synchronous reads, a particular RAM primitive, or a fixed access latency. A conforming implementation may use FPGA block RAM, external memory, or another storage system as long as completed instructions produce the behavior defined by the ISA.
