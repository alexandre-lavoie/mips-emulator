pub struct RAM {
    pub size: usize,
    pub data: Vec<u8>,
    last_modified_address: Vec<usize>
}

impl RAM {
    pub fn new(size: usize) -> RAM {
        RAM {
            size,
            data: vec![0u8; size],
            last_modified_address: vec![]
        }
    }

    pub fn print(&mut self, pc: usize) {
        let width: usize = 0x10;

        println!("");

        println!("+------------+");
        println!("| RAM Viewer |");
        println!("+------------+\n");
        println!("[{:#04x}] => Position of PC\n", pc);
        
        if self.last_modified_address.len() > 0 {
            println!("_{:#04x}_ => Position of Modified Memory\n", self.last_modified_address[0]);
        }

        print!("\t");

        for i in 0..width {
            print!("{:#03x}\t", i);
        }

        println!();

        for (i, e) in self.data.iter().enumerate() {
            if i % width == 0 {
                print!("\n{:#04x}:\t", i / width);
            }

            if pc == i {
                print!("[{:#04x}]\t", e);
            } else if self.last_modified_address.contains(&i) {
                print!("_{:#04x}_\t", e);
            } else {
                print!("{:#04x}\t", e);
            } 
        }

        self.last_modified_address = vec![];

        println!("\n");
    }

    pub fn write_byte(&mut self, address: usize, byte: u8) {
        self.last_modified_address = vec![address];
        self.data[address] = byte;
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.data[address]
    }

    pub fn write_word(&mut self, address: usize, word: u16) {
        self.last_modified_address = vec![address, address + 1];
        self.data[address] = (word & 0b1111_1111) as u8;
        self.data[address + 1] = (word >> 8) as u8;
    }

    pub fn read_word(&self, address: usize) -> u16 {
        let hb = self.data[address + 1] as u16;
        let lb = self.data[address] as u16;

        if hb == 0 {
            lb
        } else {
            hb << 8 + lb
        }
    }
}
