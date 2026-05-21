# AthenISA Instruction Encoding Table

| Instruction | Opcode | Func | Format |
| --- | --- | --- | --- |
| `NOP` | `00000` | - | No operand |
| `MOV` | `00001` | `00` | Register-register (RR) |
| `ADD` | `00001` | `01` | Register-register-register (RRR) |
| `SUB` | `00001` | `10` | Register-register-register (RRR) |
| `CMP` | `00001` | `11` | Register-register (RR) |
| `AND` | `00010` | `00` | Register-register-register (RRR) |
| `OR` | `00010` | `01` | Register-register-register (RRR) |
| `XOR` | `00010` | `10` | Register-register-register (RRR) |
| `NOT` | `00010` | `11` | Register-register (RR) |
| `LI` | `00011` | - | Register-immediate (RI) |
| `LIH` | `00100` | - | Register-immediate (RI) |
| `SLL` | `00101` | - | Register-register-immediate (RRI) |
| `SRL` | `00110` | - | Register-register-immediate (RRI) |
| `SRA` | `00111` | - | Register-register-immediate (RRI) |
| `JMP` | `01000` | - | Unconditional jump |
| `BEQ` | `01001` | - | Conditional branch |
| `BNE` | `01010` | - | Conditional branch |
| `BLT` | `01011` | - | Conditional branch |
| `BGT` | `01100` | - | Conditional branch |
| `BLE` | `01101` | - | Conditional branch |
| `BGE` | `01110` | - | Conditional branch |
| `CALL` | `01111` | - | Unconditional jump |
| `RET` | `10000` | - | No operand |
| `Reserved` | `10001` | - | - |
| `LOAD` | `10010` | - | Load |
| `STORE` | `10011` | - | Store |
| `ADDI` | `10100` | - | Register-immediate (RI) |
| `SUBI` | `10101` | - | Register-immediate (RI) |
| `CMPI` | `10110` | - | Register-immediate (RI) |
| `Reserved` | `10111` | - | - |
| `Reserved` | `11xxx` | - | - |
