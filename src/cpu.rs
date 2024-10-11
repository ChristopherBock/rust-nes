/*
Useful documentation:
https://bugzmanov.github.io/nes_ebook/chapter_3_4.html
https://skilldrick.github.io/easy6502/#addressing
https://www.nesdev.org/obelisk-6502-guide/addressing.html
https://www.nesdev.org/obelisk-6502-guide/registers.html

ToDo:
Move to bitflags: https://docs.rs/bitflags/latest/bitflags/
 */

use crate::opcodes;
use crate::mem::Mem;
use crate::bus::Bus;

const STACK_START: u16 = 0x0100;
const STACK_SIZE: u16 = 0x0100;
const STACK_RESET: u8 = 0xFD;
const STATUS_RESET: u8 = 0b0010_0100;

pub struct CPU {
    pub register_a: u8,
    // pushes to the stack decrement the stack pointer
    // pulling from it increments it
    // the stack is located between  0x0100 and  0x01FF
    // register_s is the stack pointer
    pub register_s: u8,
    pub register_x: u8,
    pub register_y: u8,
    // additional info on the status register:
    // https://www.nesdev.org/wiki/Status_flags
    // 7   6   5   4   3   2   1   0
    // N   V   -   B   D   I   Z   C
    pub status: u8,
    pub program_counter: u16,
    pub start_override: u16,
    pub last_mem_write_value: u8,
    pub last_mem_write_value_u16: u16,
    pub last_mem_write_address: u16,
    bus: Bus,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

impl Mem for CPU {
    /*
        Memory access
     */
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data);
        self.last_mem_write_address = addr;
        self.last_mem_write_value = data;
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.mem_read(addr), self.mem_read(addr + 1)])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let bytes = data.to_le_bytes();

        self.mem_write(addr, bytes[0]);
        self.mem_write(addr + 1, bytes[1]);

        self.last_mem_write_address = addr;
        self.last_mem_write_value_u16 = data;
    }
}

/* Additional information
The NES CPU uses little endian addressing: least significant bits first
 -> real adress 0x8000 is stored as 0x00 0x80
 */
