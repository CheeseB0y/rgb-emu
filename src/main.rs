use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()>{
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    let rom = BufReader::new(File::open(path)?);

    for byte in rom.bytes() {
        println!("{}", byte?);
    }

    Ok(())

}
