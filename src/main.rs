use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io::Result;
use std::collections::HashMap;

fn main(){

    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    let rom_file: BufReader<File> = read_rom(path);
    let mut addr: u32 = 0;
    let mut rom = HashMap::new();

    for byte in rom_file.bytes() {
        let b: u8 = match byte {
            Ok(b) => b,
            Err(e) => panic!("Unable to read byte: {e}"),
        };
        rom.insert(addr,b);
        print!("{:X?}", addr);
        println!(":{:X?}", b);
        addr = addr + 1;
    }
}

fn read_rom(path: &String) -> BufReader<File> {
    let file_result: Result<File> = File::open(path);

    let file: File = match file_result {
        Ok(f) => f,
        Err(e) => panic!("ROM file not found. {e}"),
    };

    BufReader::new(file)
}

// struct Rom {
//     path: String,
//     data: HashMap<u32, u8>,
// }