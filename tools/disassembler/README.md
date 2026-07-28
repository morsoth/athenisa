# AthenISA Disassembler

The reference AthenISA disassembler project is initialized, but instruction decoding and its command-line interface are not implemented yet.

The disassembler is expected to decode 16-bit machine words according to the
[instruction formats](../../spec/02_instruction_formats.md) and
[encoding table](../../spec/04_instruction_encoding.md), then emit canonical
AthenISA assembly syntax.

Until that tool exists, the assembler's `--debug` option can generate a `.lst`
file containing each emitted address, machine word, and real instruction. See
the [assembler guide](../assembler/README.md) for details.
