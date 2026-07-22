# AthenISA Disassembler

The reference AthenISA disassembler is planned but not implemented yet. This
directory is reserved for the future tool; there is currently no executable or
command-line interface to build.

The disassembler is expected to decode 16-bit machine words according to the
[instruction formats](../../spec/02_instruction_formats.md) and
[encoding table](../../spec/04_instruction_encoding.md), then emit canonical
AthenISA assembly syntax.

Until that tool exists, the assembler's `--debug` option can generate a `.lst`
file containing each emitted address, machine word, and real instruction. See
the [assembler guide](../assembler/README.md) for details.
