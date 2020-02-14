# mips-emulator
MIPS Emulator

# Read Direction
Always read from right to left.

```
R-Type
Opcode  |   rs   |   rt    |   rd   |  shamt | funct
0000_00 | 00_000 | 0_0000_ | 0000_0 | 000_00 | 00_0000

I-Type
Opcode  |   rs   |   rt    | imm
0000_00 | 00_000 | 0_0000_ | 0000_0000_0000_0000

J-Type
Opcode  |  rel  |         address           | padding
0000_00 | 00_00 | 00_0000_0000_0000_0000_00 | 00
```