impl CPU {
    pub fn new(bus: Bus) -> Self {
        CPU {
            register_a: 0,
            register_s: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            start_override: 0,
            last_mem_write_address: 0,
            last_mem_write_value: 0,
            last_mem_write_value_u16: 0,
            bus,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.load_and_run(program, true, 0x0600);
    }

    pub fn interpret_without_reset(&mut self, program: Vec<u8>, program_base_address: u16) {
        self.program_counter = program_base_address;
        self.load_and_run(program, false, program_base_address);
    }

    pub fn load_and_run (&mut self, program: Vec<u8>, reset: bool, program_base_address: u16) {
        self.load(program, program_base_address);

        if reset {
            self.reset();
        }

        self.run(|_, _|{});
    }

    pub fn load (&mut self, program: Vec<u8>, program_base_address: u16) {
        for i in 0..(program.len() as u16) {
            self.mem_write(program_base_address + i, program[i as usize]);
        }
        self.start_override = program_base_address;
    }

    pub fn reset (&mut self) {
        self.register_a = 0;
        self.register_s = STACK_RESET;
        self.register_x = 0;
        self.register_y = 0;
        self.last_mem_write_value = 0;
        self.last_mem_write_value_u16 = 0;
        self.last_mem_write_address = 0;

        self.status = STATUS_RESET;

        for i in STACK_START..(STACK_SIZE+STACK_START) {
            self.mem_write(i, 0);
        }

        if self.start_override != 0 {
            self.program_counter = self.start_override;
        } else {
            self.program_counter = self.mem_read_u16(0xFFFC)
        }
    }

    pub fn run<F> (&mut self, mut callback: F)
    where 
        F: FnMut(&mut CPU, &&opcodes::OpCode),
    {
        let ref opcodes = *opcodes::OPCODES_MAP;

        loop {
            let code = self.mem_read(self.program_counter);

            let opcode = opcodes.get(&code).expect(
                &format!("OpCode {:x} is not recognized!", code)
            );

            callback(self, opcode);

            self.program_counter += 1;

            let program_counter_state = self.program_counter;

            match code {
                0x61 | 0x65 | 0x69 | 0x6D | 0x71 | 0x75 | 0x79 | 0x7D => {
                    self.adc(&opcode.mode);
                },
                0x0B | 0x2B => {
                    self.anc(&opcode.mode);
                },
                0x21 | 0x25 | 0x29 | 0x2D | 0x31 | 0x35 | 0x39 | 0x3D => {
                    self.and(&opcode.mode);
                },
                0x06 | 0x0A | 0x0E | 0x16 | 0x1E => {
                    self.asl(&opcode.mode);
                },
                0x90 => self.bcc(&opcode.mode),
                0xB0 => self.bcs(&opcode.mode),
                0xF0 => self.beq(&opcode.mode),
                0x24 | 0x2C => {
                    self.bit(&opcode.mode);
                }
                0x30 => self.bmi(&opcode.mode),
                0xD0 => self.bne(&opcode.mode),
                0x10 => self.bpl(&opcode.mode),
                0x50 => self.bvc(&opcode.mode),
                0x70 => self.bvs(&opcode.mode),
                0x18 => self.clc(),
                0xD8 => self.cld(),
                0x58 => self.cli(),
                0xB8 => self.clv(),
                0xC1 | 0xC5 | 0xC9 | 0xCD | 0xD1 | 0xD5 | 0xD9 | 0xDD => {
                    self.cmp(&opcode.mode);
                },
                0xE0 | 0xE4 | 0xEC => {
                    self.cpx(&opcode.mode);
                },
                0xC0 | 0xC4 | 0xCC => {
                    self.cpy(&opcode.mode);
                },
                0xC7| 0xD7| 0xCF| 0xDF| 0xDB| 0xC3| 0xD3 => self.dcp(&opcode.mode),
                0xC6 | 0xCE | 0xD6 | 0xDE => {
                    self.dec(&opcode.mode);
                },
                0xCA => self.dex(),
                0x88 => self.dey(),
                0x41 | 0x45 | 0x49 | 0x4D | 0x51 | 0x55 | 0x59 | 0x5D => {
                    self.eor(&opcode.mode);
                },
                0xE6 | 0xEE | 0xF6 | 0xFE => {
                     self.inc(&opcode.mode);
                },
                0xE8 => self.inx(),
                0xC8 => self.iny(),
                0xE7 | 0xF7 | 0xEF | 0xFF | 0xFB | 0xE3 | 0xF3 => {
                    self.isb(&opcode.mode);
                }
                0x4C | 0x6C => {
                    self.jmp(&opcode.mode);
                },
                0x20 => {
                    self.jsr(&opcode.mode);
                },
                0xA7| 0xB7| 0xAF| 0xBF| 0xA3| 0xB3 => {
                    self.lax(&opcode.mode);
                },
                0xA1 |0xA5 | 0xA9 | 0xAD | 0xB1 | 0xB5 | 0xB9 | 0xBD => {
                    self.lda(&opcode.mode);
                },
                0xA2 | 0xA6 | 0xAE | 0xB6 | 0xBE  => {
                    self.ldx(&opcode.mode);
                },
                0xA0 | 0xA4 | 0xAC | 0xB4 | 0xBC  => {
                    self.ldy(&opcode.mode);
                },
                0x46 | 0x4A | 0x4E | 0x56 | 0x5E => {
                    self.lsr(&opcode.mode);
                },
                // the "normal nop"
                0xEA => self.nop(),
                // illegal nop opcodes
                0x1A| 0x3A| 0x5A| 0x7A| 0xDA| 0xFA => self.nop(),
                // the illegal opcode dops = double no operation
                0x04| 0x14| 0x34| 0x44| 0x54| 0x64| 0x74| 0x80| 0x82| 0x89| 0xC2| 0xD4| 0xE2| 0xF4 => self.nop(),
                // illegal top opcodes = triple no operation
                0x0C| 0x1C| 0x3C| 0x5C| 0x7C| 0xDC| 0xFC => self.nop(),
                0x01 | 0x05 | 0x09 | 0x0D | 0x11 | 0x15 | 0x19 | 0x1D => {
                    self.ora(&opcode.mode);
                },
                0x48 => self.pha(),
                0x08 => self.php(),
                0x68 => self.pla(),
                0x28 => self.plp(),
                0x27 | 0x37 | 0x2F | 0x3F | 0x3B | 0x23 | 0x33 => {
                    self.rla(&opcode.mode);
                }
                0x26 | 0x2A | 0x2E | 0x36 | 0x3E => {
                    self.rol(&opcode.mode);
                },
                0x66 | 0x6A | 0x6E | 0x76 | 0x7E => {
                    self.ror(&opcode.mode);
                },
                0x67 | 0x77 | 0x6F | 0x7F | 0x7B | 0x63 | 0x73 => {
                    self.rra(&opcode.mode);
                }
                0x40 => {
                    self.rti();
                },
                0x60 => {
                    self.rts();
                },
                0x87 | 0x97 | 0x83 | 0x8F => {
                    self.sax(&opcode.mode);
                },
                0xEB => self.sbc(&opcode.mode),
                0xE1 | 0xE5 | 0xE9 | 0xED | 0xF1 | 0xF5 | 0xF9 | 0xFD => {
                    self.sbc(&opcode.mode);
                }
                0x38 => {
                    self.sec();
                },
                0xF8 => {
                    self.sed();
                },
                0x78 => {
                    self.sei();
                },
                0x03 | 0x07 | 0x0F | 0x13 | 0x17 | 0x1B | 0x1F => {
                    self.slo(&opcode.mode);
                },
                0x47 | 0x57 | 0x4F | 0x5F | 0x5B | 0x43 | 0x53 => {
                    self.sre(&opcode.mode);
                }
                0x81 | 0x85 | 0x8D | 0x91 | 0x95 | 0x99 | 0x9D => {
                    self.sta(&opcode.mode);
                },
                0x86 | 0x8E | 0x96 => {
                    self.stx(&opcode.mode);
                },
                0x84 | 0x8C | 0x94 => {
                    self.sty(&opcode.mode);
                },
                0xAA => self.tax(),
                0xA8 => self.tay(),
                0xBA => self.tsx(),
                0x8A => self.txa(),
                0x9A => self.txs(),
                0x98 => self.tya(),
                0x00 => return,
                _ => todo!()
            }

            if self.program_counter == program_counter_state {
                self.program_counter += (opcode.len - 1) as u16
            }
        }
    }

    /*
    Stack Operations
     */
    fn push_stack_u16(&mut self, data: u16) {
        self.mem_write_u16((self.register_s - 1) as u16 + 0x0100, data);
        self.register_s = self.register_s.wrapping_sub(2);
    }

    fn pop_stack_u16(&mut self) -> u16 {
        let value = self.mem_read_u16((self.register_s + 1) as u16 + 0x0100);
        self.register_s = self.register_s.wrapping_add(2);

        value
    }

    fn push_stack(&mut self, data: u8) {
        self.mem_write((self.register_s) as u16 + 0x0100, data);
        self.register_s = self.register_s.wrapping_sub(1);
    }

    fn pop_stack(&mut self) -> u8 {
        let value = self.mem_read((self.register_s + 1) as u16 + 0x0100);
        self.register_s = self.register_s.wrapping_add(1);

        value
    }

    /*
        Operation helpers
     */

    fn add_to_register_a_with_carry(&mut self, value: u8, carry: u8) {
        let result_u16 = value as u16 + self.register_a as u16 + carry as u16;

        if result_u16 > 0xFF {
            self.set_carry();
        } else {
            self.clear_carry();
        }

        let result = result_u16 as u8;
        // check if there is a mathematical overflow due to type length
        if (value ^ result) & (result ^ self.register_a) & 0x80 != 0 {
            self.set_overflow_flag();
        } else {
            self.clear_overflow_flag();
        }

        self.register_a = result as u8;
        self.set_neg_and_zero_flag(self.register_a);
    }

    /*
        OP Codes
     */
    fn adc (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);

        self.add_to_register_a_with_carry(value, self.status & 0b0000_0001);
    }

