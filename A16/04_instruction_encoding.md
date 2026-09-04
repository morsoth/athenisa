# A16 Instruction Encoding

A16 uses a five-bit primary opcode. The `R` format also provides a two-bit `func` field in bits `1:0` so one opcode can group related register operations.

| Instruction | `opcode` | `func` | Instruction format |
| --- | --- | --- | --- |
| `NOP` | `00000` | - | N |
| `MOV` | `00001` | `00` | R |
| `ADD` | `00001` | `01` | R |
| `SUB` | `00001` | `10` | R |
| `CMP` | `00001` | `11` | R |
| `AND` | `00010` | `00` | R |
| `OR` | `00010` | `01` | R |
| `XOR` | `00010` | `10` | R |
| `NOT` | `00010` | `11` | R |
| `SLL` | `00011` | `00` | R |
| `SRL` | `00011` | `01` | R |
| `SRA` | `00011` | `10` | R |
| `SLLI` | `00100` | - | RRI |
| `SRLI` | `00101` | - | RRI |
| `SRAI` | `00110` | - | RRI |
| `LI` | `00111` | - | RI |
| `LIH` | `01000` | - | RI |
| `JMP` | `01001` | - | I |
| `BEQ` | `01010` | - | I |
| `BNE` | `01011` | - | I |
| `BLT` | `01100` | - | I |
| `BGE` | `01101` | - | I |
| `BLTU` | `01110` | - | I |
| `BGEU` | `01111` | - | I |
| `CALL` | `10000` | - | I |
| `RET` | `10001` | - | N |
| `JMPR` | `10010` | `00` | R |
| `CALLR` | `10010` | `01` | R |
| `PUSH` | `10010` | `10` | R |
| `POP` | `10010` | `11` | R |
| Reserved | `10011` | - | - |
| `LDW` | `10100` | - | RRI |
| `STW` | `10101` | - | RRI |
| `LDB` | `10110` | - | RRI |
| `STB` | `10111` | - | RRI |
| `ADDI` | `11000` | - | RI |
| `SUBI` | `11001` | - | RI |
| `CMPI` | `11010` | - | RI |
| `ANDI` | `11011` | - | RI |
| `ORI` | `11100` | - | RI |
| `XORI` | `11101` | - | RI |
| Reserved | `1111x` | - | - |
