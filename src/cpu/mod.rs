use crate::ram::RAM;
use std::fs::File;
use std::io::Read;
mod registry;
use registry::RegistryFile;
mod opcode;
use opcode::Opcode;
mod constants;
use constants::*;

/// Macro to reverse bit order.
/// 
/// # Examples
/// 
/// Single byte
/// 
/// ```rust
/// use mips_emulator::reverse_bits;
/// 
/// let b = reverse_bits![0b0000_0011];
/// assert_eq!(b, 0b1100_0000);
/// ```
/// 
/// Always 1 byte per set
/// 
/// ```rust, should_panic
/// use mips_emulator::reverse_bits;
/// 
/// let b = reverse_bits![0b0000_0000_0001];
/// assert_eq!(b, 0b1000_0000_0000);
/// ```
/// 
/// Set of bytes
/// 
/// ```rust
/// use mips_emulator::reverse_bits;
/// 
/// let bb = reverse_bits![0b0000_0011, 0b0000_1100];
/// assert_eq!(bb, 0b0011_0000_1100_0000);
/// ```
#[macro_export]
macro_rules! reverse_bits {
    ($($x : expr), *) => {
        {
            let mut b_vec: Vec<bool> = Vec::new();

            $(
                let mut t_vec: Vec<bool> = Vec::new();

                let mut i: u8 = 1u8;
    
                while i > 0 {
                    t_vec.push($x & i != 0);

                    i = i << 1;
                }

                t_vec.append(&mut b_vec);

                b_vec = t_vec;
            )*
    
            let mut binary: usize = 0;
            let mut shift: u32 = 0;

            for n in b_vec.iter().rev() {
                binary += (if *n { 1 } else {0}) << shift;

                shift += 1;
            }

            binary
        }
    }
}

pub fn bytestoword(b1: u8, b2: u8) -> u16 {
    ((b1 as u16) << 8) + b2 as u16 
}

pub struct CPU {

    active: bool,
    pub registry: registry::RegistryFile,
    pub ram: RAM

}

impl CPU {

