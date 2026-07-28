# AthenISA Disassembler

The reference disassembler converts AthenISA raw binaries and hexadecimal memory images into canonical assembly source. It uses the same instruction definitions and decoder as the assembler.

## Installation

Installation with Cargo and prebuilt release packages is documented in the shared [AthenISA tools guide](../README.md).

## Quick start

Disassemble a raw binary to standard output:

```sh
athenisa-dis program.bin
```

Write assembly generated from a hexadecimal image to a file:

```sh
athenisa-dis program.hex -o program.athe
```

## Command line

```text
athenisa-dis <input> [-o <output>]
```

When running through Cargo from `tools/`, select the disassembler package and place its arguments after `--`:

```sh
cargo run -p athenisa-dis -- <input> [-o <output>]
```

| Argument | Effect |
| --- | --- |
| `<input>` | Raw `.bin` or textual `.hex` machine-code image |
| `-o`, `--output <path>` | Write assembly to the specified file instead of standard output |

The input extension selects the file format and is case-insensitive. The output path is used exactly as written; the disassembler does not add or replace its extension.

### Help and version

| Option | Effect |
| --- | --- |
| `-h`, `--help` | Show command-line help |
| `-V`, `--version` | Show the installed disassembler version |

```sh
athenisa-dis --help
athenisa-dis --version
```

## Input formats

### Raw binary: `.bin`

The file contains headerless 16-bit instruction words stored as little-endian byte pairs. An odd byte count is rejected because the final instruction would be incomplete.

### Hexadecimal image: `.hex`

Each non-empty line contains one hexadecimal instruction word of at most four digits. Addresses, headers, comments, and multiple words on one line are not accepted.

Both formats are compatible with the corresponding outputs of `athenisa-asm`.

## Generated assembly

The output contains only real AthenISA instructions. Symbol names, constants, comments, source formatting, labels, and pseudo-instructions cannot be recovered from machine code.

`JMP` and `CALL` operands are written as absolute addresses. Conditional branch operands are written as signed relative offsets. Reassembling valid canonical output produces the original instruction words.

## Disassembly flow

1. Select the input format from the file extension and read its instruction words.
2. Reject images larger than the 2048-word AthenISA instruction address space.
3. Decode every word and validate its opcode and reserved fields.
4. Format each real instruction using canonical AthenISA assembly syntax.
5. Write the result to the selected file or standard output.

## Diagnostics

The disassembler stops at the first invalid word and reports its instruction address. Reserved opcodes, nonzero reserved fields, malformed hexadecimal lines, incomplete binary words, unsupported input extensions, and oversized images are errors.
