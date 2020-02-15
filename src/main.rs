mod lib;
mod enums;
mod constants;
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    println!("+---------------+");
    println!("| MIPS Emulator |");
    println!("+---------------+\n");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: mips-emulator FILE");

        return;
    }

    let mut ram = [0u8; constants::RAM_SIZE];
    let file_name: String = args[1].to_string();
    let mut file=File::open(&file_name).unwrap();
    let mut buf=[0u8; constants::REGISTRY_FILE_SIZE / 2];

    file.read(&mut buf).unwrap();
    let mut i: usize = 0;

    for c in buf.iter() {
        ram[constants::ROM_ADDRESS + i] = *c;
        i += 1;
    }

    println!("Running file: {}", file_name);
    println!("RAM size: {}", constants::RAM_SIZE);
    println!("RF size (16-bit registers): {}", constants::REGISTRY_FILE_SIZE);
    println!("ROM address: {}", constants::ROM_ADDRESS);

    let mut registry_file = [0u16; constants::REGISTRY_FILE_SIZE];

    let mut pc: usize = constants::ROM_ADDRESS;

    println!("\n======================\n");

    while ram[pc] != 0 {

        println!("F-Box");
        println!("-----");
        println!("Fetching instruction at ram[PC => {}]", pc);

        let opcode_hex = lib::fetch_opcode(&ram, pc);
        let (rs, rt) = lib::fetch_registers(&ram, pc);
        let (rd, shamt, funct_hex) = lib::fetch_rtype(&ram, pc);
        let imm = lib::fetch_immediate(&ram, pc);

        println!("\nD-Box");
        println!("-----");

        // Instruction Structure
        let (opcode, funct): (enums::Opcode, enums::Function) = match opcode_hex {
            0x0 => {
                // R-Type Instructions
                match funct_hex {
                    0x21 => { println!("addu r{}, r{}, r{}", rd, rs, rt); (enums::Opcode::FUNCT, enums::Function::ADDU) },

                    _ => todo!()
                }
            }
            0x8 => { println!("addi r{}, r{}, {}", rt, rs, imm); (enums::Opcode::ADDI, enums::Function::NULL) },
            0x9 => { println!("addiu r{}, r{}, {}", rt, rs, imm); (enums::Opcode::ADDIU, enums::Function::NULL) },
            0x20 => { println!("lb r{}, {}(r{})", rt, imm, rs); (enums::Opcode::LB, enums::Function::NULL) },
            0x28 => { println!("sb r{}, {}(r{})", rt, imm, rs); (enums::Opcode::SB, enums::Function::NULL) },
            _ => todo!()
        };

        println!("\nX-Box");
        println!("-----");

        let mut x_out: usize = 0; 
        
        match opcode {
            enums::Opcode::FUNCT => {
                match funct {
                    enums::Function::ADDU => {
                        registry_file[rd as usize] = registry_file[rs as usize] + registry_file[rt as usize];

                        println!("r{} = r{} + r{}", rd, rs, rt);
                    }

                    _ => println!("No Execution")
                }
            }

            enums::Opcode::ADDI => {
                let is_negative: bool = imm & 0b1000_0000 == 1;

                if is_negative {
                    registry_file[rt as usize] = registry_file[rs as usize] - imm & 0b0111_1111;
                } else {
                    registry_file[rt as usize] = registry_file[rs as usize] + imm & 0b0111_1111;
                }

                println!("r{} = r{} {} {} => {}", rt, rs, if is_negative {"-"} else {"+"}, imm, registry_file[rt as usize]);
            }

            enums::Opcode::ADDIU => {
                registry_file[rt as usize] = registry_file[rs as usize] + imm;

                println!("r{} = r{} + {} => {}", rt, rs, imm, registry_file[rt as usize]);
            }

            enums::Opcode::LB | enums::Opcode::SB => {
                println!("X = r{} + {}", rs, shamt);

                x_out = (registry_file[rs as usize] + shamt as u16) as usize;
            }

            _ => println!("No Execution")
        };

        println!("\nM-Box");
        println!("-----");

        let mut m_out: usize = 0;
        
        match opcode {
            enums::Opcode::LB => {
                let rv: u16 = reverse_bits![ram[x_out]] as u16;

                println!("r{} = reverse(ram[X => {}]) => {} | 1 byte", rt, x_out, rv);

                registry_file[rt as usize] = rv;
            }

            enums::Opcode::SB => {
                m_out = reverse_bits![(registry_file[rt as usize] & 0b1111_1111) as u8];

                println!("M = reverse(r{}) => {}", rt, m_out);
            }

            _ => println!("No Execution")
        };

        println!("\nW-Box");
        println!("-----");

        match opcode {
            enums::Opcode::SB => {
                ram[x_out as usize] = m_out as u8;

                println!("ram[X => {}] = M => {} | 1 byte", x_out, m_out);
            }

            _ => println!("No Execution")
        }

        pc += 4;

        println!("\n======================\n");
    }
}