    fn anc (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);
        let result = self.register_a & value;

        self.register_a = result;
        self.set_neg_and_zero_flag(result);

        if self.is_neg_flag_set(){
            self.set_carry();
        } else {
            self.clear_carry();
        }
    }

    fn and (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);
        let result = self.register_a & value;

        self.register_a = result;
        self.set_neg_and_zero_flag(result);
    }

    fn asl (&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::NoneAddressing {
            let value = self.register_a;

            let result = self.shift_left_and_set_carry(value);

            self.register_a = result;
            self.set_neg_and_zero_flag(result);
        } else {
            let address = self.get_operand_address(mode);
            let value = self.mem_read(address);

            let result = self.shift_left_and_set_carry(value);

            self.mem_write(address, result);
            self.set_neg_and_zero_flag(result);
        }
    }

    fn bcc (&mut self, mode: &AddressingMode) {
        self.branch(mode, !self.is_carry_flag_set());
    }

    fn bcs (&mut self, mode: &AddressingMode) {
        self.branch(mode, self.is_carry_flag_set());
    }

    fn bit (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);

        let result = self.register_a & value;

        if result == 0 {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }

        self.status = self.status & 0b0011_1111;
        self.status = self.status | (value & 0b1100_0000);
    }

    fn beq (&mut self, mode: &AddressingMode) {
        self.branch(mode, self.is_zero_flag_set());
    }

    fn bmi (&mut self, mode: &AddressingMode) {
        self.branch(mode, self.is_neg_flag_set());
    }

    fn bne (&mut self, mode: &AddressingMode) {
        self.branch(mode, !self.is_zero_flag_set());
    }

    fn bpl (&mut self, mode: &AddressingMode) {
        self.branch(mode,!self.is_neg_flag_set());
    }

    fn bvc (&mut self, mode: &AddressingMode) {
        self.branch(mode,!self.is_overflow_flag_set());
    }

    fn bvs (&mut self, mode: &AddressingMode) {
        self.branch(mode,self.is_overflow_flag_set());
    }

    fn clc (&mut self) {
        self.clear_carry();
    }

    fn cld (&mut self) {
        self.clear_decimal_mode_flag();
    }

    fn cli (&mut self) {
        self.clear_interrupt_disable_flag();
    }

    fn clv (&mut self) {
        self.clear_overflow_flag();
    }

    fn cmp (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);
        
        self.compare_and_set_flags(self.register_a, value);
    }

    fn cpx (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);
        
        self.compare_and_set_flags(self.register_x, value);
    }

    fn cpy (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);
        
        self.compare_and_set_flags(self.register_y, value);
    }

    fn dcp (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address).wrapping_sub(1);

        self.mem_write(address, value);
        self.compare_and_set_flags(self.register_a, value);
    }

    fn dec (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);

        let result = value.wrapping_sub(1);

        self.mem_write(address, result);
        self.set_neg_and_zero_flag(result);
    }

    fn dex (&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.set_neg_and_zero_flag(self.register_x);
    }

    fn dey (&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.set_neg_and_zero_flag(self.register_y);
    }

    fn eor (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);

        self.register_a = value ^ self.register_a;
        self.set_neg_and_zero_flag(self.register_a);
    }

    fn inc (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);

        let result = value.wrapping_add(1);

        self.mem_write(address, result);
        self.set_neg_and_zero_flag(result);
    }

    fn inx (&mut self) {
        if self.register_x == 0xff {
            self.register_x = 0
        } else {
            self.register_x += 1;
        }

        self.set_neg_and_zero_flag(self.register_x);
    }

    fn iny (&mut self) {
        self.register_y = self.register_y.wrapping_add(1);

        self.set_neg_and_zero_flag(self.register_y);
    }

    fn isb (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);

        let value_incremented = value.wrapping_add(1);
        let carry = self.status & 0b0000_0001;

        self.mem_write(address, value_incremented);

        self.add_to_register_a_with_carry(!value_incremented, carry);
    }

    // jumps
    fn jmp (&mut self, mode: &AddressingMode) {
        // opposed to other instructions the operand specifies in absolute addressing mode the address to jump to, so where to find the next instructions
        // the indirect one specifies an address of an address
        // as a consequence the parsing is already all done by our get_operand_address function
        let addr = self.get_operand_address(mode);

        self.program_counter = addr;
    }

    fn jsr (&mut self, mode: &AddressingMode) {
        let return_address = self.program_counter + 1;
        self.push_stack_u16(return_address);

        self.jmp(mode)
    }

    fn lax (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a = self.mem_read(addr);
        self.register_x = self.register_a;
        self.set_neg_and_zero_flag(self.register_a);
    }

    // ld* operations
    fn lda (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_a = self.mem_read(addr);
        self.set_neg_and_zero_flag(self.register_a);
    }

    fn ldx (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_x = self.mem_read(addr);
        self.set_neg_and_zero_flag(self.register_x);
    }

    fn ldy (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.register_y = self.mem_read(addr);
        self.set_neg_and_zero_flag(self.register_y);
    }

    fn lsr (&mut self, mode: &AddressingMode) {
        let mut value = self.register_a;
        let mut address = 0;
        if *mode != AddressingMode::NoneAddressing {
            address = self.get_operand_address(mode);
            value = self.mem_read(address);
        }

        let new_carry = value & 0b0000_0001;
        let result = value >> 1;

        self.status = (self.status & 0b111_1110) + new_carry;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            self.mem_write(address, result);
        }

        self.set_neg_and_zero_flag(result);
    }

    fn nop (&mut self) {
        return;
    }

    fn ora (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);

        self.register_a = value | self.register_a;
        self.set_neg_and_zero_flag(self.register_a);
    }

    fn pha (&mut self) {
        self.push_stack(self.register_a);
    }

    fn php (&mut self) {
        let status = self.status | 0b0011_0000;
        self.push_stack(status);
    }

    fn pla (&mut self) {
        self.register_a = self.pop_stack();
        self.set_neg_and_zero_flag(self.register_a);
    }

    fn plp (&mut self) {
        self.status = (self.pop_stack() & 0b1110_1111) | 0b0010_0000;
    }

    fn rla (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let roled_value = (value << 1) + (self.status & 0b0000_0001);
        self.mem_write(addr, roled_value);

        if (value & 0b1000_0000) > 0 {
            self.set_carry();
        } else {
            self.clear_carry();
        }

        self.set_neg_and_zero_flag(roled_value);
        self.register_a = self.register_a & roled_value;
    }

    fn rol (&mut self, mode: &AddressingMode) {
        let mut value = self.register_a;
        let mut address = 0;
        if *mode != AddressingMode::NoneAddressing {
            address = self.get_operand_address(mode);
            value = self.mem_read(address);
        }

        let new_carry = (value & 0b1000_0000) >> 7;
        let result = (value << 1) + (self.status & 0b0000_0001);

        self.status = (self.status & 0b1111_1110) + new_carry;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            self.mem_write(address, result);
        }

        self.set_neg_and_zero_flag(result);
    }

    fn ror (&mut self, mode: &AddressingMode) {
        let mut value = self.register_a;
        let mut address = 0;
        if *mode != AddressingMode::NoneAddressing {
            address = self.get_operand_address(mode);
            value = self.mem_read(address);
        }

        let new_carry = value & 0b0000_0001;
        let result = (value >> 1) + ((self.status & 0b0000_0001) << 7);

        self.status = (self.status & 0b1111_1110) + new_carry;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            self.mem_write(address, result);
        }

        self.set_neg_and_zero_flag(result);
    }

    fn rra (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = (value >> 1) + if self.is_carry_flag_set() {0x80} else {0x00};
        self.mem_write(addr, result);

        if (value & 0b0000_0001) > 0 {
            self.set_carry();
        } else {
            self.clear_carry();
        }

        let carry: u8 = self.status & 0b0000_0001;

        self.add_to_register_a_with_carry(result, carry);
    }

    // jump and interrupt returns

    fn rti (&mut self) {
        self.plp();

        let return_address = self.pop_stack_u16();
        self.program_counter = return_address;
    }

    fn rts (&mut self) {
        let return_address = self.pop_stack_u16();
        self.program_counter = return_address + 1;
    }

    fn sax (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let result = self.register_x & self.register_a;

        self.mem_write(address, result);
    }

    // sbc: A - M - (1 - C)
    // sbc uses two complement arithmetic
    // therefore we build the two complement of the number to be subtracted
    // the two complement is built by inverting all bits and adding 1 to
    // the result, this one cancels with the one from the (1- C) term
    fn sbc (&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = !self.mem_read(address);

        let carry = self.status & 0b0000_0001;
        self.add_to_register_a_with_carry(value, carry);
    }

    fn sec (&mut self) {
        self.set_carry();
    }

    fn sed (&mut self) {
        self.set_decimal_mode_flag();
    }

    fn sei (&mut self) {
        self.set_interrupt_disable_flag();
    }

    fn slo (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        if value & 0b1000_0000 != 0 {
            self.set_carry();
        } else {
            self.clear_carry();
        }

        let shifted_value = value << 1;

        self.mem_write(addr, shifted_value);

        let result = shifted_value | self.register_a;
        self.register_a = result;
        self.set_neg_and_zero_flag(result);
    }

    fn sre (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let lsred_value = value >> 1;
        self.mem_write(addr, lsred_value);

        if (value & 0b0000_0001) > 0 {
            self.set_carry();
        } else {
            self.clear_carry();
        }

        let xor_result = self.register_a ^ lsred_value;
        self.set_neg_and_zero_flag(xor_result);

        self.register_a = xor_result;
    }

    // st* operations
    fn sta (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn stx (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    fn sty (&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }

    // t** operations
    fn tax (&mut self) {
        self.register_x = self.register_a;

        self.set_neg_and_zero_flag(self.register_x);
    }

    fn tay (&mut self) {
        self.register_y = self.register_a;

        self.set_neg_and_zero_flag(self.register_y);
    }

    fn tsx (&mut self) {
        self.register_x = self.register_s;

        self.set_neg_and_zero_flag(self.register_x);
    }

    fn txa (&mut self) {
        self.register_a = self.register_x;

        self.set_neg_and_zero_flag(self.register_x);
    }

    fn txs (&mut self) {
        self.register_s = self.register_x;
    }

    fn tya (&mut self) {
        self.register_a = self.register_y;

        self.set_neg_and_zero_flag(self.register_y);
    }

    /*
        Helper functions
     */
    fn branch (&mut self, mode: &AddressingMode, branch: bool) {
        if branch {
            let address = self.get_operand_address(mode);
            let value = self.mem_read(address);

            if value > 127 {
                self.program_counter += 1 + (value as u16);
                self.program_counter -= 256;
            } else {
                self.program_counter += 1 + value as u16;
            }
        }
    }

    fn clear_carry(&mut self) {
        self.status = self.status & 0b1111_1110;
    }

    fn clear_decimal_mode_flag(&mut self) {
        self.status = self.status & 0b1111_0111;
    }

    fn clear_interrupt_disable_flag(&mut self) {
        self.status = self.status & 0b1111_1011;
    }

    fn clear_neg_flag(&mut self) {
        self.status = self.status & 0b0111_1111;
    }

    fn clear_overflow_flag (&mut self) {
        self.status = self.status & 0b1011_1111;
    }

    fn clear_zero_flag(&mut self) {
        self.status = self.status & 0b1111_1101;
    }

    fn compare_and_set_flags(&mut self, reference: u8, value: u8) {
        if reference >= value {
            self.set_carry();
        } else {
            self.clear_carry();
        }

        if reference == value {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }

        if (reference.wrapping_sub(value) & 0b1000_0000) > 0 {
            self.set_neg_flag();
        } else {
            self.clear_neg_flag();
        }
    }

    fn is_carry_flag_set (&self) -> bool {
        self.status & 0b0000_0001 > 0
    }

    fn is_decimal_mode_flag_set (&self) -> bool {
        self.status & 0b0000_1000 > 0
    }

    fn is_interrupt_disable_flag_set (&self) -> bool {
        self.status & 0b0000_0100 > 0
    }

    fn is_neg_flag_set (&self) -> bool {
        self.status & 0b1000_0000 > 0
    }

    fn is_overflow_flag_set (&self) -> bool {
        self.status & 0b0100_0000 > 0
    }

    fn is_zero_flag_set (&self) -> bool {
        self.status & 0b0000_0010 > 0
    }

    fn set_carry(&mut self) {
        self.status = self.status | 0b0000_0001;
    }

    fn set_decimal_mode_flag (&mut self) {
        self.status = self.status | 0b0000_1000;
    }

    fn set_interrupt_disable_flag (&mut self) {
        self.status = self.status | 0b0000_0100;
    }

    fn set_neg_flag(&mut self) {
        self.status = self.status | 0b1000_0000;
    }

    fn set_overflow_flag(&mut self) {
        self.status = self.status | 0b0100_0000;
    }

    fn set_zero_flag(&mut self) {
        self.status = self.status | 0b0000_0010;
    }

    fn set_neg_and_zero_flag(&mut self, result_value: u8) {
        // this sets the 0 flag in case register_a is 0
        if result_value == 0 {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }

        // this sets the negative flag in case bit 7 is 1
        if result_value & 0b1000_0000 != 0 {
            self.set_neg_flag();
        } else {
            self.clear_neg_flag();
        }
    }

    fn shift_left_and_set_carry(&mut self, value: u8) -> u8 {
        let result = value << 1;
        if value & 0b1000_0000 == 0b1000_0000 {
            self.set_carry();
        } else {
            self.clear_carry();
        }
        result
    }

    fn disable_interrupt (&mut self) {
        self.status = self.status | 0b0000_0100;
    }

    fn enable_interrupt (&mut self) {
        self.status = self.status & 0b1111_1011;
    }

    pub fn get_absolute_address (&self, mode: &AddressingMode, address: u16) -> u16 {
        match mode {
            AddressingMode::Absolute => self.mem_read_u16(address),
            AddressingMode::AbsoluteX => {
                let base_address = self.mem_read_u16(address);
                base_address.wrapping_add(self.register_x as u16)
            },
            AddressingMode::AbsoluteY => {
                let base_address = self.mem_read_u16(address);
                base_address.wrapping_add(self.register_y as u16)
            },
            AddressingMode::Indirect => {
                let indirect_read_address = self.mem_read_u16(address);

                // in indirect mode, which only the jmp instruction uses the 6502 wraps around the lo byte
                // the indirect addressing for the jmp instruction was implemented like this to save
                // costs, transistors were costly at that time
                let read_address_lo = indirect_read_address as u8;
                let read_address_hi = indirect_read_address & 0xFF00;

                let lo = self.mem_read(indirect_read_address);
                let hi = self.mem_read(read_address_hi + (read_address_lo.wrapping_add(1) as u16));

                (lo as u16) + ((hi as u16) << 8)
            },
            AddressingMode::IndirectX => {
                let base_address = self.mem_read(address).wrapping_add(self.register_x);
                // documentation is unclear on how a value of  0xFF would be handled, whether it
                // is a read from  0xFF and  0x0100 or whether it is a wrapped read from  0xFF and  0x00
                let lo = self.mem_read(base_address as u16);
                let hi = self.mem_read(base_address.wrapping_add(1) as u16);
                (lo as u16) + ((hi as u16) << 8)
            },
            AddressingMode::IndirectY => {
                let base_address = self.mem_read(address);
                // documentation is unclear on how a value of  0xFF would be handled, whether it
                // is a read from  0xFF and  0x0100 or whether it is a wrapped read from  0xFF and  0x00
                let lo = self.mem_read(base_address as u16);
                let hi = self.mem_read(base_address.wrapping_add(1) as u16);
                let indirect_address = (lo as u16) + ((hi as u16) << 8);
                indirect_address.wrapping_add(self.register_y as u16)
            },
            AddressingMode::ZeroPage => self.mem_read(address) as u16,
            AddressingMode::ZeroPageX => {
                let base_address = self.mem_read(address);
                base_address.wrapping_add(self.register_x) as u16
            },
            AddressingMode::ZeroPageY => {
                let base_address = self.mem_read(address);
                base_address.wrapping_add(self.register_y) as u16
            },
            AddressingMode::NoneAddressing | AddressingMode::Immediate  => {
                panic!("mode {:?} is not supported", mode);
            },
        }
    }

    fn get_operand_address (&self, mode: &AddressingMode) -> u16 {
        match mode {
            // use the value right after the opcode
            AddressingMode::Immediate | AddressingMode::NoneAddressing => self.program_counter,

            // for all other cases parsing is more complicated
            _ => self.get_absolute_address(mode, self.program_counter)
        }
    }

}