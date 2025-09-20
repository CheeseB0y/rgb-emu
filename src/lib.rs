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
            sp: 0,
            pc: 0,
        }
    }

    fn get_af(&self) -> (u8, u8) {
        (self.a, self.f)
    }
    fn get_bc(&self) -> (u8, u8) {
        (self.b, self.c)
    }
    fn get_de(&self) -> (u8, u8) {
        (self.d, self.e)
    }
    fn get_hl(&self) -> (u8, u8) {
        (self.h, self.l)
    }
    fn get_sp(&self) -> u16 {
        self.sp
    }
    fn get_pc(&self) -> u16 {
        self.pc
    }
    fn get_a(&self) -> u8 {
        self.a
    }
    fn get_b(&self) -> u8 {
        self.b
    }
    fn get_c(&self) -> u8 {
        self.c
    }
    fn get_d(&self) -> u8 {
        self.d
    }
    fn get_e(&self) -> u8 {
        self.e
    }
    fn get_f(&self) -> u8 {
        self.f
    }
    fn get_h(&self) -> u8 {
        self.h
    }
    fn get_l(&self) -> u8 {
        self.l
    }

    fn set_af(&mut self, a: u8, f: u8) {
        self.a = a;
        self.f = f;
    }
    fn set_bc(&mut self, b: u8, c: u8) {
        self.b = b;
        self.c = c;
    }
    fn set_de(&mut self, d: u8, e: u8) {
        self.d = d;
        self.e = e;
    }
    fn set_hl(&mut self, h: u8, l: u8) {
        self.h = h;
        self.l = l;
    }
    fn set_sp(&mut self, sp: u16) {
        self.sp = sp;
    }
    fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }
    fn set_a(&mut self, a: u8) {
        self.a = a;
    }
    fn set_b(&mut self, b: u8) {
        self.b = b;
    }
    fn set_c(&mut self, c: u8) {
        self.c = c;
    }
    fn set_d(&mut self, d: u8) {
        self.d = d;
    }
    fn set_e(&mut self, e: u8) {
        self.e = e;
    }
    fn set_f(&mut self, f: u8) {
        self.f = f;
    }
    fn set_h(&mut self, h: u8) {
        self.h = h;
    }
    fn set_l(&mut self, l: u8) {
        self.l = l;
    }
}

impl eframe::App for Cpu {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("rgb-emu CPU visualizer");
            ui.label(format!("A: {}", self.get_a()));
            ui.label(format!("B: {}", self.get_b()));
            ui.label(format!("C: {}", self.get_c()));
            ui.label(format!("D: {}", self.get_d()));
            ui.label(format!("E: {}", self.get_e()));
            ui.label(format!("F: {}", self.get_f()));
            ui.label(format!("H: {}", self.get_h()));
            ui.label(format!("L: {}", self.get_l()));
            ui.label(format!("SP: {}", self.get_sp()));
            ui.label(format!("PC: {}", self.get_pc()));
        });
    }
}
