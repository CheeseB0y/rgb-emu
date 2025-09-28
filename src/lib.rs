use eframe::App;
use std::collections::HashMap;
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

pub struct Rom {
    data: HashMap<u16, u8>, // Program will crash when attempting to read ROMs larger than 64KiB to be fixed later
    pub title: String,
    cart_type: CartType,
    rom_size: u32,
    rom_banks: u32,
    ram_size: u32,
    ram_banks: u32,
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
        let data: HashMap<u16, u8> = Rom::read_rom(path);
        let mut title: Vec<char> = Vec::new();
        for i in 0x0134..0x0143 {
            let value: Option<&u8> = data.get(&i);
            match value {
                Some(byte) => {
                    if *byte != 0x00 {
                        title.push(*byte as char)
                    }
                }
                None => continue,
            };
        }
        let title: String = title.into_iter().collect();
        let cart_type: &u8 = match data.get(&0x0147) {
            Some(value) => value,
            None => &0x00,
        };
        let cart_type: CartType = Rom::get_cart_type(cart_type);
        let rom_size: &u8 = match data.get(&0x0148) {
            Some(value) => value,
            None => &0x00,
        };
        let (rom_size, rom_banks) = Rom::get_rom_size_banks(rom_size);
        let ram_size: &u8 = match data.get(&0x0149) {
            Some(value) => value,
            None => &0,
        };
        let (ram_size, ram_banks) = Rom::get_ram_size_banks(ram_size);
        Self {
            data: data,
            title: title,
            cart_type: cart_type,
            rom_size: rom_size,
            rom_banks: rom_banks,
            ram_size: ram_size,
            ram_banks: ram_banks,
        }
    }

    fn read_rom(path: &String) -> HashMap<u16, u8> {
        let file: Result<File> = File::open(path);

        let file: File = match file {
            Ok(f) => f,
            Err(e) => panic!("ROM file not found. {e}"),
        };

        let mut data: HashMap<u16, u8> = HashMap::new();
        for (addr, byte) in (0_u16..).zip(file.bytes()) {
            match byte {
                Ok(entry) => data.insert(addr, entry),
                Err(e) => {
                    eprintln!("Unable to parse byte at addr: {:X?}. Error: {e}", addr);
                    continue;
                }
            };
        }
        data
    }

    fn get_cart_type(byte: &u8) -> CartType {
        match byte {
            0x00 => CartType::ROMONLY,
            0x01 => CartType::MBC1,
            0x02 => CartType::MBC1RAM,
            0x03 => CartType::MBC1RAMBATTERY,
            0x05 => CartType::MBC2,
            0x06 => CartType::MBC2BATTERY,
            0x08 => CartType::ROMRAM,
            0x09 => CartType::ROMRAMBATTERY,
            0x0B => CartType::MMM01,
            0x0C => CartType::MMM01RAM,
            0x0D => CartType::MMM01RAMBATTERY,
            0x0F => CartType::MBC3TIMERBATTERY,
            0x10 => CartType::MBC3TIMERRAMBATTERY,
            0x11 => CartType::MBC3,
            0x12 => CartType::MBC3RAM,
            0x13 => CartType::MBC3RAMBATTERY,
            0x19 => CartType::MBC5,
            0x1A => CartType::MBC5RAM,
            0x1B => CartType::MBC5RAMBATTERY,
            0x1C => CartType::MBC5RUMBLE,
            0x1D => CartType::MBC5RUMBLERAM,
            0x1E => CartType::MBC5RUMBLERAMBATTERY,
            0x20 => CartType::MBC6,
            0x22 => CartType::MBC7SENSORRUMBLERAMBATTERY,
            0xFC => CartType::POCKETCAMERA,
            0xFD => CartType::BANDAITAMA5,
            0xFE => CartType::HuC3,
            0xFF => CartType::HuC1RAMBATTERY,
            _ => panic!("Unknown cartridge type: {:X?}", byte),
        }
    }

    fn get_rom_size_banks(byte: &u8) -> (u32, u32) {
        match byte {
            0x00 => (32768, 2),
            0x01 => (65536, 4),
            0x02 => (131072, 8),
            0x03 => (262144, 16),
            0x04 => (524288, 32),
            0x05 => (1048576, 64),
            0x06 => (2097152, 128),
            0x07 => (4194304, 256),
            0x08 => (8388608, 512),
            _ => panic!("Invalid rom size value: {:X?}", byte),
        }
    }

    fn get_ram_size_banks(byte: &u8) -> (u32, u32) {
        match byte {
            0x00 => (0, 0),
            0x02 => (8192, 1),
            0x03 => (32768, 4),
            0x04 => (131072, 16),
            0x05 => (65536, 8),
            _ => panic!("Invalid ram size value: {:X?}", byte),
        }
    }

    pub fn rom_size(&self) -> &u32 {
        &self.rom_size
    }

    pub fn get_value(&self, addr: u16) -> &u8 {
        match &self.data.get(&addr) {
            Some(byte) => byte,
            None => &0x00,
        }
    }

    pub fn print_rom(&self) {
        for addr in 0..self.rom_size {
            print!("{:X?}:", addr);
            println!("{:X?}", &self.get_value(addr as u16));
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
    membus: MemBus,
}

impl Cpu {
    pub fn new(membus: MemBus) -> Self {
        Cpu {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
            sp: 0xFFFE,
            pc: 0x0100,
            membus: membus,
        }
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    // 00
    fn nop(&mut self) {
        self.pc += 1;
    }
    // 01
    fn ldbcn16(&mut self) {
        self.pc += 1;
        self.b = *self.membus.access(self.pc);
        self.pc += 1;
        self.c = *self.membus.access(self.pc);
        self.pc += 1;
    }
    // 02
    fn ldbca(&mut self, ) {
        self.pc += 1;
        self.membus.write(self.get_bc(), self.a);
        self.pc += 1;
    }
    // 03
    fn incbc(&mut self) {
        panic!("Instruction not yet implemented");
    }
    // 04
    fn incb(&mut self) {
        panic!("Instruction not yet implemented");
    }

    pub fn exec(&mut self, rom: &Rom) {
        let op: &u8 = &rom.get_value(self.pc);
        match op {
            0x00 => self.nop(),
            0x01 => self.ldbcn16(),
            0x02 => self.ldbca(),
            0x03 => self.incbc(),
            0x04 => self.incb(),
            _ => panic!("Instruction not yet implemented"),
        }
    }
}

pub struct Wram {
    data: HashMap<u16, u8>,
}
impl Wram {
    pub fn new() -> Self {
        Wram {data: HashMap::new()}
    }
    pub fn set_value(&mut self, addr: u16, entry: u8) {
        self.data.insert(addr, entry);
    }
    pub fn get_value(&self, addr: u16) -> &u8 {
        match &self.data.get(&addr) {
            Some(byte) => byte,
            None => &0x00,
        }
    }
}
pub struct Vram {
    data: HashMap<u16, u8>,
}
impl Vram {
    pub fn new() -> Self {
        Vram {data: HashMap::new()}
    }
    pub fn set_value(&mut self, addr: u16, entry: u8) {
        self.data.insert(addr, entry);
    }
    pub fn get_value(&self, addr: u16) -> &u8 {
        match &self.data.get(&addr) {
            Some(byte) => byte,
            None => &0x00,
        }
    }
}

pub struct MemBus {
    rom: Rom,
    wram: Wram,
    vram: Vram,
}

impl MemBus {
    pub fn new(rom: Rom) -> Self {
        MemBus {
            rom: rom,
            wram: Wram::new(),
            vram: Vram::new(),
        }
    }

    fn access(&self, addr: u16) -> &u8 {
        match addr {
            0x0000..=0x3FFF => self.rom.get_value(addr),
            0x4000..=0x7FFF => self.rom.get_value(addr), // This should be able to access switchable rom banks through a mapper, to be fixed later.
            0x8000..=0x9FFF => self.vram.get_value(addr),
            0xA000..=0xBFFF => &0x00, // should access external ram on cartridge
            0xC000..=0xCFFF => self.wram.get_value(addr),
            0xD000..=0xDFFF => self.wram.get_value(addr), // This should also be a switchable bank to be fixed later
            0xE000..=0xFDFF => &0x00, // Echo RAM. Can be ignored.
            0xFE00..=0xFE9F => &0x00, // Object attribute memory
            0xFEA0..=0xFEFF => &0xFF, // Not usable, ignore.
            0xFF00..=0xFF7F => &0x00, // IO registers
            0xFF80..=0xFFFE => &0x00, // High RAM
            0xFFFF => &0x00, // Interrupt register
        }
    }

    fn write(&mut self, addr: u16, entry: u8) {
        match addr {
            0x0000..=0x3FFF => eprintln!("Attempted to write to ROM address {addr}"),
            0x4000..=0x7FFF => eprintln!("Attempted to write to ROM address {addr}"),
            0x8000..=0x9FFF => self.vram.set_value(addr, entry),
            0xA000..=0xBFFF => (), // should access external ram on cartridge
            0xC000..=0xCFFF => self.wram.set_value(addr, entry),
            0xD000..=0xDFFF => self.wram.set_value(addr, entry), // This should be a switchable bank to be fixed later
            0xE000..=0xFDFF => eprintln!("Attempted to write to echo RAM address {addr}"), 
            0xFE00..=0xFE9F => (), // Object attribute memory
            0xFEA0..=0xFEFF => eprintln!("Attempted to write to unuasable space address {addr}"),
            0xFF00..=0xFF7F => (), // IO registers
            0xFF80..=0xFFFE => (), // High RAM
            0xFFFF => (), // Interrupt register
        };
    }
}

pub struct Gui {
    cpu: Cpu,
}

impl Gui {
    pub fn new(cpu: Cpu) -> Self {
        Gui { cpu: cpu }
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
