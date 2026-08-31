# AthenISA

AthenISA is the 16-bit Instruction Set Architecture (ISA) implemented by the [Tydeus-16](https://github.com/morsoth/tydeus16-core) processor core. It is a compact, fixed-width ISA designed for a small multicycle implementation, FPGA use, and educational CPU development.

This repository is the source of truth for the programmer-visible architecture, assembly language, and reference tools.

## Environment

| Name | Type | Version |
| --- | --- | --- |
| [AthenISA](https://github.com/morsoth/athenisa) | Instruction Set Architecture | `0.1.0` |
| [Tydeus-16](https://github.com/morsoth/tydeus16-core) | Processor core | `0.1.0` |
| Diomedes | Programming language | `-` |
| ArgOS | Operating system | `-` |

## Summary

| Property | AthenISA |
| --- | --- |
| Instruction width | 16 bits |
| Data width | 16 bits |
| General-purpose registers | 7 (`R0` to `R6`) |
| Stack pointer | `SP` |
| Instruction address width | 11 bits |
| Data address width | 16 bits |
| Memory model | Separate instruction and data spaces, word-addressed |
| Status flags | Zero, carry, negative, overflow (`Z`, `C`, `N`, `V`) |

## Documentation

Read the documents according to what you are trying to do:

| Document | Purpose |
| --- | --- |
| [A16 overview](A16/spec/00_overview.md) | Architectural scope, execution model, notation, and specification conventions |
| [A16 registers](A16/spec/01_registers.md) | Register encodings, special registers, reset state, and flags |
| [A16 instruction set](A16/spec/02_instruction_set.md) | Programmer-visible semantics of every real instruction |
| [A16 instruction formats](A16/spec/03_instruction_formats.md) | Bit layout of every 16-bit instruction format |
| [A16 instruction encoding](A16/spec/04_instruction_encoding.md) | Opcode and function-field assignments |
| [A16 memory model](A16/spec/05_memory.md) | Address spaces, effective addresses, control-flow targets, and stack convention |
| [Assembly syntax](asm/syntax.md) | Source syntax, literals, symbols, operands, and pseudo-instructions |
| [Tool installation](tools/README.md) | Installing, running, and releasing the AthenISA command-line tools |
| [Assembler guide](tools/assembler/README.md) | Assembler command line, outputs, diagnostics, and processing flow |
| [Disassembler guide](tools/disassembler/README.md) | Disassembler command line, input formats, output, and limitations |
| [VS Code extension](vscode/README.md) | Syntax highlighting and basic editing support for `.athe` files |

## Repository layout

```text
A16/spec/               A16 architectural specification
A16/extensions/         A16 architectural extensions
A32/spec/               A32 architectural specification
A32/extensions/         A32 architectural extensions
asm/                    Assembly-language syntax
tools/isa/              Shared instruction encoding library
tools/assembler/        Reference assembler
tools/disassembler/     Reference disassembler
vscode/                 Visual Studio Code language support
```

## Command-line tools

From the `tools/` workspace, assemble a source program and disassemble the resulting hexadecimal image with:

```sh
cargo run -p athenisa-asm -- examples/program.athe -o target/program
cargo run -p athenisa-dis -- target/program.hex
```

See the [tools guide](tools/README.md) for installation and the individual tool guides for their complete command-line interfaces.

## VS Code extension

The [AthenISA Language Support extension](https://marketplace.visualstudio.com/items?itemName=morsoth.athenisa-language) can be installed from the Visual Studio Marketplace to add syntax highlighting and basic editing support for `.athe` files. See the [extension guide](vscode/README.md) for more information.

## Specification boundaries

AthenISA defines software-visible state, instruction semantics, encodings, and the memory address model. It does not prescribe a pipeline, cycle count, memory latency, HDL structure, FPGA memory primitive, or external exception interface. Those choices belong to an implementation such as [Tydeus-16](https://github.com/morsoth/tydeus16-core).

## License

AthenISA is released under the [MIT License](LICENSE).
