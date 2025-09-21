use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::io::prelude::*;

pub struct Rom {
    data: HashMap<u32, u8>,
    size: u32,
    pub title: String,
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
        let mut title: Vec<char> = Vec::new();
        for i in 308..324 {
            let value: Option<&u8> = data.get(&i);
            match value {
                Some(byte) => {
                    if *byte != 0 {
                        title.push(*byte as char)
                    }
                }
                None => continue,
            };
        }
        let title: String = title.into_iter().collect();
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

    pub fn get_title(&self) -> &String {
        &self.title
    }
}

pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 65534,
            pc: 256,
        }
    }

    fn nop(&mut self) {
        self.pc += 1;
    }


    fn exec(&mut self, rom: Rom) {
        let op: &u8 = rom.get_value(self.pc as u32);
        match op {
            0 => self.nop(),
            _ => panic!("Instruction not yet implemented"),
        }
    }
}

impl eframe::App for Cpu {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("rgb-emu CPU visualizer");
            ui.label(format!("A: {:X?}", self.a));
            ui.label(format!("B: {:X?}", self.b));
            ui.label(format!("C: {:X?}", self.c));
            ui.label(format!("D: {:X?}", self.d));
            ui.label(format!("E: {:X?}", self.e));
            ui.label(format!("F: {:X?}", self.f));
            ui.label(format!("H: {:X?}", self.h));
            ui.label(format!("L: {:X?}", self.l));
            ui.label(format!("SP: {:X?}", self.sp));
            ui.label(format!("PC: {:X?}", self.pc));
        });
    }
}
