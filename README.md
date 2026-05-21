# AthenISA

###### Version: 0.1

AthenISA is the 16-bit Instruction Set Architecture implemented by the Tydeus-16 core.

## Ecosystem

- Core: Tydeus-16
- ISA: AthenISA
- Assembly source: `.athe`
- High-level language: Diomedes (`.dio`)

## Main features

- 16-bit instruction width
- 16-bit data width
- Fixed-width instructions
- 8 architectural registers: R0-R7
- R0 hardwired to zero
- PC, SP and FLAGS registers
- Separate instruction and data memories
- Base + signed offset memory addressing

## Repository layout

- `spec/`: formal ISA specification
- `encodings/`: machine-readable encoding definitions
- `asm/`: assembly syntax and examples
- `tools/`: reference assembler and disassembler
- `docs/`: rationale and project notes