extern crate bitflags;

/* iNES 1.0 format
+--------+--------------------------------------------+
| Byte   |                Description                 |
+--------+--------------------------------------------+
| 00-03  | "NES" followed by 0x1A (File signature)    |
| 04     | PRG ROM Size in 16 KB units                |
| 05     | CHR ROM Size in 8 KB units                 |
| 06     | Flags 6 (Mapper, Mirroring, Battery, etc.) |
| 07     | Flags 7 (Mapper, VS/PlayChoice, NES 2.0)   |
| 08     | PRG RAM Size in 8 KB units (0 = 8 KB)      |
| 09     | Flags 9 (TV System, NTSC/PAL)              |
| 10     | Flags 10 (Unused, PRG RAM, TV System)      |
| 11-15  | Unused (Should be zero-filled)             |
+--------+--------------------------------------------+
 */
const NES_SIGNATURE: u32 = 0x4E45531A;

const PROGRAM_ROM_PAGE_SIZE: usize = 0x4000;
const CHARACTER_ROM_PAGE_SIZE: usize = 0x2000;

/* Flags 6
+-------+--------------------------------------------+
|  Bit  |                  Description               |
+-------+--------------------------------------------+
|   7   | Mapper Number (Low bit 4)                  |
+-------+--------------------------------------------+
|   6   | Mapper Number (Low bit 3)                  |
+-------+--------------------------------------------+
|   5   | Mapper Number (Low bit 2)                  |
+-------+--------------------------------------------+
|   4   | Mapper Number (Low bit 1)                  |
+-------+--------------------------------------------+
|   3   | Four-Screen VRAM                           |
|       | 0: No                                      |
|       | 1: Yes                                     |
+-------+--------------------------------------------+
|   2   | Trainer                                    |
|       | 0: No                                      |
|       | 1: 512-byte trainer at $7000-$71FF         |
+-------+--------------------------------------------+
|   1   | Battery-Backed Save RAM                    |
|       | 0: No                                      |
|       | 1: Yes                                     |
+-------+--------------------------------------------+
|   0   | Mirroring                                  |
|       | 0: Horizontal Mirroring                    |
|       | 1: Vertical Mirroring                      |
+-------+--------------------------------------------+

 */
bitflags::bitflags! {
    struct Flags6 : u8 {
        const VerticalMirroring = 0b0000_0001;
        const BatteryBacke = 0b0000_0010;
        const TrainerData = 0b0000_0100;
        const FourScreenVRAM = 0b0000_1000;
        const Mapper0 = 0b0001_0000;
        const Mapper1 = 0b0010_0000;
        const Mapper2 = 0b0100_0000;
        const Mapper3 = 0b1000_0000;
    }
}

/* Flags 7
+-------+---------------------------------------------------+
|  Bit  |                    Description                   |
+-------+---------------------------------------------------+
|   7   | Mapper Number (bit 7)                            |
+-------+---------------------------------------------------+
|   6   | Mapper Number (bit 6)                            |
+-------+---------------------------------------------------+
|   5   | Mapper Number (bit 5)                            |
+-------+---------------------------------------------------+
|   4   | Mapper Number (bit 4)                            |
+-------+---------------------------------------------------+
|   3   | VS Unisystem                                     |
|       | 0: No                                            |
|       | 1: Yes                                           |
+-------+---------------------------------------------------+
|   2   | PlayChoice-10                                    |
|       | 0: No                                            |
|       | 1: Yes                                           |
+-------+---------------------------------------------------+
|   1   | NES 2.0 Format                                   |
|       | 0: iNES format (standard)                        |
|       | 1: NES 2.0 format                                |
+-------+---------------------------------------------------+
|   0   | Unused (Should be 0)                             |
+-------+---------------------------------------------------+
 */
bitflags::bitflags! {
    struct Flags7 : u8 {
        const Unused = 0b0000_0001;
        const iNES2Format = 0b0000_0010;
        const PlayChoice = 0b0000_0100;
        const VSUniSystem = 0b0000_1000;
        const Mapper4 = 0b0001_0000;
        const Mapper5 = 0b0010_0000;
        const Mapper6 = 0b0100_0000;
        const Mapper7 = 0b1000_0000;
    }
}

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

pub struct Cartridge {
    pub program_rom: Vec<u8>,
    pub character_rom: Vec<u8>,
    flags_6: Flags6,
    flags_7: Flags7,
}

impl Cartridge {
    pub fn new (raw_data: &Vec<u8>) -> Result<Cartridge, String> {
        if raw_data.len() < 16 {
            return Err("File does not seem to be in the correct format".to_string());
        }

        let parsed_signature = u32::from_le_bytes(raw_data[..4].try_into().unwrap_or([0, 0, 0, 0]));
        if parsed_signature != NES_SIGNATURE {
            return Err(format!("Rom signature is not correct, I read {:x}!", parsed_signature));
        }

        let program_rom_size = raw_data[4] as usize * PROGRAM_ROM_PAGE_SIZE;
        let character_rom_size = raw_data[5] as usize * CHARACTER_ROM_PAGE_SIZE;

        let flags_6 = Flags6::from_bits(raw_data[6]).unwrap();
        let flags_7 = Flags7::from_bits(raw_data[7]).unwrap();

        let program_rom_start = 16 + if flags_6.contains(Flags6::TrainerData) {512} else {0};
        let character_rom_start = program_rom_start + program_rom_size;

        Ok(Cartridge {
            program_rom: raw_data[program_rom_start .. (program_rom_start + program_rom_size)].to_vec(),
            character_rom: raw_data[character_rom_start .. (character_rom_start + character_rom_size)].to_vec(),
            flags_6,
            flags_7,
        })
    }
}

pub fn create_test_cartridge(dummy_trainer_data: bool) -> Cartridge {
    let mut raw_data = Vec::new();
    raw_data.extend_from_slice(&NES_SIGNATURE.to_le_bytes());

    let mut flags_6 = Flags6::from_bits(0b0000_0000).unwrap();
    let flags_7 = Flags7::from_bits(0b0000_0000).unwrap();

    if dummy_trainer_data {
        flags_6 = flags_6 | Flags6::TrainerData;
    }

    let program_rom_pages: u8 = 2;
    let character_rom_pages: u8 = 0;

    raw_data.push(program_rom_pages);
    raw_data.push(character_rom_pages);
    
    raw_data.push(flags_6.bits());
    raw_data.push(flags_7.bits());

    raw_data.push(0); // size of ram on cartridge
    raw_data.push(0);

    raw_data.extend_from_slice(&[0u8; 6]);

    if flags_6.contains(Flags6::TrainerData) {
        raw_data.extend_from_slice(&[3u8; 512]);
    }

    for _i in 0 .. program_rom_pages {
        raw_data.extend_from_slice(&[0u8; 0x4000]);
    }

    for _i in 0 .. character_rom_pages {
        raw_data.extend_from_slice(&[0u8; 0x4000]);
    }

    Cartridge::new(&raw_data).unwrap()
}