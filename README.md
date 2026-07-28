# AthenISA

###### Version: 0.1

AthenISA is the 16-bit instruction set architecture implemented by the
[Tydeus-16](https://github.com/morsoth/tydeus16-core) processor core. It is a compact, fixed-width ISA
designed for a small multicycle implementation, FPGA use, and educational CPU
development.

This repository is the source of truth for the programmer-visible architecture,
the assembly language, and the accompanying tools.

## Architecture at a glance

| Property | AthenISA |
| --- | --- |
| Instruction width | 16 bits |
| Data width | 16 bits |
| General register encodings | 7 (`R1` to `R7`) |
| Zero register | `R0` |
| Instruction address width | 11 bits |
| Data address width | 16 bits |
| Memory model | Separate instruction and data spaces, word-addressed |
| Status flags | Zero, carry, negative, overflow (`Z`, `C`, `N`, `V`) |
| Data-memory addressing | Base register plus signed 5-bit offset |

## Documentation map

Read the documents according to what you are trying to do:

| Document | Purpose |
| --- | --- |
| [ISA overview](spec/00_overview.md) | Architectural scope, execution model, notation, and specification conventions |
| [Registers](spec/01_registers.md) | Register encodings, special registers, reset state, and flags |
| [Instruction formats](spec/02_instruction_formats.md) | Bit layout of every 16-bit instruction format |
| [Instruction set](spec/03_instruction_set.md) | Programmer-visible semantics of every real instruction |
| [Instruction encoding](spec/04_instruction_encoding.md) | Opcode and function-field assignments |
| [Memory model](spec/05_memory.md) | Address spaces, effective addresses, control-flow targets, and stack convention |
| [Assembly syntax](asm/syntax.md) | Source syntax, literals, symbols, operands, and pseudo-instructions |
| [Assembler guide](tools/assembler/README.md) | Building and using the reference assembler and understanding its outputs |
| [Design decisions](docs/design_decisions.md) | Non-normative rationale behind the architecture |
| [Naming conventions](docs/naming.md) | Terminology and style used across the specification and tools |

The specification under `spec/` is normative. The documents under `docs/` are
explanatory and do not change architectural behavior.

## Repository layout

```text
asm/                    Assembly-language syntax
docs/                   Design rationale and contributor conventions
spec/                   Versioned architectural specification
tools/assembler/        Reference assembler
tools/disassembler/     Planned reference disassembler
```

## Assemble a program

AthenISA source files use the `.athe` extension. From `tools/assembler`:

```bash
cargo run -- examples/complete_test.athe -o examples/complete_test --hex --sym --debug
```

This generates the default raw binary plus the selected text outputs. See the
[assembler guide](tools/assembler/README.md) for the complete command-line and
file-format reference.

## Specification boundaries

AthenISA defines software-visible state, instruction semantics, encodings, and
the memory address model. It does not prescribe a pipeline, cycle count, memory
latency, HDL structure, FPGA memory primitive, or external exception interface.
Those choices belong to an implementation such as Tydeus-16.
