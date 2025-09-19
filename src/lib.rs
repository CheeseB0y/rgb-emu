use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::io::prelude::*;
pub struct Rom {
    data: HashMap<u32, u8>,
    size: u32,
    pub title: Vec<u8>,
}

impl Rom {
    pub fn new(path: &String) -> Self {
        let file: BufReader<File> = Rom::read_rom(path);
        let mut data: HashMap<u32, u8> = HashMap::new();
        let mut addr: u32 = 0;
        for byte in file.bytes() {
            match byte {
                Ok(entry) => data.insert(addr, entry),
                Err(e) => {
                    eprintln!("Unable to parse byte at addr: {:X?}", addr);
                    panic!("{e}");
                }
            };
            addr += 1;
        }
        let data: HashMap<u32, u8> = data;
        let mut title: Vec<u8> = Vec::new();
        for i in 309..324 {
            let value: Option<&u8> = data.get(&i);
            match value {
                Some(byte) => title.push(byte.clone()),
                None => title.push(0),
            };
        }
        Self {
            data: data,
            size: addr,
            title: title,
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

    pub fn get_value(&self, addr: u32) -> &u8 {
        let get: &Option<&u8> = &self.data.get(&addr);
        match get {
            Some(byte) => byte,
            None => &0,
        }
    }

    pub fn rom_size(&self) -> &u32 {
        &self.size
    }

    pub fn print_rom(&self) {
        for addr in 0..self.size as i32 {
            print!("{:X?}:", addr);
            println!("{:X?}", &self.get_value(addr as u32));
        }
    }
}
