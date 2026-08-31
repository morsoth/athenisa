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
| `JMP` | `01000` | - | Absolute jump |
| `BRA` | `01001` | - | Relative branch |
| `BEQ` | `01010` | - | Relative branch |
| `BNE` | `01011` | - | Relative branch |
| `BLT` | `01100` | - | Relative branch |
| `BGE` | `01101` | - | Relative branch |
| `BLTU` | `01110` | - | Relative branch |
| `BGEU` | `01111` | - | Relative branch |
| `CALL` | `10000` | - | Absolute jump |
| `RET` | `10001` | - | No operand |
| `JMPR` | `10010` | `00` | R |
| `CALLR` | `10010` | `01` | R |
| `PUSH` | `10010` | `10` | R |
| `POP` | `10010` | `11` | R |
| `LOAD` | `10011` | - | Load |
| `STORE` | `10100` | - | Store |
| `ADDI` | `10101` | - | RI |
| `SUBI` | `10110` | - | RI |
| `CMPI` | `10111` | - | RI |
| Reserved | `11xxx` | - | - |
