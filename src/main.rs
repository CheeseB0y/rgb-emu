use rgb_emu::Rom;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    let rom: Rom = Rom::new(path);
    rom.print_rom();
}
