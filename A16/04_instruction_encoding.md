# A16 Instruction Encoding

| Instruction | `opcode` | `func` | Instruction format |
| --- | --- | --- | --- |
| `NOP` | `00000` | - | No operand |
| `MOV` | `00001` | `00` | RR |
| `ADD` | `00001` | `01` | RRR |
| `SUB` | `00001` | `10` | RRR |
| `CMP` | `00001` | `11` | RR |
| `AND` | `00010` | `00` | RRR |
| `OR` | `00010` | `01` | RRR |
| `XOR` | `00010` | `10` | RRR |
| `NOT` | `00010` | `11` | RR |
| `LI` | `00011` | - | RI |
| `LIH` | `00100` | - | RI |
| `SLL` | `00101` | - | RRI |
| `SRL` | `00110` | - | RRI |
| `SRA` | `00111` | - | RRI |
| `JMP` | `01000` | - | Jump |
| Reserved | `01001` | - | - |
| `BEQ` | `01010` | - | Jump |
| `BNE` | `01011` | - | Jump |
| `BLT` | `01100` | - | Jump |
| `BGE` | `01101` | - | Jump |
| `BLTU` | `01110` | - | Jump |
| `BGEU` | `01111` | - | Jump |
| `CALL` | `10000` | - | Jump |
| `RET` | `10001` | - | No operand |
| `JMPR` | `10010` | `00` | R |
| `CALLR` | `10010` | `01` | R |
| `PUSH` | `10010` | `10` | R |
| `POP` | `10010` | `11` | R |
| `LDW` | `10011` | - | Load |
| `STW` | `10100` | - | Store |
| `ADDI` | `10101` | - | RI |
| `SUBI` | `10110` | - | RI |
| `CMPI` | `10111` | - | RI |
| `LDB` | `11000` | - | Load |
| `STB` | `11001` | - | Store |
| Reserved | `1101x` | - | - |
| Reserved | `111xx` | - | - |