    pub fn new(rom_path: String) -> CPU {
        let mut file = File::open(&rom_path).unwrap();

        let mut header_buf = vec![0u8; 0xA];
        file.read(&mut header_buf).unwrap();

        let registry_size: usize = ((header_buf[0x5] as usize) << 8) + (header_buf[0x4] as usize);
        let mut registry_buf = vec![0u8; registry_size];
        file.read(&mut registry_buf).unwrap();

        let data_size: usize = ((header_buf[0x7] as usize) << 8) + (header_buf[0x6] as usize);
        let mut data_buf = vec![0u8; data_size];
        file.read(&mut data_buf).unwrap();

        let main_size: usize = ((header_buf[0x9] as usize) << 8) + (header_buf[0x8] as usize);
        let mut main_buf = vec![0u8; main_size];
        file.read(&mut main_buf).unwrap();

        let mut ram = RAM::new(RAM_SIZE);
        let pc = RAM_SIZE - main_size;
        let mut registry = RegistryFile::new(pc, REGISTRY_SIZE);

        let mut j: usize = 0;

        while j < registry_buf.len() - 1 {
            registry.registers[j / 2] = ((registry_buf[j + 1] as u16) << 8) + (registry_buf[j] as u16);
            j += 2;
        }

        for (i, c) in data_buf.iter().enumerate() {
            ram.data[i] = *c;
        }

        for (i, c) in main_buf.iter().enumerate() {
            ram.data[pc + i] = *c;
        }

        CPU {
            active: true,
            ram,
            registry
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Returns Opcode, RS, RT at PC.
    fn read_header(&self) -> (u8, u8, u8) {
        let data: u16 = reverse_bits![self.ram.read_byte(self.registry.pc), self.ram.read_byte(self.registry.pc + 1)] as u16;

        let opcode: u8 = (data & 0b11_1111) as u8;
        let rs: u8 = ((data & 0b111_1100_0000) >> 6) as u8;
        let rt: u8 = ((data & 0b1111_1000_0000_0000) >> 11) as u8;

        (opcode, rs, rt)
    }

    /// Returns immediate value at PC.
    fn read_itype(&self) -> u16 {
        reverse_bits![self.ram.read_byte(self.registry.pc + 2), self.ram.read_byte(self.registry.pc + 3)] as u16
    }

    /// Returns RD, shamt, and funct at PC.
    fn read_rtype(&self) -> (u8, u8, u8) {
        let data: u16 = reverse_bits![self.ram.read_byte(self.registry.pc + 2), self.ram.read_byte(self.registry.pc + 3)] as u16;

        let rd: u8 = (data & 0b1_1111) as u8;
        let shamt: u8 = ((data & 0b11_1110_0000) >> 5) as u8;
        let funct: u8 = ((data & 0b1111_1100_0000_0000) >> 10) as u8;

        (rd, shamt, funct)
    }

    pub fn step(&mut self) {

        println!("+-----------------+");
        println!("| Pipeline Viewer |");
        println!("+-----------------+\n");

        println!("F-Box");
        println!("-----");
        println!("Fetching instruction at ram[PC => {:#x}]", self.registry.pc);

        if self.registry.pc >= self.ram.size {
            self.active = false;
            println!("End of program");
            println!("\n======================");
            return;
        }

        let (opcode, rs, rt) = self.read_header();

        if rs == 0x0 && rt == 0x0 {
            self.active = false;
            println!("End of program");
            println!("\n======================");
            return;
        }

        println!("\nD-Box");
        println!("-----");

        let d_out: (Opcode, u8, u8) = match opcode {
            0x0 => {
                let (rd, shamt, funct) = self.read_rtype();
        
                (match funct {
                    0x19 => { println!("multu r{}, r{}, r{}", rd, rs, rt); Opcode::MULTU },
                    0x21 => { println!("addu r{}, r{}, r{}", rd, rs, rt); Opcode::ADDU },
                    _ => todo!("Function {:#x}", funct)
                }, rd, shamt)
            }

            _ => {
                let imm = self.read_itype();
                let uimm: u8 = (imm >> 8) as u8;
                let limm: u8 = (imm & 0b1111_1111) as u8;
        
                (match opcode {
                    0x8 => { println!("addi r{}, r{}, {}", rt, rs, imm); Opcode::ADDI },
                    0x9 => { println!("addiu r{}, r{}, {}", rt, rs, imm); Opcode::ADDIU },
                    0x20 => { println!("lb r{}, {}(r{})", rt, imm, rs); Opcode::LB },
                    0x23 => { println!("lw r{}, {}(r{})", rt, imm, rs); Opcode::LW },
                    0x28 => { println!("sb r{}, {}(r{})", rt, imm, rs); Opcode::SB },
                    0x2B => { println!("sw r{}, {}(r{})", rt, imm, rs); Opcode::SW },
                    _ => todo!("Opcode {:#x}", opcode)
                }, uimm, limm)
            }
        };

        println!("\nX-Box");
        println!("-----");

        let mut x_out: usize = 0;

        match d_out.0 {
            Opcode::ADDI => {
                let imm = bytestoword(d_out.1, d_out.2);

                let is_negative: bool = imm & 0b1000_0000_0000_0000 == 1;

                let rs_word: u16 = self.registry.read_word(rs);

                if is_negative {
                    self.registry.write_word(rt, rs_word - (imm & 0b0111_1111_1111_1111));
                } else {
                    self.registry.write_word(rt, rs_word + (imm & 0b0111_1111_1111_1111));
                }

                println!("r{} = r{} {} {} => {:#x}", rt, rs, if is_negative {"-"} else {"+"}, imm, self.registry.read_word(rt));
            }

            Opcode::ADDIU => {
                let imm = bytestoword(d_out.1, d_out.2);

                let rs_word: u16 = self.registry.read_word(rs);

                self.registry.write_word(rt, rs_word + imm);

                println!("r{} = r{} + {} => {:#x}", rt, rs, imm, self.registry.read_word(rt));
            }

            Opcode::ADDU => {
                let rd = d_out.1;

                let rs_word: u16 = self.registry.read_word(rs);
                let rt_word: u16 = self.registry.read_word(rt);

                self.registry.write_word(rd, rs_word + rt_word);

                println!("r{} = r{} + r{} => {:#x}", rd, rs, rt, self.registry.read_word(rd));
            }

            Opcode::MULTU => {

                let rd = d_out.1;
                let rtv = self.registry.read_word(rt) as usize;
                let rsv = self.registry.read_word(rs) as usize;

                self.registry.write_word(rd, (rtv * rsv) as u16);

                println!("r{} = r{} * r{} => {:#x}", rd, rs, rt, self.registry.read_word(rd));
            }

            Opcode::LB | Opcode::SB => {
                let imm = bytestoword(d_out.1, d_out.2);

                x_out = self.registry.read_byte(rs) as usize + imm as usize;

                println!("X = r{} + {} => {:#x}", rs, imm, x_out);
            }

            Opcode::LW | Opcode::SW => {
                let imm = bytestoword(d_out.1, d_out.2);

                x_out = self.registry.read_word(rs) as usize + imm as usize;

                println!("X = r{} + {} => {:#x}", rs, imm, x_out);
            }

            _ => println!("No Execution")
        }

        println!("\nM-Box");
        println!("-----");

        let mut m_out: usize = 0;

        match d_out.0 {
            Opcode::LB => {
                self.registry.write_byte(rt, self.ram.read_byte(x_out));

                println!("r{} = ram[X => {}] => {:#x}", rt, x_out, self.registry.read_word(rt));
            }

            Opcode::SB => {
                m_out = self.registry.read_byte(rt) as usize;

                println!("M = r{} => {:#x}", rt, m_out);
            }

            Opcode::LW => {
                self.registry.write_word(rt, self.ram.read_word(x_out));

                println!("r{} = ram[X => {}] => {:#x}", rt, x_out, self.registry.read_word(rt));
            }

            Opcode::SW => {
                m_out = self.registry.read_word(rt) as usize;

                println!("M = r{} => {:#x}", rt, m_out);
            }

            _ => println!("No Execution")
        };

        println!("\nW-Box");
        println!("-----");

        match d_out.0 {
            Opcode::SB => {
                self.ram.write_byte(x_out, m_out as u8);

                println!("ram[X => {}] = M => {:#x}", x_out, m_out);
            }

            Opcode::SW => {
                self.ram.write_word(x_out, m_out as u16);

                println!("ram[X => {}] = M => {:#x}", x_out, m_out);
            }

            _ => println!("No Execution")
        }

        match opcode {
            _ => self.registry.pc += 4
        }

        println!("\n======================");
    }
}