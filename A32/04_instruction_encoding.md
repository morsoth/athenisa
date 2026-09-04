# A32 Instruction Encoding

A32 uses a six-bit primary opcode. The `R` format also provides a two-bit `func` field in bits `1:0` so one opcode can group related register operations. Every base A32 opcode is formed by prefixing the corresponding five-bit A16 opcode with `0`; assigned `func` values are identical in both bases.

| Instruction | `opcode` | `func` | Instruction format |
| --- | --- | --- | --- |
| `NOP` | `000000` | - | N |
| `MOV` | `000001` | `00` | R |
| `ADD` | `000001` | `01` | R |
| `SUB` | `000001` | `10` | R |
| `CMP` | `000001` | `11` | R |
| `AND` | `000010` | `00` | R |
| `OR` | `000010` | `01` | R |
| `XOR` | `000010` | `10` | R |
| `NOT` | `000010` | `11` | R |
| `SLL` | `000011` | `00` | R |
| `SRL` | `000011` | `01` | R |
| `SRA` | `000011` | `10` | R |
| `SLLI` | `000100` | - | RRI |
| `SRLI` | `000101` | - | RRI |
| `SRAI` | `000110` | - | RRI |
| `LI` | `000111` | - | RI |
| `LIH` | `001000` | - | RI |
| `JMP` | `001001` | - | I |
| `BEQ` | `001010` | - | I |
| `BNE` | `001011` | - | I |
| `BLT` | `001100` | - | I |
| `BGE` | `001101` | - | I |
| `BLTU` | `001110` | - | I |
| `BGEU` | `001111` | - | I |
| `CALL` | `010000` | - | I |
| `RET` | `010001` | - | N |
| `JMPR` | `010010` | `00` | R |
| `CALLR` | `010010` | `01` | R |
| `PUSH` | `010010` | `10` | R |
| `POP` | `010010` | `11` | R |
| `LDW` | `010011` | - | RRI |
| `STW` | `010100` | - | RRI |
| `LDB` | `011000` | - | RRI |
| `STB` | `011001` | - | RRI |
| `ADDI` | `010101` | - | RRI |
| `SUBI` | `010110` | - | RRI |
| `CMPI` | `010111` | - | RI |
| `ANDI` | `011010` | - | RRI |
| `ORI` | `011011` | - | RRI |
| `XORI` | `011100` | - | RRI |
| Reserved | `011101` | - | - |
| Reserved | `01111x` | - | - |
| Reserved | `1xxxxx` | - | - |
