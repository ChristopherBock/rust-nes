/*
Useful documentation on opcodes:
https://www.nesdev.org/obelisk-6502-guide/reference.html
 */

use crate::cpu::AddressingMode;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub struct OpCode {
    pub code: u8,
    pub name: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new (code: u8, name: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code: code,
            name: name,
            len: len,
            cycles: cycles,
            mode: mode,
        }
    }
}

lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OpCode> = vec![
        OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x0A, "ASL", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x1E, "ASL", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),

        OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0x4C, "JMP", 3, 2, AddressingMode::Absolute),
        OpCode::new(0x6C, "JMP", 3, 5, AddressingMode::Indirect),

        OpCode::new(0x20, "JSR", 3, 6, AddressingMode::Absolute),

        OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
        // in case of a page crossing 0xB1 is one cycle longer
        OpCode::new(0xB1, "LDA", 2, 5, AddressingMode::IndirectY),
        OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPageX),
        // in case of a page crossing 0xB9 is one cycle longer
        OpCode::new(0xB9, "LDA", 3, 4, AddressingMode::AbsoluteY),
        // in case of a page crossing 0xBD is one cycle longer
        OpCode::new(0xBD, "LDA", 3, 4, AddressingMode::AbsoluteX),

        OpCode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPageY),
        // in case of a page crossing 0xBE is one cycle longer
        OpCode::new(0xBE, "LDX", 3, 4, AddressingMode::AbsoluteY),

        OpCode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPageX),
        // in case of a page crossing 0xBC is one cycle longer
        OpCode::new(0xBC, "LDY", 3, 4, AddressingMode::AbsoluteX),

        OpCode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x2A, "ROL", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x2E, "ROL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x3E, "ROL", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x60, "RTS", 1, 6, AddressingMode::NoneAddressing),

        OpCode::new(0x38, "SEC", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::AbsoluteY),
        OpCode::new(0x9D, "STA", 3, 5, AddressingMode::AbsoluteX),

        OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPageY),

        OpCode::new(0x84, "STY", 2, 6, AddressingMode::ZeroPage),
        OpCode::new(0x8C, "STY", 2, 3, AddressingMode::ZeroPageX),
        OpCode::new(0x94, "STY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xA8, "TAY", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xBA, "TSX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x8A, "TXA", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x9A, "TXS", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x98, "TYA", 1, 2, AddressingMode::NoneAddressing),
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for cpuop in &*CPU_OPS_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}
