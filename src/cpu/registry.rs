pub struct RegistryFile {

    pub size: usize,
    pub pc: usize,
    pub registers: Vec<u16>

}

impl RegistryFile {
    pub fn new(pc: usize, size: usize) -> RegistryFile {
        RegistryFile {
            pc,
            size,
            registers: vec![0u16; size]
        }
    }

    pub fn print(&self) {

        println!("");

        println!("+-----------------+");
        println!("| Registry Viewer |");
        println!("+-----------------+\n");

        for i in 0..self.size {
            print!("r{}\t", i);
        }

        println!("");

        for register in self.registers.iter() {
            print!("{:#06x}\t", register);
        }

        println!("");

    }

    pub fn write_byte(&mut self, register: u8, byte: u8) {
        self.registers[register as usize] = byte as u16;
    }

    pub fn write_word(&mut self, register: u8, word: u16) {
        self.registers[register as usize] = word as u16;
    }

    pub fn read_byte(&mut self, register: u8) -> u8 {
        self.registers[register as usize] as u8
    }

    pub fn read_word(&mut self, register: u8) -> u16 {
        self.registers[register as usize] as u16
    }
}