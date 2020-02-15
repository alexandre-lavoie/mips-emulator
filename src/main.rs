mod cpu;
use cpu::CPU;
mod ram;
use std::env;

fn main() {
    println!("+-----------------------------+");
    println!("| MIPS Emulator               |");
    println!("| Created by Alexandre Lavoie |");
    println!("+-----------------------------+\n");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: mips-emulator FILE");

        return;
    }

    let mut cpu = CPU::new(args[1].to_string());

    println!("Running file: {}", args[1].to_string());
    println!("RAM size: {:#x}", cpu.ram.size);
    println!("RF size (16-bit registers): {:#x}", cpu.registry.size);
    println!("ROM address: {:#x}", cpu.registry.pc);

    println!("\n======================");
    
    while cpu.is_active() {
        cpu.registry.print();
        cpu.ram.print(cpu.registry.pc);
        cpu.step();
    }
}
