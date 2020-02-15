# mips-emulator
MIPS Emulator

# Instruction Types

```
R-Type (Register)
Opcode  |   rs   |   rt    |   rd   |  shamt | funct
0000_00 | 00_000 | 0_0000_ | 0000_0 | 000_00 | 00_0000

I-Type (Immediate)
Opcode  |   rs   |   rt    | imm
0000_00 | 00_000 | 0_0000_ | 0000_0000_0000_0000

J-Type (Jump)
Opcode  |  rel  |         address           | padding
0000_00 | 00_00 | 00_0000_0000_0000_0000_00 | 00
```

# Opcodes

```
addi rs, rt, imm -> Add immediate
addiu rs, rt, imm -> Add immediate unsigned
lb rt, imm(rs) -> Load byte
lw rt, imm(rs) -> Load word
sb rt, imm(rs) -> Store byte
sw rt, imm(rs) -> Store word
```

# Functions

```
multu rd, rs, rt -> Multiply to register unsigned
addu rd, rs, rt -> Add to register unsigned
```

