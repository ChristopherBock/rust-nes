/*
Useful documentation:
https://bugzmanov.github.io/nes_ebook/chapter_3_4.html
https://skilldrick.github.io/easy6502/#addressing
https://www.nesdev.org/obelisk-6502-guide/addressing.html
https://www.nesdev.org/obelisk-6502-guide/registers.html
 */

use crate::opcodes;

pub struct CPU {
    pub register_a: u8,
    // pushes to the stack decrement the stack pointer
    // pulling from it increments it
    pub register_s: u8, // the stack is located between $0100 and $01FF
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}

#[derive(Debug)]
#[derive(PartialEq)]
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

/* Additional information
The NES CPU uses little endian addressing: least significant bits first
 -> real adress 0x8000 is stored as 0x00 0x80
 */
impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_s: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.load_and_run(program)
    }

    pub fn load_and_run (&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load (&mut self, program: Vec<u8>) {
        let program_base_adress = 0x0600 as u16;
        self.memory[program_base_adress as usize .. (program_base_adress as usize + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, program_base_adress)
    }

    pub fn reset (&mut self) {
        self.register_a = 0;
        self.register_s = 0xFF;
        self.register_x = 0;
        self.register_y = 0;

        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC)
    }

    pub fn run (&mut self) {
        let ref opcodes = *opcodes::OPCODES_MAP;

        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let program_counter_state = self.program_counter;

            let opcode = opcodes.get(&code).expect(
                &format!("OpCode {:x} is not recognized!", code)
            );

            println!("OpCode {}", opcode.name);

            match code {
                0x06 | 0x0A | 0x0E | 0x16 | 0x1E => {
                    self.asl(&opcode.mode);
                },
                0xE8 => self.inx(),
                0x4C | 0x6C => {
                    self.jmp(&opcode.mode);
                },
                0x20 => {
                    self.jsr(&opcode.mode);
                }
                0xA1 |0xA5 | 0xA9 | 0xAD | 0xB1 | 0xB5 | 0xB9 | 0xBD => {
                    self.lda(&opcode.mode);
                },
                0xA2 | 0xA6 | 0xAE | 0xB6 | 0xBE  => {
                    self.ldx(&opcode.mode);
                },
                0xA0 | 0xA4 | 0xAC | 0xB4 | 0xBC  => {
                    self.ldy(&opcode.mode);
                },
                0x26 | 0x2A | 0x2E | 0x36 | 0x3E => {
                    self.rol(&opcode.mode);
                },
                0x60 => {
                    self.rts();
                },
                0x38 => {
                    self.sec();
                },
                0x85 | 0x8D | 0x95 | 0x99 | 0x9D => {
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
        Memory access
     */
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        /*let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;

        (hi << 8) | (lo as u16)*/
        u16::from_le_bytes([self.mem_read(addr), self.mem_read(addr + 1)])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        /*let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;

        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);*/

        let bytes = data.to_le_bytes();

        self.mem_write(addr, bytes[0]);
        self.mem_write(addr + 1, bytes[1]);
    }

    /*
    Stack Operations
     */
    fn push_stack_u16(&mut self, data: u16) {
        self.mem_write_u16((self.register_s - 1) as u16 + 0x0100, data);
        self.register_s -= 2;
    }

    fn pop_stack_u16(&mut self) -> u16 {
        let value = self.mem_read_u16((self.register_s + 1) as u16 + 0x0100);
        self.register_s += 2;

        value
    }

    /*
        OP Codes
     */
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

    fn inx (&mut self) {
        if self.register_x == 0xff {
            self.register_x = 0
        } else {
            self.register_x += 1;
        }

        self.set_neg_and_zero_flag(self.register_x);
    }

    // jumps
    fn jmp (&mut self, mode: &AddressingMode) {
        let target_addr = self.get_operand_address(mode);
        self.program_counter = target_addr;
    }

    fn jsr (&mut self, mode: &AddressingMode) {
        let return_address = self.program_counter + 2;
        self.push_stack_u16(return_address);

        self.jmp(mode)
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

    fn rol (&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::NoneAddressing {
            let new_carry = (self.register_a & 0b1000_0000) >> 7;
            let result = (self.register_a << 1) + (self.status & 0b0000_0001);

            self.status = (self.status & 0b111_1110) + new_carry;

            self.register_a = result;
            self.set_neg_and_zero_flag(result);
        } else {
            let address = self.get_operand_address(mode);
            let value = self.mem_read(address);

            let new_carry = (value & 0b1000_0000) >> 7;
            let result = (value << 1) + (self.status & 0b0000_0001);

            self.status = (self.status & 0b111_1110) + new_carry;

            self.mem_write(address, result);
            self.set_neg_and_zero_flag(result);
        }
    }

    // jump and interrupt returns

    fn rts (&mut self) {
        let return_address = self.pop_stack_u16();
        self.program_counter = return_address + 1;
    }

    fn sec (&mut self) {
        self.set_carry();
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

        self.set_neg_and_zero_flag(self.register_s);
    }

    fn tya (&mut self) {
        self.register_a = self.register_x;

        self.set_neg_and_zero_flag(self.register_x);
    }

    /*
        Helper functions
     */
    fn set_neg_and_zero_flag(&mut self, result_value: u8) {
        // this sets the 0 flag in case register_a is 0
        if result_value == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        // this sets the negative flag in case bit 7 is 1
        if result_value & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    fn set_carry(&mut self) {
        self.status = self.status | 0b0000_0001;
    }

    fn shift_left_and_set_carry(&mut self, value: u8) -> u8 {
        let result = value << 1;
        if value & 0b1000_0000 == 0b1000_0000 {
            self.set_carry();
        }
        result
    }

    fn disable_interrupt (&mut self) {
        self.status = self.status | 0b0000_0100;
    }

    fn enable_interrupt (&mut self) {
        self.status = self.status & 0b1111_1011;
    }

    fn get_operand_address (&self, mode: &AddressingMode) -> u16 {
        match mode {

            // use the value right after the opcode
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::AbsoluteX => {
                let base_address = self.mem_read_u16(self.program_counter);
                base_address.wrapping_add(self.register_x as u16)
            },
            AddressingMode::AbsoluteY => {
                let base_address = self.mem_read_u16(self.program_counter);
                base_address.wrapping_add(self.register_y as u16)
            },

            AddressingMode::Indirect => self.mem_read_u16(self.mem_read_u16(self.program_counter)),
            AddressingMode::IndirectX => {
                let address = self.mem_read(self.program_counter);
                // documentation is unclear on how a value of $FF would be handled, whether it
                // is a read from $FF and $0100 or whether it is a wrapped read from $FF and $00
                self.mem_read_u16(address.wrapping_add(self.register_x) as u16)
            },
            AddressingMode::IndirectY => {
                let address = self.mem_read(self.program_counter) as u16;
                // documentation is unclear on how a value of $FF would be handled, whether it
                // is a read from $FF and $0100 or whether it is a wrapped read from $FF and $00
                self.mem_read_u16(address) + self.register_y as u16
            },

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPageX => {
                let address = self.mem_read(self.program_counter);
                address.wrapping_add(self.register_x) as u16
            },
            AddressingMode::ZeroPageY => {
                let address = self.mem_read(self.program_counter);
                address.wrapping_add(self.register_y) as u16
            },

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            },
        }
    }

}