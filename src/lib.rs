use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::io::prelude::*;
use eframe::App;

pub struct Rom {
    data: HashMap<u32, u8>,
    size: u32,
    pub title: String,
    cart_type: CartType,
}

enum CartType {
    ROMONLY,
    MBC1,
    MBC1RAM,
    MBC1RAMBATTERY,
    MBC2,
    MBC2BATTERY,
    ROMRAM,
    ROMRAMBATTERY,
    MMM01,
    MMM01RAM,
    MMM01RAMBATTERY,
    MBC3TIMERBATTERY,
    MBC3TIMERRAMBATTERY,
    MBC3,
    MBC3RAM,
    MBC3RAMBATTERY,
    MBC5,
    MBC5RAM,
    MBC5RAMBATTERY,
    MBC5RUMBLE,
    MBC5RUMBLERAM,
    MBC5RUMBLERAMBATTERY,
    MBC6,
    MBC7SENSORRUMBLERAMBATTERY,
    POCKETCAMERA,
    BANDAITAMA5,
    HuC3,
    HuC1RAMBATTERY,
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
        let cart_type_value = match data.get(&327) {
            Some(value) => value,
            None => &0,
        };
        let cart_type: CartType = Rom::get_cart_type(cart_type_value);
        Self {
            data: data,
            size: addr,
            title: title,
            cart_type: cart_type,
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

    fn get_cart_type(value: &u8) -> CartType {
        match value {
            0 => CartType::ROMONLY,
            1 => CartType::MBC1,
            2 => CartType::MBC1RAM,
            3 => CartType::MBC1RAMBATTERY,
            5 => CartType::MBC2,
            6 => CartType::MBC2BATTERY,
            8 => CartType::ROMRAM,
            9 => CartType::ROMRAMBATTERY,
            11 => CartType::MMM01,
            12 => CartType::MMM01RAM,
            13 => CartType::MMM01RAMBATTERY,
            15 => CartType::MBC3TIMERBATTERY,
            16 => CartType::MBC3TIMERRAMBATTERY,
            17 => CartType::MBC3,
            18 => CartType::MBC3RAM,
            19 => CartType::MBC3RAMBATTERY,
            25 => CartType::MBC5,
            26 => CartType::MBC5RAM,
            27 => CartType::MBC5RAMBATTERY,
            28 => CartType::MBC5RUMBLE,
            29 => CartType::MBC5RUMBLERAM,
            30 => CartType::MBC5RUMBLERAMBATTERY,
            32 => CartType::MBC6,
            34 => CartType::MBC7SENSORRUMBLERAMBATTERY,
            252 => CartType::POCKETCAMERA,
            253 => CartType::BANDAITAMA5,
            254 => CartType::HuC3,
            255 => CartType::HuC1RAMBATTERY,
            _ => panic!("Unknown cartridge type: {:X?}", value),
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

    // 00
    fn nop(&mut self) {
        self.pc += 1;
    }
    // 01
    fn ldbcn16(&mut self, rom: &Rom) {
        self.pc += 1;
        self.b = rom.get_value(self.pc as u32).clone();
        self.pc += 1;
        self.c = rom.get_value(self.pc as u32).clone();
        self.pc += 1;
    }
    // 02
    fn ldbca(&mut self, rom: &Rom) {
        panic!("Instruction not yet implemented");
    }
    // 03
    fn incbc (&mut self) {
        panic!("Instruction not yet implemented");
    }
    // 04
    fn incb (&mut self) {
        panic!("Instruction not yet implemented");
    }

    pub fn exec(&mut self, rom: &Rom) {
        let op: &u8 = &rom.get_value(self.pc as u32);
        match op {
            0 => self.nop(),
            1 => self.ldbcn16(&rom),
            2 => self.ldbca(&rom),
            3 => self.incbc(),
            4 => self.incb(),
            _ => panic!("Instruction not yet implemented"),
        }
    }
}

pub struct Ram {
    data: HashMap<u16, u8>,
}
pub struct Vram {
    data: HashMap<u16, u8>,
}

pub struct MemBus {
    rom: Rom,
    ram: Ram,
    vram: Vram,
}

impl MemBus {
    pub fn new(rom: Rom, ram: Ram, vram: Vram) -> Self {
        MemBus {
            rom: rom,
            ram: ram,
            vram: vram
        }
    }
    
    // fn access(addr: u16) -> u8 {
    //     if addr < 16384 {

    //     }
    // }
}

pub struct Gui {
    cpu: Cpu,
}

impl Gui {
    pub fn new(cpu: Cpu) -> Self {
        Gui {
            cpu: cpu,
        }
    }
}

impl App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("rgb-emu CPU visualizer");
            ui.label(format!("A: {:X?}", self.cpu.a));
            ui.label(format!("B: {:X?}", self.cpu.b));
            ui.label(format!("C: {:X?}", self.cpu.c));
            ui.label(format!("D: {:X?}", self.cpu.d));
            ui.label(format!("E: {:X?}", self.cpu.e));
            ui.label(format!("F: {:X?}", self.cpu.f));
            ui.label(format!("H: {:X?}", self.cpu.h));
            ui.label(format!("L: {:X?}", self.cpu.l));
            ui.label(format!("SP: {:X?}", self.cpu.sp));
            ui.label(format!("PC: {:X?}", self.cpu.pc));
        });
    }
}
