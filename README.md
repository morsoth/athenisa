# AthenISA

AthenISA is a family of compact, fixed-width Instruction Set Architectures (ISA) designed for small multicycle implementations, FPGA use, and educational CPU development. The A16 base architecture is implemented by the [Tydeus-16](https://github.com/morsoth/tydeus16-core) processor.

This repository is the source of truth for the programmer-visible architecture, assembly language, and reference tools.

## Environment

| Name | Type | Version |
| --- | --- | --- |
| [AthenISA](https://github.com/morsoth/athenisa) | Instruction Set Architecture | `0.1.0` |
| [Tydeus-16](https://github.com/morsoth/tydeus16-core) | Processor core | `0.1.0` |
| Diomedes | Programming language | `-` |
| ArgOS | Operating system | `-` |

## Architectures

| Architecture | Status | Documentation |
| --- | --- | --- |
| A16 | Defined | [A16 documentation](A16/README.md) |
| A32 | In development | [A32 documentation](A32/README.md) |

## Documentation

Read the documents according to what you are trying to do:

| Document | Purpose |
| --- | --- |
| [A16 documentation](A16/README.md) | A16 specification chapters, extensions, and supported instructions |
| [A32 documentation](A32/README.md) | Current status of the A32 specification, extensions, and instruction set |
| [Assembly syntax](asm/syntax.md) | Source structure, literals, symbols, expressions, and data declarations shared by all profiles |
| [A16 assembly reference](asm/A16.md) | A16 registers, operands, ranges, pseudo-instructions, and accepted instruction forms |
| [A32 assembly reference](asm/A32.md) | Status of the A32 assembly profile |
| [Tool installation](tools/README.md) | Installing, running, and releasing the AthenISA command-line tools |
| [Assembler guide](tools/assembler/README.md) | Assembler command line, outputs, diagnostics, and processing flow |
| [Disassembler guide](tools/disassembler/README.md) | Disassembler command line, input formats, output, and limitations |
| [VS Code extension](vscode/README.md) | Syntax highlighting and basic editing support for `.athe` files |

## Repository layout

```text
A16/                    A16 architecture and extensions
A32/                    A32 architecture and extensions
asm/                    Assembly-language syntax
tools/isa/              Shared instruction encoding library
tools/assembler/        Reference assembler
tools/disassembler/     Reference disassembler
vscode/                 Visual Studio Code language support
```

## Command-line tools

From the `tools/` workspace, assemble a source program and disassemble the resulting hexadecimal image with:

```sh
cargo run -p athenisa-asm -- examples/original.athe -o target/program
cargo run -p athenisa-dis -- target/program.hex
```

See the [tools guide](tools/README.md) for installation and the individual tool guides for their complete command-line interfaces.

## VS Code extension

The [AthenISA Language Support extension](https://marketplace.visualstudio.com/items?itemName=morsoth.athenisa-language) can be installed from the Visual Studio Marketplace to add syntax highlighting and basic editing support for `.athe` files. See the [extension guide](vscode/README.md) for more information.

## Specification boundaries

AthenISA defines software-visible state, instruction semantics, encodings, and the memory address model. It does not prescribe a pipeline, cycle count, memory latency, HDL structure, FPGA memory primitive, or external exception interface. Those choices belong to an implementation such as [Tydeus-16](https://github.com/morsoth/tydeus16-core).

## License

AthenISA is released under the [MIT License](LICENSE).
