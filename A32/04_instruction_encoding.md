# A32 Instruction Encoding

A32 uses a six-bit primary opcode. The `R` format also provides a three-bit `func` field so one opcode can group related register operations. Opcode assignments intentionally follow the A16 ordering where practical, with a leading zero added to the corresponding five-bit A16 opcode.

| Instruction | `opcode` | `func` | Instruction format |
| --- | --- | --- | --- |
| `NOP` | `000000` | - | I |
| `MOV` | `000001` | `000` | R |
| `ADD` | `000001` | `001` | R |
| `SUB` | `000001` | `010` | R |
| `CMP` | `000001` | `011` | R |
| `AND` | `000010` | `000` | R |
| `OR` | `000010` | `001` | R |
| `XOR` | `000010` | `010` | R |
| `NOT` | `000010` | `011` | R |
| `LI` | `000011` | - | RI |
| `LIH` | `000100` | - | RI |
| `SLL` | `000101` | - | RRI |
| `SRL` | `000110` | - | RRI |
| `SRA` | `000111` | - | RRI |
| `JMP` | `001000` | - | I |
| Reserved | `001001` | - | - |
| `BEQ` | `001010` | - | I |
| `BNE` | `001011` | - | I |
| `BLT` | `001100` | - | I |
| `BGE` | `001101` | - | I |
| `BLTU` | `001110` | - | I |
| `BGEU` | `001111` | - | I |
| `CALL` | `010000` | - | I |
| `RET` | `010001` | - | I |
| `JMPR` | `010010` | `000` | R |
| `CALLR` | `010010` | `001` | R |
| `PUSH` | `010010` | `010` | R |
| `POP` | `010010` | `011` | R |
| `LDW` | `010011` | - | RRI |
| `STW` | `010100` | - | RRI |
| `ADDI` | `010101` | - | RRI |
| `SUBI` | `010110` | - | RRI |
| `CMPI` | `010111` | - | RI |
| `LDB` | `011000` | - | RRI |
| `STB` | `011001` | - | RRI |
| `ANDI` | `011010` | - | RRI |
| `ORI` | `011011` | - | RRI |
| `XORI` | `011100` | - | RRI |
| Reserved | `011101` | - | - |
| Reserved | `01111x` | - | - |
| Reserved | `1xxxxx` | - | - |

Unused `func` values within an assigned opcode group are reserved. A32 extensions allocate their instructions from the reserved opcode and function space without changing base instruction encodings.
