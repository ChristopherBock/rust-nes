use crate::mem::Mem;

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

pub struct Bus {
    cpu_ram: [u8; 0x0800],
    program_start: u16, // this is a workaround until ROM loading is implemented
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_ram: [0; 0x0800],
            program_start: 0,
        }
    }

    fn match_address(addr: u16) -> u16 {
        match addr {
            RAM_START .. RAM_MIRRORS_END => {
                let real_addr = addr & 0b0000_0111_1111_1111;
                real_addr
            },
            0xFFFC..0xFFFE => {
                addr
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
        let real_addr = Bus::match_address(addr);
        if real_addr == 0xFFFC {
            (self.program_start & 0xFF) as u8
        } else if real_addr == 0xFFFD {
            (self.program_start >> 8) as u8
        } else {
            self.cpu_ram[real_addr as usize]
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        let real_addr = Bus::match_address(addr);
        if real_addr == 0xFFFC {
            self.program_start = (self.program_start & 0xFF00) + (data as u16);
        } else if real_addr == 0xFFFD {
            self.program_start = (self.program_start & 0x00FF) + ((data as u16) << 8);
        } else {
            self.cpu_ram[real_addr as usize] = data;
        }
    }
}