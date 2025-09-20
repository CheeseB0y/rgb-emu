use rgb_emu::{Cpu, Rom};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    let rom: Rom = Rom::new(path);
    let cpu: Cpu = Cpu::new();
    // rom.print_rom();
    // println!("{}", rom.get_title());
}
