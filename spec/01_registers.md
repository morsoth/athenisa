# AthenISA Registers

The processor exposes a compact architectural register set composed of one zero register,
seven general-purpose registers, the program counter, the stack pointer, and a flags register.

### Zero register

`R0` is a 16-bit constant zero register. Its value is permanently fixed to `0x0000`, and any write
to `R0` is ignored.

### General-purpose registers

The processor provides seven 16-bit general-purpose registers, `R1` to `R7`. These registers are
used as operands and destinations for the instructions.

### Program counter register

The program counter, `PC`, is an 11-bit register that holds the address of the current instruction
in instruction memory.

> [!NOTE]
> `PC` register is 11-bit wide due to the instruction memory size being `2^11 = 2048` words. This allows absolute jumps to reach all the instruction memory adressess with the 11 bits encoded in the instruction.

### Stack pointer register

The stack pointer, SP, is a 16-bit register that points to the element currently located at the
top of the stack.

### Flags register

The `FLAGS` register is a 4-bit register that stores the processor’s ALU status flags. It contains
the following four 1-bit elements:

| Flag | Meaning |
| --- | --- |
| Z | Zero flag |
| C | Carry flag |
| N | Negative flag |
| V | Overflow flag |