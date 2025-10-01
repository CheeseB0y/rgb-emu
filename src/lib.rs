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

enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    SP,
    PC,
    AF,
    BC,
    DE,
    HL,
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

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x0F) as u8;
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x0F) as u8;
    }
    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x0F) as u8;
    }
    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x0F) as u8;
    }

    fn get_16b_value(&self) -> u16 {
        (*self.membus.access(self.pc) as u16) << 8 | (*self.membus.access(self.pc + 1) as u16)
    }

    fn inc_pc(&mut self) {
        self.pc += 1;
    }

    fn not_implemented(&self) {
        eprintln!("Instruction {:X?} not yet implemented", self.pc)
    }

    // Operations need flag logic and timing logic
    fn nop(&mut self) {
        self.inc_pc();
    }
    fn load_r8r8(&mut self, source: Register, dest: Register) {
        let value: &u8 = match source {
            Register::A => &self.a,
            Register::B => &self.b,
            Register::C => &self.c,
            Register::D => &self.d,
            Register::E => &self.e,
            Register::F => &self.f,
            Register::H => &self.h,
            Register::L => &self.l,
            _ => {
                eprintln!("Invalid register");
                &0x00
            }
        };
        match dest {
            Register::A => self.a = *value,
            Register::B => self.b = *value,
            Register::C => self.c = *value,
            Register::D => self.d = *value,
            Register::E => self.e = *value,
            Register::F => self.f = *value,
            Register::H => self.h = *value,
            Register::L => self.l = *value,
            _ => eprintln!("Invalid register"),
        };
        self.inc_pc();
    }
    fn load_r8n8(&mut self, dest: Register) {
        self.inc_pc();
        match dest {
            Register::A => self.a = *self.membus.access(self.pc),
            Register::B => self.b = *self.membus.access(self.pc),
            Register::C => self.c = *self.membus.access(self.pc),
            Register::D => self.d = *self.membus.access(self.pc),
            Register::E => self.e = *self.membus.access(self.pc),
            Register::F => self.f = *self.membus.access(self.pc),
            Register::H => self.h = *self.membus.access(self.pc),
            Register::L => self.l = *self.membus.access(self.pc),
            _ => eprintln!("Invalid register"),
        };
        self.inc_pc();
    }
    fn load_r16n16(&mut self, dest: Register) {
        self.inc_pc();
        match dest {
            Register::AF => self.set_af(self.get_16b_value()),
            Register::BC => self.set_bc(self.get_16b_value()),
            Register::DE => self.set_de(self.get_16b_value()),
            Register::HL => self.set_hl(self.get_16b_value()),
            Register::PC => self.pc = self.get_16b_value(),
            Register::SP => self.sp = self.get_16b_value(),
            _ => eprintln!("Invalid register"),
        }
        self.inc_pc();
        self.inc_pc();
    }

    fn exec(&mut self) {
        let op: &u8 = self.membus.access(self.pc);
        match op {
            0x00 => self.nop(),
            0x01 => self.load_r16n16(Register::BC),
            0x02 => self.not_implemented(),
            0x03 => self.not_implemented(),
            0x04 => self.not_implemented(),
            0x05 => self.not_implemented(),
            0x06 => self.load_r8n8(Register::B),
            0x07 => self.not_implemented(),
            0x08 => self.not_implemented(),
            0x09 => self.not_implemented(),
            0x0A => self.not_implemented(),
            0x0B => self.not_implemented(),
            0x0C => self.not_implemented(),
            0x0D => self.not_implemented(),
            0x0E => self.load_r8n8(Register::C),
            0x0F => self.not_implemented(),
            0x10 => self.not_implemented(),
            0x11 => self.not_implemented(),
            0x12 => self.not_implemented(),
            0x13 => self.not_implemented(),
            0x14 => self.not_implemented(),
            0x15 => self.not_implemented(),
            0x16 => self.load_r8n8(Register::D),
            0x17 => self.not_implemented(),
            0x18 => self.not_implemented(),
            0x19 => self.not_implemented(),
            0x1A => self.not_implemented(),
            0x1B => self.not_implemented(),
            0x1C => self.not_implemented(),
            0x1D => self.not_implemented(),
            0x1E => self.load_r8n8(Register::E),
            0x1F => self.not_implemented(),
            0x20 => self.not_implemented(),
            0x21 => self.not_implemented(),
            0x22 => self.not_implemented(),
            0x23 => self.not_implemented(),
            0x24 => self.not_implemented(),
            0x25 => self.not_implemented(),
            0x26 => self.load_r8n8(Register::H),
            0x27 => self.not_implemented(),
            0x28 => self.not_implemented(),
            0x29 => self.not_implemented(),
            0x2A => self.not_implemented(),
            0x2B => self.not_implemented(),
            0x2C => self.not_implemented(),
            0x2D => self.not_implemented(),
            0x2E => self.load_r8n8(Register::L),
            0x2F => self.not_implemented(),
            0x30 => self.not_implemented(),
            0x31 => self.not_implemented(),
            0x32 => self.not_implemented(),
            0x33 => self.not_implemented(),
            0x34 => self.not_implemented(),
            0x35 => self.not_implemented(),
            0x36 => self.not_implemented(),
            0x37 => self.not_implemented(),
            0x38 => self.not_implemented(),
            0x39 => self.not_implemented(),
            0x3A => self.not_implemented(),
            0x3B => self.not_implemented(),
            0x3C => self.not_implemented(),
            0x3D => self.not_implemented(),
            0x3E => self.load_r8n8(Register::A),
            0x3F => self.not_implemented(),
            0x40 => self.load_r8r8(Register::B, Register::B),
            0x41 => self.load_r8r8(Register::C, Register::B),
            0x42 => self.load_r8r8(Register::D, Register::B),
            0x43 => self.load_r8r8(Register::E, Register::B),
            0x44 => self.load_r8r8(Register::B, Register::H),
            0x45 => self.load_r8r8(Register::B, Register::L),
            0x46 => self.not_implemented(),
            0x47 => self.load_r8r8(Register::B, Register::A),
            0x48 => self.load_r8r8(Register::C, Register::B),
            0x49 => self.load_r8r8(Register::C, Register::C),
            0x4A => self.load_r8r8(Register::C, Register::D),
            0x4B => self.load_r8r8(Register::C, Register::E),
            0x4C => self.load_r8r8(Register::C, Register::H),
            0x4D => self.load_r8r8(Register::C, Register::L),
            0x4E => self.not_implemented(),
            0x4F => self.load_r8r8(Register::C, Register::A),
            0x50 => self.load_r8r8(Register::D, Register::B),
            0x51 => self.load_r8r8(Register::D, Register::C),
            0x52 => self.load_r8r8(Register::D, Register::D),
            0x53 => self.load_r8r8(Register::D, Register::E),
            0x54 => self.load_r8r8(Register::D, Register::H),
            0x55 => self.load_r8r8(Register::D, Register::L),
            0x56 => self.not_implemented(),
            0x57 => self.load_r8r8(Register::D, Register::A),
            0x58 => self.load_r8r8(Register::E, Register::B),
            0x59 => self.load_r8r8(Register::E, Register::C),
            0x5A => self.load_r8r8(Register::E, Register::D),
            0x5B => self.load_r8r8(Register::E, Register::E),
            0x5C => self.load_r8r8(Register::E, Register::H),
            0x5D => self.load_r8r8(Register::E, Register::L),
            0x5E => self.not_implemented(),
            0x5F => self.load_r8r8(Register::E, Register::A),
            0x60 => self.load_r8r8(Register::H, Register::B),
            0x61 => self.load_r8r8(Register::H, Register::C),
            0x62 => self.load_r8r8(Register::H, Register::D),
            0x63 => self.load_r8r8(Register::H, Register::E),
            0x64 => self.load_r8r8(Register::H, Register::H),
            0x65 => self.load_r8r8(Register::H, Register::L),
            0x66 => self.not_implemented(),
            0x67 => self.load_r8r8(Register::H, Register::A),
            0x68 => self.load_r8r8(Register::L, Register::B),
            0x69 => self.load_r8r8(Register::L, Register::C),
            0x6A => self.load_r8r8(Register::L, Register::D),
            0x6B => self.load_r8r8(Register::L, Register::E),
            0x6C => self.load_r8r8(Register::L, Register::H),
            0x6D => self.load_r8r8(Register::L, Register::L),
            0x6E => self.not_implemented(),
            0x6F => self.load_r8r8(Register::L, Register::A),
            0x70 => self.not_implemented(),
            0x71 => self.not_implemented(),
            0x72 => self.not_implemented(),
            0x73 => self.not_implemented(),
            0x74 => self.not_implemented(),
            0x75 => self.not_implemented(),
            0x76 => self.not_implemented(),
            0x77 => self.not_implemented(),
            0x78 => self.load_r8r8(Register::A, Register::B),
            0x79 => self.load_r8r8(Register::A, Register::C),
            0x7A => self.load_r8r8(Register::A, Register::D),
            0x7B => self.load_r8r8(Register::A, Register::E),
            0x7C => self.load_r8r8(Register::A, Register::H),
            0x7D => self.load_r8r8(Register::A, Register::L),
            0x7E => self.not_implemented(),
            0x7F => self.load_r8r8(Register::A, Register::A),
            0x80 => self.not_implemented(),
            0x81 => self.not_implemented(),
            0x82 => self.not_implemented(),
            0x83 => self.not_implemented(),
            0x84 => self.not_implemented(),
            0x85 => self.not_implemented(),
            0x86 => self.not_implemented(),
            0x87 => self.not_implemented(),
            0x88 => self.not_implemented(),
            0x89 => self.not_implemented(),
            0x8A => self.not_implemented(),
            0x8B => self.not_implemented(),
            0x8C => self.not_implemented(),
            0x8D => self.not_implemented(),
            0x8E => self.not_implemented(),
            0x8F => self.not_implemented(),
            0x90 => self.not_implemented(),
            0x91 => self.not_implemented(),
            0x92 => self.not_implemented(),
            0x93 => self.not_implemented(),
            0x94 => self.not_implemented(),
            0x95 => self.not_implemented(),
            0x96 => self.not_implemented(),
            0x97 => self.not_implemented(),
            0x98 => self.not_implemented(),
            0x99 => self.not_implemented(),
            0x9A => self.not_implemented(),
            0x9B => self.not_implemented(),
            0x9C => self.not_implemented(),
            0x9D => self.not_implemented(),
            0x9E => self.not_implemented(),
            0x9F => self.not_implemented(),
            0xA0 => self.not_implemented(),
            0xA1 => self.not_implemented(),
            0xA2 => self.not_implemented(),
            0xA3 => self.not_implemented(),
            0xA4 => self.not_implemented(),
            0xA5 => self.not_implemented(),
            0xA6 => self.not_implemented(),
            0xA7 => self.not_implemented(),
            0xA8 => self.not_implemented(),
            0xA9 => self.not_implemented(),
            0xAA => self.not_implemented(),
            0xAB => self.not_implemented(),
            0xAC => self.not_implemented(),
            0xAD => self.not_implemented(),
            0xAE => self.not_implemented(),
            0xAF => self.not_implemented(),
            0xB0 => self.not_implemented(),
            0xB1 => self.not_implemented(),
            0xB2 => self.not_implemented(),
            0xB3 => self.not_implemented(),
            0xB4 => self.not_implemented(),
            0xB5 => self.not_implemented(),
            0xB6 => self.not_implemented(),
            0xB7 => self.not_implemented(),
            0xB8 => self.not_implemented(),
            0xB9 => self.not_implemented(),
            0xBA => self.not_implemented(),
            0xBB => self.not_implemented(),
            0xBC => self.not_implemented(),
            0xBD => self.not_implemented(),
            0xBE => self.not_implemented(),
            0xBF => self.not_implemented(),
            0xC0 => self.not_implemented(),
            0xC1 => self.not_implemented(),
            0xC2 => self.not_implemented(),
            0xC3 => self.not_implemented(),
            0xC4 => self.not_implemented(),
            0xC5 => self.not_implemented(),
            0xC6 => self.not_implemented(),
            0xC7 => self.not_implemented(),
            0xC8 => self.not_implemented(),
            0xC9 => self.not_implemented(),
            0xCA => self.not_implemented(),
            0xCB => self.not_implemented(),
            0xCC => self.not_implemented(),
            0xCD => self.not_implemented(),
            0xCE => self.not_implemented(),
            0xCF => self.not_implemented(),
            0xD0 => self.not_implemented(),
            0xD1 => self.not_implemented(),
            0xD2 => self.not_implemented(),
            0xD3 => self.not_implemented(),
            0xD4 => self.not_implemented(),
            0xD5 => self.not_implemented(),
            0xD6 => self.not_implemented(),
            0xD7 => self.not_implemented(),
            0xD8 => self.not_implemented(),
            0xD9 => self.not_implemented(),
            0xDA => self.not_implemented(),
            0xDB => self.not_implemented(),
            0xDC => self.not_implemented(),
            0xDD => self.not_implemented(),
            0xDE => self.not_implemented(),
            0xDF => self.not_implemented(),
            0xE0 => self.not_implemented(),
            0xE1 => self.not_implemented(),
            0xE2 => self.not_implemented(),
            0xE3 => self.not_implemented(),
            0xE4 => self.not_implemented(),
            0xE5 => self.not_implemented(),
            0xE6 => self.not_implemented(),
            0xE7 => self.not_implemented(),
            0xE8 => self.not_implemented(),
            0xE9 => self.not_implemented(),
            0xEA => self.not_implemented(),
            0xEB => self.not_implemented(),
            0xEC => self.not_implemented(),
            0xED => self.not_implemented(),
            0xEE => self.not_implemented(),
            0xEF => self.not_implemented(),
            0xF0 => self.not_implemented(),
            0xF1 => self.not_implemented(),
            0xF2 => self.not_implemented(),
            0xF3 => self.not_implemented(),
            0xF4 => self.not_implemented(),
            0xF5 => self.not_implemented(),
            0xF6 => self.not_implemented(),
            0xF7 => self.not_implemented(),
            0xF8 => self.not_implemented(),
            0xF9 => self.not_implemented(),
            0xFA => self.not_implemented(),
            0xFB => self.not_implemented(),
            0xFC => self.not_implemented(),
            0xFD => self.not_implemented(),
            0xFE => self.not_implemented(),
            0xFF => self.not_implemented(),
        };
    }

    pub fn run(&mut self) {
        loop {
            self.exec();
        }
    }
}

pub struct Wram {
    data: HashMap<u16, u8>,
}
impl Wram {
    pub fn new() -> Self {
        Wram {
            data: HashMap::new(),
        }
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
        Vram {
            data: HashMap::new(),
        }
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
            0xE000..=0xFDFF => &0x00,                     // Echo RAM. Can be ignored.
            0xFE00..=0xFE9F => &0x00,                     // Object attribute memory
            0xFEA0..=0xFEFF => &0xFF,                     // Not usable, ignore.
            0xFF00..=0xFF7F => &0x00,                     // IO registers
            0xFF80..=0xFFFE => &0x00,                     // High RAM
            0xFFFF => &0x00,                              // Interrupt register
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
            0xFFFF => (),          // Interrupt register
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
