mod lib;
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

        let opcode = lib::fetch_opcode(&ram, pc);
        let (rs, rt) = lib::fetch_registers(&ram, pc);
        let (rd, shamt, funct) = lib::fetch_rtype(&ram, pc);
        let imm = lib::fetch_immediate(&ram, pc);

        // D-Box

        // Instruction Type
        match opcode {
            0x0 => println!("R-Type Intruction"),
            0x2 | 0x3 => println!("J-Type Instruction"),
            _ => println!("I-Type Instruction")
        }

        // Instruction Structure
        match opcode {
            0x0 => {
                match funct {
                    0x21 => println!("addu rd, rs, rt"),

                    _ => todo!()
                }
            }
            0x8 => println!("addi rt, rs, imm"),
            0x9 => println!("addiu rt, rs, imm"),
            0x20 => println!("lb rt, imm(rs)"),
            0x28 => println!("sb rt, imm(rs)"),
            _ => todo!()
        }

        // X-Box

        let mut x_out: usize = 0;
        match opcode {
            0x0 => {
                match funct {
                    0x21 => {
                        registry_file[rd as usize] = registry_file[rs as usize] + registry_file[rt as usize];

                        println!("$r{} = $r{} + $r{}", rd, rs, rt);
                    }

                    _ => println!("No Execution")
                }
            }

            0x8 => {
                if imm & 0b1000_0000 == 1 {
                    registry_file[rt as usize] = registry_file[rs as usize] - imm & 0b0111_1111;

                    println!("$r{} = $r{} - {}", rt, rs, imm);
                } else {
                    registry_file[rt as usize] = registry_file[rs as usize] + imm & 0b0111_1111;

                    println!("$r{} = $r{} + {}", rt, rs, imm);
                }
            }

            0x9 => {
                registry_file[rt as usize] = registry_file[rs as usize] + imm;

                println!("$r{} = $r{} + {}", rt, rs, imm);
            }

            0x20 | 0x28 => {
                x_out = (registry_file[rs as usize] + shamt as u16) as usize;
                println!("X = $r{} + {}", rs, shamt);
            }

            _ => println!("No X Execution")
        }
        
        // M-Box

        let mut m_out: usize = 0;
        match opcode {
            0x20 => {
                registry_file[rt as usize] = reverse_bits![ram[x_out]] as u16;

                println!("$r{} = ram[{}] | 1 byte", rt, x_out);
            }

            0x28 => {
                m_out = registry_file[rs as usize] as usize;

                println!("M = $r{} -> {}", rt, m_out);
            }

            _ => println!("No M Execution")
        }

        // W-Box

        match opcode {
            0x28 => {
                ram[x_out as usize] = m_out as u8;

                println!("ram[X] = M | 1 byte");
            }

            _ => println!("No W Execution")
        }

        pc += 2;
    }
}
