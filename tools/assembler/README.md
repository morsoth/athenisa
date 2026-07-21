# AthenISA Assembler

This tool assembles AthenISA assembly source files into machine-code outputs for
the Tydeus16 core.

The assembler is intentionally small and direct. It parses the source file,
collects symbols, expands supported pseudo-instructions, encodes real
instructions, and writes the selected output files.

For assembly language syntax, see [asm/syntax.md](../../asm/syntax.md).

## Assembly Flow

The assembler works in a few simple stages:

1. It reads the input `.athe` source file as text.
2. It scans the source and builds the symbol table.
3. It parses each instruction using the collected symbols.
4. It expands supported pseudo-instructions into real ISA instructions.
5. It encodes each real instruction into one 16-bit machine word.
6. It writes the selected output files.

Symbols are collected before instructions are fully parsed. This allows labels
and constants to be used before they are defined later in the file.

Pseudo-instructions do not appear in the final machine-code stream. They are
only a source-level convenience and are expanded before encoding.

The `.lst` output is useful for seeing the final instruction stream that will be
loaded into instruction memory.

## Build

From this directory:

```bash
cargo build
```

For an optimized executable:

```bash
cargo build --release
```

## Usage

```bash
cargo run -- <input.athe> -o <output-base> [--hex] [--sym] [--debug] [--no-bin]
```

Example:

```bash
cargo run -- examples/complete_test.athe -o examples/complete_test --hex --sym --debug
```

The `-o` / `--output` argument is used as an output base path. The assembler
replaces its extension depending on each selected output.

For example, both commands use the same output base:

```bash
cargo run -- examples/complete_test.athe -o examples/complete_test --hex
cargo run -- examples/complete_test.athe -o examples/complete_test.hex --hex
```

Both generate:

```text
examples/complete_test.hex
```

## Output Files

By default, the assembler generates a binary file:

```bash
cargo run -- examples/complete_test.athe -o examples/complete_test
```

This creates:

```text
examples/complete_test.bin
```

The binary output stores each 16-bit instruction word as two bytes in
little-endian order: low byte first, high byte second.

### `--hex`

Generates a `.hex` file:

```bash
cargo run -- examples/complete_test.athe -o examples/complete_test --hex
```

Format:

```text
0000
8000
190A
```

Each line contains one encoded 16-bit instruction word in hexadecimal.

### `--sym`

Generates a `.sym` file:

```bash
cargo run -- examples/complete_test.athe -o examples/complete_test --sym
```

Format:

```text
mem_off     -4
plus_one    1
entry       24
```

The symbol file contains one symbol per line:

```text
<name>  <value>
```

Columns are aligned for readability. Values are written in decimal.

### `--debug`

Generates a `.lst` listing file:

```bash
cargo run -- examples/complete_test.athe -o examples/complete_test --debug
```

Format:

```text
0000  0000  NOP
0001  190A  LI R1, 0x0A
0002  2112  LIH R1, 0x12
```

Columns:

```text
<pc>  <encoded-word>  <real-instruction>
```

The listing shows the real instructions emitted by the assembler.
Pseudo-instructions are shown after expansion.

For example, `LDI` appears as `LI` followed by `LIH`.

### `--no-bin`

Disables the default `.bin` output.

This is useful when generating only text outputs:

```bash
cargo run -- examples/complete_test.athe -o examples/complete_test --hex --sym --debug --no-bin
```

If `--no-bin` is used without any other output flag, the assembler reports an
error because no output file would be generated.

## Symbols

The assembler supports labels and constants using the same symbol table.

Labels take the current instruction address:

```asm
loop:
    BEQ loop
```

Constants provide an explicit value:

```asm
limit: 15
    LI R1, limit
```

Symbols are collected before instruction parsing, so forward references are
valid:

```asm
    JMP end

end:
    RET
```

## Pseudo-Instructions

Pseudo-instructions are expanded before encoding. The current supported
pseudo-instructions are:

```text
LDI
CLR
INC
DEC
```

The `.lst` output is the easiest way to inspect how pseudo-instructions expand.

## Warnings And Errors

The assembler stops at the first hard error, such as:

```text
unknown instruction
invalid register
undefined symbol
invalid number
program too large for instruction memory
```

Immediate and offset values that do not fit in their field currently produce a
warning and are truncated to the field width.

For example, an 8-bit immediate larger than `0xFF` emits a warning and keeps the
low 8 bits.

## Examples

Example source files are available in:

```text
examples/
```

Useful commands:

```bash
cargo run -- examples/complete_test.athe -o examples/complete_test --hex --sym --debug
cargo run -- examples/warning_test.athe -o examples/warning_test --hex --debug --no-bin
cargo run -- examples/error_test.athe -o examples/error_test --hex --no-bin
```
