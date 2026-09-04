# A16 Registers

The processor exposes a compact architectural register set composed of seven general-purpose registers, the program counter, the stack pointer, and a flags register.

## Register file

Every encoded register field is three bits wide.

| Encoding | Register | Width | Behavior |
| --- | --- | --- | --- |
| `000` to `110` | `R0` to `R6` | 16 bits | General-purpose registers |
| `111` | `SP` | 16 bits | Stack pointer |

`R0` through `R6` are general-purpose registers. `SP` shares the same encoded register fields and can also be used as a source, destination, or base register where the instruction permits it.

## Program counter

`PC` is an 11-bit register containing the address of the current instruction.

> [!NOTE]
> `PC` is 11 bits wide because instruction memory contains `2^11 = 2048` words. Its width matches the `imm11` field used by PC-relative control-flow instructions, so their target calculations use 11-bit wrapping arithmetic.

Sequential execution advances `PC` by one. `PC` is not directly addressable by the general register fields.

## Stack pointer

`SP` is a 16-bit register used both explicitly through register encoding `111` and implicitly by `CALL`, `CALLR`, `RET`, `PUSH`, and `POP`. The stack convention grows toward lower data-memory addresses.

Software is responsible for initializing `SP`, assigning the valid stack region, and preventing stack overflow or underflow. The complete stack convention is defined in [05_memory.md](05_memory.md#stack).

## Flags register

`FLAGS` contains four one-bit status flags.

| Bit | Flag | Meaning |
| --- | --- | --- |
| `3` | `V` | Signed overflow |
| `2` | `N` | Negative result; equal to result bit 15 |
| `1` | `C` | Unsigned carry, or no-borrow indication for subtraction |
| `0` | `Z` | Zero result |

For addition, `C` is the carry out of bit 15. For subtraction, `C = 1` means no unsigned borrow occurred and `C = 0` means a borrow occurred. `V` reports signed two's-complement overflow.

Flag updates are grouped as follows:

| Instructions | `Z` | `C` | `N` | `V` |
| --- | --- | --- | --- | --- |
| `ADD`, `ADDI` | From result | Carry out | From result | Addition overflow |
| `SUB`, `SUBI`, `CMP`, `CMPI` | From result | No borrow | From result | Subtraction overflow |
| `AND`, `OR`, `XOR`, `NOT` | From result | `0` | From result | `0` |
| `SLL`, `SRL`, `SRA` | From result | `0` | From result | `0` |
| All other instructions | Unchanged | Unchanged | Unchanged | Unchanged |

`CMP` and `CMPI` calculate flags exactly like subtraction but do not write the subtraction result to the register file. Conditional branches read `Z`, `C`, `N`, and `V` as required to determine whether their branch condition is satisfied.

## Reset state

The reset profile is:

| State | Reset value |
| --- | --- |
| `R0` to `R6` | `0x0000` |
| `PC` | `0x000` |
| `SP` | `0x0000` |
| `FLAGS` | `0000` |

The reset value of `SP` does not establish a valid stack. Software must initialize it before executing stack instructions. Instruction and data memory contents are not cleared by architectural reset. Program loading and memory initialization are platform responsibilities.
