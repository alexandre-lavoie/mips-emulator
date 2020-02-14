mod lib;
mod enums;
mod constants;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: mips-emulator [FILE]");

        return;
    }

    let mut ram = [0u8; constants::RAM_SIZE];

    let mut registry_file = [0u16; constants::REGISTRY_FILE_SIZE];

    let mut pc: usize = constants::ROM_ADDRESS;

    while pc != constants::HALT_ADDRESS {

        // F-Box

        let opcode_hex = lib::fetch_opcode(&ram, pc);
        let (rs, rt) = lib::fetch_registers(&ram, pc);
        let (rd, shamt, funct_hex) = lib::fetch_rtype(&ram, pc);
        let imm = lib::fetch_immediate(&ram, pc);

        // D-Box

        // Instruction Type
        match opcode_hex {
            0x0 => println!("R-Type Intruction"),
            0x2 | 0x3 => println!("J-Type Instruction"),
            _ => println!("I-Type Instruction")
        }

        // Instruction Structure
        let (opcode, funct): (enums::Opcode, enums::Function) = match opcode_hex {
            0x0 => {
                match funct_hex {
                    0x21 => { println!("addu rd, rs, rt"); (enums::Opcode::FUNCT, enums::Function::ADDU) },

                    _ => todo!()
                }
            }
            0x8 => { println!("addi rt, rs, imm"); (enums::Opcode::ADDI, enums::Function::NULL) },
            0x9 => { println!("addiu rt, rs, imm"); (enums::Opcode::ADDIU, enums::Function::NULL) },
            0x20 => { println!("lb rt, imm(rs)"); (enums::Opcode::LB, enums::Function::NULL) },
            0x28 => { println!("sb rt, imm(rs)"); (enums::Opcode::SB, enums::Function::NULL) },
            _ => todo!()
        };

        // X-Box

        let x_out: usize = match opcode {
            enums::Opcode::FUNCT => {
                match funct {
                    enums::Function::ADDU => {
                        registry_file[rd as usize] = registry_file[rs as usize] + registry_file[rt as usize];

                        println!("$r{} = $r{} + $r{}", rd, rs, rt);

                        0
                    }

                    _ => {println!("No Execution"); 0}
                }
            }

            enums::Opcode::ADDI => {
                if imm & 0b1000_0000 == 1 {
                    registry_file[rt as usize] = registry_file[rs as usize] - imm & 0b0111_1111;

                    println!("$r{} = $r{} - {}", rt, rs, imm);
                } else {
                    registry_file[rt as usize] = registry_file[rs as usize] + imm & 0b0111_1111;

                    println!("$r{} = $r{} + {}", rt, rs, imm);
                }

                0
            }

            enums::Opcode::ADDIU => {
                registry_file[rt as usize] = registry_file[rs as usize] + imm;

                println!("$r{} = $r{} + {}", rt, rs, imm);

                0
            }

            enums::Opcode::LB | enums::Opcode::SB => {
                println!("X = $r{} + {}", rs, shamt);

                (registry_file[rs as usize] + shamt as u16) as usize
            }

            _ => {println!("No X Execution"); 0}
        };
        
        // M-Box

        let m_out: usize = match opcode {
            enums::Opcode::LB => {
                println!("$r{} = ram[{}] | 1 byte", rt, x_out);

                registry_file[rt as usize] = reverse_bits![ram[x_out]] as u16;

                0
            }

            enums::Opcode::SB => {
                println!("M = $r{} -> {}", rt, registry_file[rs as usize]);

                registry_file[rs as usize] as usize
            }

            _ => {println!("No M Execution"); 0}
        };

        // W-Box

        match opcode {
            enums::Opcode::SB => {
                ram[x_out as usize] = m_out as u8;

                println!("ram[X] = M | 1 byte");
            }

            _ => println!("No W Execution")
        }

        pc += 2;
    }
}
