# AthenISA Registers

The processor exposes a compact architectural register set composed of one zero register,
seven general-purpose registers, the program counter, the stack pointer, and a flags register.

## Register file

Every encoded register field is three bits wide.

| Encoding | Register | Width | Behavior |
| --- | --- | --- | --- |
| `000` | `R0` | 16 bits | Constant zero; writes are ignored |
| `001` | `R1` | 16 bits | General-purpose register |
| `010` | `R2` | 16 bits | General-purpose register |
| `011` | `R3` | 16 bits | General-purpose register |
| `100` | `R4` | 16 bits | General-purpose register |
| `101` | `R5` | 16 bits | General-purpose register |
| `110` | `R6` | 16 bits | General-purpose register |
| `111` | `R7` | 16 bits | General-purpose register |

`R0` always reads as `0x0000`. An instruction may name `R0` as a destination,
but the write has no effect. A flag-setting instruction still updates `FLAGS`
from its computed result even when its destination is `R0`.

## Program counter

`PC` is an 11-bit register containing the word address of the current
instruction.

> [!NOTE]
> `PC` is 11 bits wide because instruction memory contains `2^11 = 2048`
> words. This allows an [absolute jump](02_instruction_formats.md#absolute-jump)
> to reach every instruction-memory address using the 11-bit field encoded in
> the instruction.

Sequential execution advances `PC` by one. `PC` is not directly addressable by
the general register fields.

## Stack pointer

`SP` is a 16-bit register used implicitly by `CALL` and `RET`. The stack grows
toward lower data-memory addresses. `SP = 0x0000` is the empty-stack marker, so
the first `CALL` stores its return address at `0xFFFF`.

`SP` is not directly addressable by the general register fields. The complete
stack convention is defined in [05_memory.md](05_memory.md#stack).

## Flags register

`FLAGS` contains four one-bit status flags.

| Bit | Flag | Meaning |
| --- | --- | --- |
| `3` | `V` | Signed overflow |
| `2` | `N` | Negative result; equal to result bit 15 |
| `1` | `C` | Unsigned carry, or no-borrow indication for subtraction |
| `0` | `Z` | Zero result |

For addition, `C` is the carry out of bit 15. For subtraction, `C = 1` means no
unsigned borrow occurred and `C = 0` means a borrow occurred. `V` reports signed
two's-complement overflow.

Flag updates are grouped as follows:

| Instructions | `Z` | `C` | `N` | `V` |
| --- | --- | --- | --- | --- |
| `ADD`, `ADDI` | From result | Carry out | From result | Addition overflow |
| `SUB`, `SUBI`, `CMP`, `CMPI` | From result | No borrow | From result | Subtraction overflow |
| `AND`, `OR`, `XOR`, `NOT` | From result | Cleared | From result | Cleared |
| `SLL`, `SRL`, `SRA` | From result | Cleared | From result | Cleared |
| All other instructions | Unchanged | Unchanged | Unchanged | Unchanged |

`CMP` and `CMPI` calculate flags exactly like subtraction but do not write the
subtraction result to the register file. Conditional branches read `Z`, `N`, and
`V`.

## Reset state

The reset profile is:

| State | Reset value |
| --- | --- |
| `R0` to `R7` | `0x0000` |
| `PC` | `0x000` |
| `SP` | `0x0000` (empty stack) |
| `FLAGS` | `0000` |

Instruction and data memory contents are not cleared by architectural reset.
Program loading and memory initialization are platform responsibilities.
