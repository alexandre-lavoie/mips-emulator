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

/// Returns RS, RT at PC.
/// 
/// # Example
/// 
/// ```rust
/// use mips_emulator::fetch_registers;
/// 
/// let mut ram = [0u8; 64];
/// ram[0] = 0b0000_0001;
/// ram[1] = 0b1100_1000;
/// assert_eq!(fetch_registers(&ram, 0), (0b0_1110, 0b0_0010));
/// ```
pub fn fetch_registers(ram: &[u8], pc: usize) -> (u8, u8) {

    let data: u16 = reverse_bits![ram[pc], ram[pc + 1]] as u16;

    let rs: u8 = ((data & 0b111_1100_0000) >> 6) as u8;
    let rt: u8 = ((data & 0b1111_1000_0000_0000) >> 11) as u8;
    return (rs, rt);
}

/// Returns the opcode at PC.
/// 
/// # Example
/// 
/// ```rust
/// use mips_emulator::fetch_opcode;
/// 
/// let mut ram = [0u8; 64];
/// ram[0] = 0b1100_1000;
/// assert_eq!(fetch_opcode(&ram, 0), 0b1_0011);
/// ```
pub fn fetch_opcode(ram: &[u8], pc: usize) -> u8 {

    let data: u16 = reverse_bits![ram[pc], ram[pc + 1]] as u16;

    return (data as u8) & 0b11_1111;
}

/// Returns immediate value at PC.
/// 
/// # Example
/// 
/// ```rust
/// use mips_emulator::fetch_immediate;
/// 
/// let mut ram = [0u8; 64];
/// ram[2] = 0b1100_1000;
/// ram[3] = 0b1100_1000;
/// assert_eq!(fetch_immediate(&ram, 0), 0b0001_0011_0001_0011);
/// ```
pub fn fetch_immediate(ram: &[u8], pc: usize) -> u16 {
    return reverse_bits![ram[pc + 2], ram[pc + 3]] as u16;
}

/// Returns RD, shamt, and funct from PC.
/// 
/// # Example
/// 
/// ```rust
/// use mips_emulator::fetch_immediate;
/// 
/// let mut ram = [0u8; 64];
/// ram[2] = 0b1100_1000;
/// ram[3] = 0b1100_1000;
/// assert_eq!(fetch_immediate(&ram, 0), (0b1_0011, 0b1_1000, 0b10));
/// ```
pub fn fetch_rtype(ram: &[u8], pc: usize) -> (u8, u8, u8) {

    let data: u16 = reverse_bits![ram[pc + 2], ram[pc + 3]] as u16;

    let rd: u8 = (data & 0b1_1111) as u8;
    let shamt: u8 = ((data & 0b11_1110_0000) >> 5) as u8;
    let funct: u8 = ((data & 0b1111_1100_0000_0000) >> 10) as u8;

    return (rd, shamt, funct);
}