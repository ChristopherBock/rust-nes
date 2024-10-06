use crate::mem::Mem;
use crate::cartridge::Cartridge;

/*
NES memory map illustrated using ChatGPT 4o

+--------------------------+ 0xFFFF (65535)
|   Upper Bank (Mirrored)   | <- 0xC000 - 0xFFFF: PRG-ROM (Mirrored)
+--------------------------+ 0x8000 (32768)
|      PRG-ROM Bank 1       | <- 0x8000 - 0xBFFF: PRG-ROM Bank 1
+--------------------------+ 0x6000 (24576)
|   Cartridge Save RAM      | <- 0x6000 - 0x7FFF: Save RAM (Optional)
+--------------------------+ 0x4020 (16416)
| Expansion ROM/Registers   | <- 0x4020 - 0x5FFF: Expansion ROM (Rarely used)
+--------------------------+ 0x4000 (16384)
|    I/O Registers          | <- 0x4000 - 0x401F: I/O Registers (APU/IO)
+--------------------------+ 0x2008 (8200)
|    PPU Registers          | <- 0x2008 - 0x3FFF: PPU Registers (Mirrored)
+--------------------------+ 0x2000 (8192)
|    PPU Registers          | <- 0x2000 - 0x2007: PPU Registers
+--------------------------+ 0x0800 (2048)
|    Internal RAM (Mirror)  | <- 0x0800 - 0x1FFF: Internal RAM (Mirrored)
+--------------------------+ 0x0000 (0)
|      Internal RAM         | <- 0x0000 - 0x07FF: Internal RAM (2 KB)
+--------------------------+
*/

const RAM_START: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;

const CARTRIDGE_START: u16 = 0x8000;
const PROGRAM_START_ADDRESS: u16 = 0xFFFC;
const PROGRAM_START_END_ADDRESS: u16 = 0xFFFE;

#[derive(Debug, PartialEq)]
enum BusReadFrom {
    CpuRam,
    CartridgeProgramRom,
}

pub struct Bus {
    cpu_ram: [u8; 0x0800],
    program_start: u16, // this is a workaround until ROM loading is implemented
    cartridge: Cartridge,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Bus {
            cpu_ram: [0; 0x0800],
            program_start: 0,
            cartridge
        }
    }

    fn match_address(addr: u16, program_rom_mirrored: bool) -> (BusReadFrom, u16) {
        match addr {
            RAM_START .. RAM_MIRRORS_END => {
                let real_addr = addr & 0b0000_0111_1111_1111;
                (BusReadFrom::CpuRam, real_addr)
            },
            CARTRIDGE_START .. PROGRAM_START_END_ADDRESS => {
                let mut real_addr = addr - CARTRIDGE_START;
                if program_rom_mirrored && real_addr >= 0x4000{
                    real_addr = real_addr - 0x4000;
                }
                (BusReadFrom::CartridgeProgramRom, real_addr)
            },
            _ => {
                println!("Read at {:x} not yet implemented", addr);
                todo!()
            },
        }
    }
}

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        let (read_from, real_addr) = Bus::match_address(addr, self.cartridge.is_program_rom_mirrored);
        match read_from {
            BusReadFrom::CpuRam => self.cpu_ram[real_addr as usize],
            BusReadFrom::CartridgeProgramRom => self.cartridge.program_rom[real_addr as usize],
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        let (write_to, real_addr) = Bus::match_address(addr, self.cartridge.is_program_rom_mirrored);
        match write_to {
            BusReadFrom::CpuRam => {self.cpu_ram[real_addr as usize] = data;},
            BusReadFrom::CartridgeProgramRom => {
                panic!("Write to cartridge rom detected!")
            }
        }
    }
}