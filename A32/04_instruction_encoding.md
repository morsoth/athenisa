# A32 Instruction Encoding

A32 uses a six-bit primary opcode and a three-bit function field where an opcode groups related operations. Opcode assignments intentionally follow the A16 ordering where practical, with a leading zero added to the corresponding five-bit A16 opcode.

| Instruction | `opcode` | `func` | Instruction format |
| --- | --- | --- | --- |
| `NOP` | `000000` | - | No operand |
| `MOV` | `000001` | `000` | RR |
| `ADD` | `000001` | `001` | RRR |
| `SUB` | `000001` | `010` | RRR |
| `CMP` | `000001` | `011` | RR |
| `AND` | `000010` | `000` | RRR |
| `OR` | `000010` | `001` | RRR |
| `XOR` | `000010` | `010` | RRR |
| `NOT` | `000010` | `011` | RR |
| `LI` | `000011` | - | RI |
| `LIH` | `000100` | - | RI |
| `SLL` | `000101` | - | RRI |
| `SRL` | `000110` | - | RRI |
| `SRA` | `000111` | - | RRI |
| `JMP` | `001000` | - | Absolute jump |
| `BRA` | `001001` | - | Relative branch |
| `BEQ` | `001010` | - | Relative branch |
| `BNE` | `001011` | - | Relative branch |
| `BLT` | `001100` | - | Relative branch |
| `BGE` | `001101` | - | Relative branch |
| `BLTU` | `001110` | - | Relative branch |
| `BGEU` | `001111` | - | Relative branch |
| `CALL` | `010000` | - | Absolute jump |
| `RET` | `010001` | - | No operand |
| `JMPR` | `010010` | `000` | R |
| `CALLR` | `010010` | `001` | R |
| `PUSH` | `010010` | `010` | R |
| `POP` | `010010` | `011` | R |
| `LOAD` | `010011` | - | Load |
| `STORE` | `010100` | - | Store |
| `ADDI` | `010101` | - | RRI |
| `SUBI` | `010110` | - | RRI |
| `CMPI` | `010111` | - | RI |
| Reserved | `011xxx` | - | - |
| Reserved | `1xxxxx` | - | - |

Unused `func` values within an assigned opcode group are reserved. A32 extensions allocate their instructions from the reserved opcode and function space without changing base instruction encodings.
