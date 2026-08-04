# AthenISA Instruction Encoding

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
| `BEQ` | `01001` | - | Conditional branch |
| `BNE` | `01010` | - | Conditional branch |
| `BLT` | `01011` | - | Conditional branch |
| `BGT` | `01100` | - | Conditional branch |
| `BLE` | `01101` | - | Conditional branch |
| `BGE` | `01110` | - | Conditional branch |
| `CALL` | `01111` | - | Absolute jump |
| `RET` | `10000` | - | No operand |
| Reserved | `10001` | - | - |
| `LOAD` | `10010` | - | Load |
| `STORE` | `10011` | - | Store |
| `ADDI` | `10100` | - | RI |
| `SUBI` | `10101` | - | RI |
| `CMPI` | `10110` | - | RI |
| Reserved | `10111` | - | - |
| Reserved | `11xxx` | - | - |