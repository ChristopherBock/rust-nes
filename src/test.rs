#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::vec;

    use rand::Rng;

    use crate::bus::Bus;
    use crate::cpu::CPU;
    use crate::cpu::AddressingMode;
    use crate::mem::Mem;
    use crate::cartridge::create_test_cartridge;

    fn create_new_cpu() -> CPU {
        let cartridge = create_test_cartridge(false);
        let bus = Bus::new(cartridge);

        CPU::new(bus)
    }

    macro_rules! hashmap {
        ($( $key:expr => $value:expr ), * $(,)?) => {
            {
                let mut map = HashMap::new();
                $(
                    map.insert($key, $value);
                )*
                map
            }
        };
    }

    fn check_zero_and_neg_flags(cpu: &CPU, expected_zero_flag: bool, expected_neg_flag: bool) {
        if expected_zero_flag {
            assert!(cpu.status & 0b0000_0010 == 0b10);
        } else {
            assert!(cpu.status & 0b0000_0010 == 0b00);
        }

        if expected_neg_flag {
            assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
        } else {
            assert!(cpu.status & 0b1000_0000 == 0b00);
        }
    }

    fn check_carry_flag(cpu: &CPU, expected_carry_flag: bool) {
        if expected_carry_flag {
            assert!(cpu.status & 0b0000_0001 == 0b01);
        } else {
            assert!(cpu.status & 0b0000_0001 == 0b00);
        }
    }

    fn check_decimal_flag(cpu: &CPU, expected_decimal_flag: bool) {
        if expected_decimal_flag {
            assert!(cpu.status & 0b0000_1000 == 0b1000);
        } else {
            assert!(cpu.status & 0b0000_1000 == 0b00);
        }
    }

    fn check_interrupt_disable_flag(cpu: &CPU, expected_interrupt_disable_flag: bool) {
        if expected_interrupt_disable_flag {
            assert!(cpu.status & 0b0000_0100 == 0b100);
        } else {
            assert!(cpu.status & 0b0000_0100 == 0b00);
        }
    }

    fn check_overflow_flag(cpu: &CPU, expected_overflow_flag: bool) {
        if expected_overflow_flag {
            assert!(cpu.status & 0b0100_0000 == 0b0100_0000);
        } else {
            assert!(cpu.status & 0b0100_0000 == 0b00);
        }
    }

    #[test]
    fn snake() {
        let game_code = vec![
    0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02, 0x85,
    0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9, 0x0f, 0x85,
    0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe,
    0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3,
    0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9,
    0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60,
    0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
    0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02,
    0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06,
    0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07,
    0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
    0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
    0x35, 0x07, 0x60, 0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02,
    0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
    0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
    0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xb0,
    0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29,
    0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60,
    0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea,
    0xea, 0xca, 0xd0, 0xfb, 0x60
];

        let mut cpu = create_new_cpu();
        cpu.interpret(game_code);
    }

    #[test]
    fn test_sta_and_lda_from_memory() {
        let mut cpu = create_new_cpu();
        cpu.interpret(vec![0xa9, 0x07, 0x8D, 0x00, 0x00, 0xa9, 0x02, 0xAE, 0x00, 0x00, 0x00]);

        assert_eq!(cpu.register_a, 0x02);
        assert_eq!(cpu.register_x, 0x07);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = create_new_cpu();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
        
        assert_eq!(cpu.register_x, 1);
    }

    fn build_instructions(opcode: u8, addressing_mode: &AddressingMode, value: u16, xydeviation: u8) -> Vec<u8> {
        let mut result = vec![opcode];
        let mut rng = rand::thread_rng();

        let number_of_random_bytes = rng.gen_range(5, 14) as u8;
        let random_numbers: Vec<u8> = (0..number_of_random_bytes)
            .map(|_| rng.gen_range(1 , 77))
            .collect();

        match addressing_mode {
            AddressingMode::NoneAddressing => {
                // do nothing
            },
            AddressingMode::Immediate => {
                result.push(value as u8);
            },
            AddressingMode::Absolute => {
                let address = 1 + 2 + 1 + number_of_random_bytes; // len(opcode) + len(address) + len(BRK opcode)
                result.extend_from_slice(&(address as u16).to_le_bytes());
            },
            AddressingMode::AbsoluteX => {
                let address = 1 + 2 + 1 + number_of_random_bytes; // len(opcode) + len(address) + len(BRK opcode)
                result.extend_from_slice(&(address as u16).to_le_bytes());
                result.extend_from_slice(&vec![0x00; xydeviation as usize]);
            },
            AddressingMode::AbsoluteY => {
                let address = 1 + 2 + 1 + number_of_random_bytes; // len(opcode) + len(address) + len(BRK opcode)
                result.extend_from_slice(&(address as u16).to_le_bytes());
                result.extend_from_slice(&vec![0x00; xydeviation as usize]);
            },
            AddressingMode::ZeroPage => {
                let address = 1 + 1 + 1 + number_of_random_bytes; // len(opcode) + len(address) + len(BRK opcode)
                result.extend_from_slice(&address.to_le_bytes());
            },
            AddressingMode::ZeroPageX => {
                let address = 1 + 1 + 1 + number_of_random_bytes; // len(opcode) + len(address) + len(BRK opcode)
                result.extend_from_slice(&address.to_le_bytes());
                result.extend_from_slice(&vec![0x00; xydeviation as usize]);
            },
            AddressingMode::ZeroPageY => {
                let address = 1 + 1 + 1 + number_of_random_bytes; // len(opcode) + len(address) + len(BRK opcode)
                result.extend_from_slice(&address.to_le_bytes());
                result.extend_from_slice(&vec![0x00; xydeviation as usize]);
            },
            AddressingMode::Indirect => {
                let address_of_address = 1 + 1 + 1; // len(opcode) + len(address_of_address) + len(BRK opcode) + len(obscurity)
                let address = 1 + 1 + 1 + 2 + 1 + number_of_random_bytes;
                // len(opcode) + len(address_of_address) + len(BRK opcode) + len(obscurity) + len(address) + len(superfluous BRK) + len(random bytes)
                result.extend_from_slice(&(address_of_address as u8).to_le_bytes()); // this is the address of the address...
                result.push(0x00);
                result.extend_from_slice(&(address as u16).to_le_bytes());
            },
            AddressingMode::IndirectX => {
                let address_of_address = 1 + 1 + 1; // len(opcode) + len(address_of_address) + len(BRK opcode) + len(obscurity)
                let address = 1 + 1 + 1 + xydeviation + 2 + 1 + number_of_random_bytes;
                // len(opcode) + len(address_of_address) + len(BRK opcode) + len(obscurity) + len(address) + len(superfluous BRK) + len(random bytes)
                result.extend_from_slice(&(address_of_address as u8).to_le_bytes()); // this is the address of the address...
                result.push(0x00);
                result.extend_from_slice(&vec![0x00; xydeviation as usize]);
                result.extend_from_slice(&(address as u16).to_le_bytes());
            },
            AddressingMode::IndirectY => {
                let address_of_address = 1 + 1 + 1; // len(opcode) + len(address_of_address) + len(BRK opcode) + len(obscurity)
                let address = 1 + 1 + 1 + 2 + 1 + number_of_random_bytes;
                // len(opcode) + len(address_of_address) + len(BRK opcode) + len(obscurity) + len(address) + len(superfluous BRK) + len(random bytes)
                result.extend_from_slice(&(address_of_address as u8).to_le_bytes()); // this is the address of the address...
                result.push(0x00);
                result.extend_from_slice(&(address as u16).to_le_bytes());
                result.extend_from_slice(&vec![0x00; xydeviation as usize]);
            },
        }

        result.push(0x00);

        result.extend_from_slice(&random_numbers);

        result.extend_from_slice(&value.to_le_bytes());

        result.extend_from_slice(&random_numbers);

        result
    }

    macro_rules! opcode_test_case {
        ( $s:expr, $( $opcode:expr, $addressing_mode:expr, $value:expr, $setup:expr, $assert_stmt:stmt, $xydeviation:expr), * $(,)?) => {
            $(
                $s.reset();
                $setup;

                let instructions = build_instructions($opcode, $addressing_mode, $value, $xydeviation);

                $s.interpret_without_reset(instructions, 0x00);

                $assert_stmt
            )*
        };
    }

    // todo: this is shakey, since the random numbers might have the very same bit patterns
    #[test]
    fn test_and () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            0x29, &AddressingMode::Immediate, 0b0101_0101, cpu.register_a = 0b0101, assert_eq!(cpu.register_a, 0b0101), 0,
            0x29, &AddressingMode::Immediate, 0b1101_0101, cpu.register_a = 0b0000_0101, {assert_eq!(cpu.register_a, 0b0000_0101); check_zero_and_neg_flags(&cpu, false, false);}, 0,
            0x29, &AddressingMode::Immediate, 0b1101_0101, cpu.register_a = 0b1000_0101, {assert_eq!(cpu.register_a, 0b1000_0101); check_zero_and_neg_flags(&cpu, false, true);}, 0,
            0x29, &AddressingMode::Immediate, 0b1010_0101, cpu.register_a = 0b0101_1010, {assert_eq!(cpu.register_a, 0b0000_0000); check_zero_and_neg_flags(&cpu, true, false);}, 0,
            0x2D, &AddressingMode::Absolute, 0b0101_0111, cpu.register_a = 0b0011, assert_eq!(cpu.register_a, 0b0011), 0,
            0x3D, &AddressingMode::AbsoluteX, 0b0101_0111, {cpu.register_a = 0b0011; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0b0011), xydeviation,
            0x39, &AddressingMode::AbsoluteY, 0b0101_0111, {cpu.register_a = 0b0011; cpu.register_y = xydeviation;}, assert_eq!(cpu.register_a, 0b0011), xydeviation,
            0x25, &AddressingMode::ZeroPage, 0b0101_0111, cpu.register_a = 0b0101_0000, assert_eq!(cpu.register_a, 0b0101_0000), 0,
            0x35, &AddressingMode::ZeroPageX, 0b0101_0111, {cpu.register_a = 0b0011_0011; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0b001_0011), xydeviation,
            0x21, &AddressingMode::IndirectX, 0b0101_0111, {cpu.register_a = 0b0101_0000; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0b0101_0000), xydeviation,
            0x31, &AddressingMode::IndirectY, 0b0101_0111, {cpu.register_a = 0b0011_0011; cpu.register_y = xydeviation;}, assert_eq!(cpu.register_a, 0b001_0011), xydeviation,
        }
    }

    #[test]
    fn test_adc() {
        let mut cpu = create_new_cpu();

        macro_rules! adc_test_cases {
            ( $( $base:expr, $instructions:expr, $result:expr, $additional_setup:block), * $(,)?) => {
                $(
                    cpu.reset();
                    cpu.register_a = $base;
                    $additional_setup

                    cpu.interpret_without_reset($instructions, 0x00);

                    assert_eq!(cpu.register_a, $result);
                )*
            };
        }

        adc_test_cases!{
            // immediate addressing
            0x22, vec![0x69, 0x33, 0x00], 0x55, {},
            // absolute addressing
            0x25, vec![0x6D, 0x07, 0x00, 0x00, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A], 0x2C, {},
            // absolute x addressing
            0x33, vec![0x7D, 0x07, 0x00, 0x00, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A], 0x3C, {cpu.register_x = 0x02},
            // absolute y addressing
            0x33, vec![0x79, 0x07, 0x00, 0x00, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x10, 0x20], 0x43, {cpu.register_y = 0x04},
            // zero page addressing
            0x23, vec![0x65, 0x07, 0x00, 0x00, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A], 0x2A, {},
            // zero page x addressing
            0x23, vec![0x75, 0x07, 0x00, 0x00, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A], 0x2C, {cpu.register_x = 0x02},
            // zero page x wrap addressing
            0x23, vec![0x75, 0x07, 0x00, 0x00, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A], 0x28, {cpu.register_x = 0xFE},
            // indirect x addressing: takes operand + register_x and reads address there
            0x23, vec![0x61, 0x04, 0x00, 0x03, 0x08, 0x00, 0x06, 0x0B, 0x00, 0x09, 0x0A, 0x10, 0x20], 0x33, { cpu.register_x = 3 },
            // indirect y addressing: takes operand, reads address there + register_y
            0x23, vec![0x71, 0x04, 0x00, 0x03, 0x08, 0x00, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x10, 0x20], 0x2D, { cpu.register_y = 2 },
        }
    }

    #[test]
    fn test_asl () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            0x0A, &AddressingMode::NoneAddressing, 0b0000_0101, cpu.register_a = 0b0000_0101, {
                assert_eq!(cpu.register_a, 0b0000_1010);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
            0x06, &AddressingMode::ZeroPage,    0b0101_0111, cpu.register_a = 0b0101_0111, {
                assert_eq!(cpu.last_mem_write_value, 0b1010_1110);
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
            }, 0,
            0x16, &AddressingMode::ZeroPageX,   0b0000_0100, {cpu.register_a = 0b0000_0100; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0b0000_1000);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, xydeviation,
            0x0E, &AddressingMode::Absolute,    0b1101_1110, cpu.register_a = 0b1101_1110, {
                assert_eq!(cpu.last_mem_write_value, 0b1011_1100);
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, true);
            }, 0,
            0x1E, &AddressingMode::AbsoluteX,   0b0000_0000, {cpu.register_a = 0b0000_0000; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0b0000_0000);
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, false);
            }, xydeviation,
        }
    }

    #[test]
    fn test_bcs () {
        let mut cpu = create_new_cpu();
        cpu.reset();
        cpu.interpret_without_reset(vec![0xB0, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x03);
        assert_eq!(cpu.register_x, 0x00);

        cpu.reset();
        cpu.status = 0b0000_0001;
        cpu.interpret_without_reset(vec![0xB0, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x05);
        assert_eq!(cpu.register_x, 0x01);
    }

    #[test]
    fn test_bcc () {
        let mut cpu = create_new_cpu();
        cpu.reset();
        cpu.interpret_without_reset(vec![0x90, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x05);
        assert_eq!(cpu.register_x, 0x01);

        cpu.reset();
        cpu.status = 0b0000_0001;
        cpu.interpret_without_reset(vec![0x90, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x03);
        assert_eq!(cpu.register_x, 0x00);
    }

    #[test]
    fn test_beq () {
        let mut cpu = create_new_cpu();
        cpu.reset();
        cpu.interpret_without_reset(vec![0xF0, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x03);
        assert_eq!(cpu.register_x, 0x00);

        cpu.reset();
        cpu.status = 0b0000_0010;
        cpu.interpret_without_reset(vec![0xF0, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x05);
        assert_eq!(cpu.register_x, 0x01);

        cpu.reset();
        cpu.status = 0b0000_0010;
        // inx resets the zero flag, therefore the second branch needs to be an BNE
        cpu.interpret_without_reset(vec![0xF0, 0x01, 0x00, 0xE8, 0xD0, 0xFC, 0xE8, 0xE8, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x03);
        assert_eq!(cpu.register_x, 0x01);
    }

    #[test]
    fn test_bit () {
        let mut cpu = create_new_cpu();
        opcode_test_case!{
            cpu,
            0x24, &AddressingMode::ZeroPage, 0b0001_0010, {cpu.register_a = 0b0001_0010;}, {
                check_zero_and_neg_flags(&cpu, false, false);
            }, 0,
            0x24, &AddressingMode::ZeroPage, 0b0001_0010, {cpu.register_a = 0b0000_0100;}, {
                check_zero_and_neg_flags(&cpu, true, false);
            }, 0,
            0x2C, &AddressingMode::Absolute, 0b0001_0010, {cpu.register_a = 0b0000_1100;}, {
                check_zero_and_neg_flags(&cpu, true, false);
            }, 0,
            0x2C, &AddressingMode::Absolute, 0b1000_0000, {cpu.register_x = 0b0000_1100;}, {
                check_zero_and_neg_flags(&cpu, true, true);
            }, 0,
        }
    }

    #[test]
    fn test_bmi () {
        let mut cpu = create_new_cpu();
        cpu.reset();
        cpu.status = 0b1000_0000;
        cpu.interpret_without_reset(vec![0x30, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x05);
        assert_eq!(cpu.register_x, 0x01);

        cpu.reset();
        cpu.interpret_without_reset(vec![0x30, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x03);
        assert_eq!(cpu.register_x, 0x00);
    }

    #[test]
    fn test_bne () {
        let mut cpu = create_new_cpu();
        cpu.reset();
        cpu.interpret_without_reset(vec![0xD0, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x05);
        assert_eq!(cpu.register_x, 0x01);

        cpu.reset();
        cpu.status = 0b0000_0010;
        cpu.interpret_without_reset(vec![0xD0, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x03);
        assert_eq!(cpu.register_x, 0x00);

        cpu.reset();
        cpu.interpret_without_reset(vec![0xD0, 0x01, 0x00, 0xE8, 0xD0, 0xFC, 0xE8, 0xE8, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x03);
        assert_eq!(cpu.register_x, 0x01);
    }

    #[test]
    fn test_bpl () {
        let mut cpu = create_new_cpu();
        cpu.reset();
        cpu.interpret_without_reset(vec![0x10, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x05);
        assert_eq!(cpu.register_x, 0x01);

        cpu.reset();
        cpu.status = 0b1000_0000;
        cpu.interpret_without_reset(vec![0x10, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x03);
        assert_eq!(cpu.register_x, 0x00);
    }

    #[test]
    fn test_bvc () {
        let mut cpu = create_new_cpu();
        cpu.reset();
        cpu.interpret_without_reset(vec![0x50, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x05);
        assert_eq!(cpu.register_x, 0x01);

        cpu.reset();
        cpu.status = 0b0100_0000;
        cpu.interpret_without_reset(vec![0x50, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x03);
        assert_eq!(cpu.register_x, 0x00);
    }

    #[test]
    fn test_bvs () {
        let mut cpu = create_new_cpu();
        cpu.reset();
        cpu.status = 0b0100_0000;
        cpu.interpret_without_reset(vec![0x70, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x05);
        assert_eq!(cpu.register_x, 0x01);

        cpu.reset();
        cpu.interpret_without_reset(vec![0x70, 0x01, 0x00, 0xE8, 0x00], 0x00);

        assert_eq!(cpu.program_counter, 0x03);
        assert_eq!(cpu.register_x, 0x00);
    }

    #[test]
    fn test_clc_and_sec () {
        let mut cpu = create_new_cpu();
        opcode_test_case!{
            cpu,
            0x18, &AddressingMode::NoneAddressing, 0b0000_0001, {cpu.status = 0b0000_0001; check_carry_flag(&cpu, true);}, check_carry_flag(&cpu, false), 0,
            0x38, &AddressingMode::NoneAddressing, 0b0000_0000, {cpu.status = 0b0000_0000; check_carry_flag(&cpu, false);}, check_carry_flag(&cpu, true), 0,
        }
    }

    #[test]
    fn test_cld_and_sed () {
        let mut cpu = create_new_cpu();
        opcode_test_case!{
            cpu,
            0xD8, &AddressingMode::NoneAddressing, 0b0000_1000, {cpu.status = 0b0000_1000; check_decimal_flag(&cpu, true);}, check_decimal_flag(&cpu, false), 0,
            0xF8, &AddressingMode::NoneAddressing, 0b0000_0000, {cpu.status = 0b0000_0000; check_decimal_flag(&cpu, false);}, check_decimal_flag(&cpu, true), 0,
        }
    }

    #[test]
    fn test_cli_and_sei () {
        let mut cpu = create_new_cpu();
        opcode_test_case!{
            cpu,
            0x58, &AddressingMode::NoneAddressing, 0b0000_0100, {cpu.status = 0b0000_0100; check_interrupt_disable_flag(&cpu, true);}, check_interrupt_disable_flag(&cpu, false), 0,
            0x78, &AddressingMode::NoneAddressing, 0b0000_0000, {cpu.status = 0b0000_0000; check_interrupt_disable_flag(&cpu, false);}, check_interrupt_disable_flag(&cpu, true), 0,
        }
    }

    #[test]
    fn test_clv () {
        let mut cpu = create_new_cpu();
        opcode_test_case!{
            cpu,
            0xB8, &AddressingMode::NoneAddressing, 0b0100_0000, {cpu.status = 0b0100_0000; check_overflow_flag(&cpu, true);}, check_overflow_flag(&cpu, false), 0,
        }
    }

    #[test]
    fn test_cmp () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            // without carry set
            0xC9, &AddressingMode::Immediate, 0x22, cpu.register_a = 0x22, {
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0xC5, &AddressingMode::ZeroPage, 0x22, cpu.register_a = 0x23, {
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0xD5, &AddressingMode::ZeroPageX, 0x32, {cpu.register_a = 0x24; cpu.register_x = xydeviation;}, {
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
            }, xydeviation,
            0xCD, &AddressingMode::Absolute, 0x42, cpu.register_a = 0x25, {
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
            }, 0,
            0xDD, &AddressingMode::AbsoluteX, 0x21, {cpu.register_a = 0x26; cpu.register_x = xydeviation;}, {
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, xydeviation,
            0xD9, &AddressingMode::AbsoluteY, 0x20, {cpu.register_a = 0x27; cpu.register_y = xydeviation;}, {
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, xydeviation,
            0xC1, &AddressingMode::IndirectX, 0x28, {cpu.register_a = 0x28; cpu.register_x = xydeviation;}, {
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, true);
            }, xydeviation,
            0xD1, &AddressingMode::IndirectY, 0x22, {cpu.register_a = 0x29; cpu.register_y = xydeviation;}, {
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, xydeviation,
        }
    }

    #[test]
    fn test_cpu_ram_mirroring() {
        let mut  cpu = create_new_cpu();

        cpu.interpret(vec![0xEA, 0xEA, 0xEA, 0xEA, 0x00]);
        assert_eq!(cpu.program_counter, 0x0605);
        assert_eq!(cpu.register_s, 0xFD);
        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.register_y, 0x00);

        let first_page = cpu.mem_read(0x0601);
        let second_page = cpu.mem_read(0x0E01);
        let third_page = cpu.mem_read(0x1601);
        let fourth_page = cpu.mem_read(0x1E01);

        assert_eq!(first_page, 0xEA);
        assert_eq!(first_page, second_page);
        assert_eq!(first_page, third_page);
        assert_eq!(first_page, fourth_page);
    }

    #[test]
    fn test_cpx_cpy () {
        let mut cpu = create_new_cpu();
        opcode_test_case!{
            cpu,
            // without carry set
            0xE0, &AddressingMode::Immediate, 0x22, cpu.register_x = 0x22, {
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0xE0, &AddressingMode::Immediate, 0x23, cpu.register_x = 0x22, {
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
            }, 0,
            0xE0, &AddressingMode::Immediate, 0x21, cpu.register_x = 0x22, {
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0xE4, &AddressingMode::ZeroPage, 0x22, cpu.register_x = 0x23, {
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0xEC, &AddressingMode::Absolute, 0x42, cpu.register_x = 0x25, {
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
            }, 0,
            0xC0, &AddressingMode::Immediate, 0x22, cpu.register_y = 0x22, {
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0xC0, &AddressingMode::Immediate, 0x23, cpu.register_y = 0x22, {
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
            }, 0,
            0xC0, &AddressingMode::Immediate, 0x21, cpu.register_y = 0x22, {
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0xC4, &AddressingMode::ZeroPage, 0x22, cpu.register_y = 0x23, {
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0xCC, &AddressingMode::Absolute, 0x42, cpu.register_y = 0x25, {
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
            }, 0,
        }
    }

    #[test]
    fn test_dec () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            0xC6, &AddressingMode::ZeroPage, 0x23, {}, assert_eq!(cpu.last_mem_write_value, 0x22), 0,
            0xD6, &AddressingMode::ZeroPageX, 0x33, {cpu.register_x = xydeviation;}, assert_eq!(cpu.last_mem_write_value, 0x32), xydeviation,
            0xCE, &AddressingMode::Absolute, 0x43, {}, assert_eq!(cpu.last_mem_write_value, 0x42), 0,
            0xDE, &AddressingMode::AbsoluteX, 0x44, {cpu.register_x = xydeviation;}, assert_eq!(cpu.last_mem_write_value, 0x43), xydeviation,
            // now test the flag setting
            0xCE, &AddressingMode::Absolute, 0x01, {}, {
                assert_eq!(cpu.last_mem_write_value, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);
            }, 0,
            0xCE, &AddressingMode::Absolute, 0x00, {}, {
                assert_eq!(cpu.last_mem_write_value, 0xFF);
                check_zero_and_neg_flags(&cpu, false, true);
            }, 0,
            0xCE, &AddressingMode::Absolute, 0x80, {}, {
                assert_eq!(cpu.last_mem_write_value, 0x7F);
                check_zero_and_neg_flags(&cpu, false, false);
            }, 0,
            0xCE, &AddressingMode::Absolute, 0x81, {}, {
                assert_eq!(cpu.last_mem_write_value, 0x80);
                check_zero_and_neg_flags(&cpu, false, true);
            }, 0,
        }
    }

    #[test]
    fn test_dex_and_dey () {
        let mut cpu = create_new_cpu();
        opcode_test_case!{
            cpu,
            0xCA, &AddressingMode::NoneAddressing, 0x23, {cpu.register_x = 0x23;}, assert_eq!(cpu.register_x, 0x22), 0,
            0xCA, &AddressingMode::NoneAddressing, 0x01, {cpu.register_x = 0x01;}, {
                assert_eq!(cpu.register_x, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);
            }, 0,
            0xCA, &AddressingMode::NoneAddressing, 0x00, {cpu.register_x = 0x00;}, {
                assert_eq!(cpu.register_x, 0xFF);
                check_zero_and_neg_flags(&cpu, false, true);
            }, 0,
            0x88, &AddressingMode::NoneAddressing, 0x43, {cpu.register_y = 0x43;}, assert_eq!(cpu.register_y, 0x42), 0,
            0x88, &AddressingMode::NoneAddressing, 0x01, {cpu.register_y = 0x01;}, {
                assert_eq!(cpu.register_y, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);
            }, 0,
            0x88, &AddressingMode::NoneAddressing, 0x00, {cpu.register_y = 0x00;}, {
                assert_eq!(cpu.register_y, 0xFF);
                check_zero_and_neg_flags(&cpu, false, true);
            }, 0,
        }
    }

    #[test]
    fn test_eor () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            0x49, &AddressingMode::Immediate,   0b0101_0101, cpu.register_a = 0b0000_0101, assert_eq!(cpu.register_a, 0b0101_0000), 0,
            0x49, &AddressingMode::Immediate,   0b0111_0101, cpu.register_a = 0b0000_0001, {assert_eq!(cpu.register_a, 0b0111_0100); check_zero_and_neg_flags(&cpu, false, false);}, 0,
            0x49, &AddressingMode::Immediate,   0b1000_0010, cpu.register_a = 0b0000_0001, {assert_eq!(cpu.register_a, 0b1000_0011); check_zero_and_neg_flags(&cpu, false, true);}, 0,
            0x49, &AddressingMode::Immediate,   0b0000_0000, cpu.register_a = 0b0000_0000, {assert_eq!(cpu.register_a, 0b0000_0000); check_zero_and_neg_flags(&cpu, true, false);}, 0,
            0x45, &AddressingMode::ZeroPage,    0b0101_0111, cpu.register_a = 0b0101_0000, assert_eq!(cpu.register_a, 0b0000_0111), 0,
            0x55, &AddressingMode::ZeroPageX,   0b0101_0111, {cpu.register_a = 0b0011_0011; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0b0110_0100), xydeviation,
            0x41, &AddressingMode::IndirectX,   0b0101_0111, {cpu.register_a = 0b0101_0000; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0b0000_0111), xydeviation,
            0x51, &AddressingMode::IndirectY,   0b0101_0110, {cpu.register_a = 0b0010_0010; cpu.register_y = xydeviation;}, assert_eq!(cpu.register_a, 0b0111_0100), xydeviation,
            0x4D, &AddressingMode::Absolute,    0b0101_1110, cpu.register_a = 0b0000_0011, assert_eq!(cpu.register_a, 0b0101_1101), 0,
            0x5D, &AddressingMode::AbsoluteX,   0b0010_0111, {cpu.register_a = 0b0000_0011; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0b0010_0100), xydeviation,
            0x59, &AddressingMode::AbsoluteY,   0b0001_0101, {cpu.register_a = 0b0000_1000; cpu.register_y = xydeviation;}, assert_eq!(cpu.register_a, 0b0001_1101), xydeviation,
        }
    }

    #[test]
    fn test_inx () {
        let mut cpu = create_new_cpu();
        opcode_test_case!{
            cpu,
            0xE8, &AddressingMode::NoneAddressing, 0x23, {cpu.register_x = 0x23;}, assert_eq!(cpu.register_x, 0x24), 0,
            0xE8, &AddressingMode::NoneAddressing, 0xFF, {cpu.register_x = 0xFF;}, {
                assert_eq!(cpu.register_x, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);
            }, 0,
            0xE8, &AddressingMode::NoneAddressing, 0x7F, {cpu.register_x = 0x7F;}, {
                assert_eq!(cpu.register_x, 0x80);
                check_zero_and_neg_flags(&cpu, false, true);
            }, 0,
            0xE8, &AddressingMode::NoneAddressing, 0x94, {cpu.register_x = 0x94;}, {
                assert_eq!(cpu.register_x, 0x95);
                check_zero_and_neg_flags(&cpu, false, true);
            }, 0,
        }
    }

    #[test]
    fn test_iny () {
        let mut cpu = create_new_cpu();
        opcode_test_case!{
            cpu,
            0xC8, &AddressingMode::NoneAddressing, 0x23, {cpu.register_y = 0x23;}, assert_eq!(cpu.register_y, 0x24), 0,
            0xC8, &AddressingMode::NoneAddressing, 0xFF, {cpu.register_y = 0xFF;}, {
                assert_eq!(cpu.register_y, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);
            }, 0,
            0xC8, &AddressingMode::NoneAddressing, 0x7F, {cpu.register_y = 0x7F;}, {
                assert_eq!(cpu.register_y, 0x80);
                check_zero_and_neg_flags(&cpu, false, true);
            }, 0,
            0xC8, &AddressingMode::NoneAddressing, 0x94, {cpu.register_y = 0x94;}, {
                assert_eq!(cpu.register_y, 0x95);
                check_zero_and_neg_flags(&cpu, false, true);
            }, 0,
        }
    }

    #[test]
    fn test_inc () {
        let mut cpu = create_new_cpu();
        let xydeviation = 9 as u8;
        opcode_test_case!{
            cpu,
            0xE6, &AddressingMode::ZeroPage, 0x23, {}, {
                assert_eq!(cpu.last_mem_write_value, 0x24);
                check_zero_and_neg_flags(&cpu, false, false);
            }, 0,
            0xF6, &AddressingMode::ZeroPageX, 0xFF, {cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);
            }, xydeviation,
            0xEE, &AddressingMode::Absolute, 0x7F, {}, {
                assert_eq!(cpu.last_mem_write_value, 0x80);
                check_zero_and_neg_flags(&cpu, false, true);
            }, 0,
            0xFE, &AddressingMode::AbsoluteX, 0x94, {cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0x95);
                check_zero_and_neg_flags(&cpu, false, true);
            }, xydeviation,
        }
    }

    #[test]
    fn test_jmp () {
        let mut cpu = create_new_cpu();

        cpu.interpret(vec![0x4C, 0x02, 0x11, 0x00, 0x00]);
        assert_eq!(cpu.program_counter, 0x1103);
        cpu.interpret(vec![0x6C, 0x03, 0x06, 0x02, 0x11, 0x00, 0x00]); // default base address is (time of writing 0x0600)
        assert_eq!(cpu.program_counter, 0x1103);
    }

    #[test]
    fn test_jsr () {
        let mut cpu = create_new_cpu();
        cpu.interpret(vec![0x20, 0x02, 0x11, 0x00, 0x00]);
        assert_eq!(cpu.program_counter, 0x1103);
        assert_eq!(cpu.register_s, 0xFB);
        assert_eq!(cpu.last_mem_write_address, 0x01FC); // check if it pushed something to the stack
        assert_eq!(cpu.last_mem_write_value_u16, 0x0603);
    }

    // load into register opcode tests
    macro_rules! generate_load_test {
        ($test_name:ident, $cpu_target_register:ident, $opcode_map:expr) => {
            #[test]
            fn $test_name () {
                let mut cpu = create_new_cpu();
                let opcodes: HashMap<AddressingMode, u8> = $opcode_map;

                // immediate mode loading of values [1, 127]
                cpu.interpret(vec![opcodes[&AddressingMode::Immediate], 0x07, 0x00]);
                assert_eq!(cpu.$cpu_target_register, 0x07);
                check_zero_and_neg_flags(&cpu, false, false);

                // immediate mode value 0, for a valid test we need to pollute the register first
                cpu.interpret(vec![opcodes[&AddressingMode::Immediate], 0x07, opcodes[&AddressingMode::Immediate], 0x00, 0x00]);
                assert_eq!(cpu.$cpu_target_register, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);

                // immediate mode negative values [128, 255]
                cpu.interpret(vec![opcodes[&AddressingMode::Immediate], 0x97, 0x00]);
                assert_eq!(cpu.$cpu_target_register, 0x97);
                check_zero_and_neg_flags(&cpu, false, true);

                // absolute mode loading of values [1, 127]
                cpu.interpret(vec![opcodes[&AddressingMode::Absolute], 0x06, 0x06, 0x00, 0x05, 0x08, 0x09, 0x0A]);
                assert_eq!(cpu.$cpu_target_register, 0x09);
                check_zero_and_neg_flags(&cpu, false, false);

                // absolute mode value 0
                cpu.interpret(vec![opcodes[&AddressingMode::Immediate], 0x07, opcodes[&AddressingMode::Absolute], 0x08, 0x06, 0x00, 0x05, 0x08, 0x00, 0x0A]);
                assert_eq!(cpu.$cpu_target_register, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);

                // absolute mode negative values [128, 255]
                cpu.interpret(vec![opcodes[&AddressingMode::Absolute], 0x06, 0x06, 0x00, 0x00, 0x00, 0x97]);
                assert_eq!(cpu.$cpu_target_register, 0x97);
                check_zero_and_neg_flags(&cpu, false, true);

                macro_rules! test_absolute_addressing_xy {
                    ($addressing_mode:ident, $addressing_register:ident) => {
                        // absolute x/y mode loading of values [1, 127]
                        cpu.reset();
                        cpu.$addressing_register = 0x01;
                        
                        cpu.interpret_without_reset(vec![opcodes[&AddressingMode::$addressing_mode], 0x06, 0x06, 0x00, 0x05, 0x08, 0x09, 0x0A, 0x0B], 0x0600);
                        assert_eq!(cpu.$cpu_target_register, 0x0A);
                        check_zero_and_neg_flags(&cpu, false, false);
            
                        // absolute x/y mode value 0
                        cpu.reset();
                        cpu.$addressing_register = 0x01;
            
                        cpu.interpret_without_reset(vec![opcodes[&AddressingMode::Immediate], 0x07, opcodes[&AddressingMode::$addressing_mode], 0x08, 0x06, 0x00, 0x05, 0x08, 0x09, 0x00, 0x0B], 0x0600);
                        assert_eq!(cpu.$cpu_target_register, 0x00);
                        check_zero_and_neg_flags(&cpu, true, false);
            
                        // absolute x/y mode negative values [128, 255]
                        cpu.reset();
                        cpu.$addressing_register = 0x01;
            
                        cpu.interpret_without_reset(vec![opcodes[&AddressingMode::$addressing_mode], 0x06, 0x06, 0x00, 0x00, 0x08, 0x03, 0x99, 0x02], 0x0600);
                        assert_eq!(cpu.$cpu_target_register, 0x99);
                        check_zero_and_neg_flags(&cpu, false, true);
                    };
                }

                if opcodes.contains_key(&AddressingMode::AbsoluteY) {
                    test_absolute_addressing_xy!(
                        AbsoluteY, register_y
                    );
                }

                if opcodes.contains_key(&AddressingMode::AbsoluteX) {
                    test_absolute_addressing_xy!(
                        AbsoluteX, register_x
                    );
                }

                // zero page
                cpu.reset();
                cpu.interpret_without_reset(vec![opcodes[&AddressingMode::ZeroPage], 0x05, 0x00, 0x05, 0x08, 0x09, 0x0A, 0x0B], 0x00);
                assert_eq!(cpu.$cpu_target_register, 0x09);
                check_zero_and_neg_flags(&cpu, false, false);

                cpu.reset();
                cpu.interpret_without_reset(vec![opcodes[&AddressingMode::Immediate], 0x07, opcodes[&AddressingMode::ZeroPage], 0x08, 0x00, 0x05, 0x08, 0x09, 0x00, 0x0B, 0x0C], 0x00);
                assert_eq!(cpu.$cpu_target_register, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);

                cpu.reset();
                cpu.interpret_without_reset(vec![opcodes[&AddressingMode::ZeroPage], 0x05, 0x00, 0x00, 0x08, 0x99, 0x03, 0x02], 0x00);
                assert_eq!(cpu.$cpu_target_register, 0x99);
                check_zero_and_neg_flags(&cpu, false, true);

                macro_rules! test_zero_page_addressing_xy {
                    ($addressing_mode:ident, $addressing_register:ident) => {
                        // absolute x/y mode loading of values [1, 127]
                        cpu.reset();
                        cpu.$addressing_register = 0x01;
                        
                        cpu.interpret_without_reset(vec![opcodes[&AddressingMode::$addressing_mode], 0x05, 0x00, 0x05, 0x08, 0x09, 0x0A, 0x0B], 0x00);
                        assert_eq!(cpu.$cpu_target_register, 0x0A);
                        check_zero_and_neg_flags(&cpu, false, false);
            
                        // absolute x/y mode value 0
                        cpu.reset();
                        cpu.$addressing_register = 0x01;
            
                        cpu.interpret_without_reset(vec![opcodes[&AddressingMode::Immediate], 0x07, opcodes[&AddressingMode::$addressing_mode], 0x08, 0x00, 0x05, 0x08, 0x09, 0x0B, 0x00, 0x0C], 0x00);
                        assert_eq!(cpu.$cpu_target_register, 0x00);
                        check_zero_and_neg_flags(&cpu, true, false);
            
                        // absolute x/y mode negative values [128, 255]
                        cpu.reset();
                        cpu.$addressing_register = 0x01;
            
                        cpu.interpret_without_reset(vec![opcodes[&AddressingMode::$addressing_mode], 0x05, 0x00, 0x00, 0x08, 0x03, 0x99, 0x02], 0x00);
                        assert_eq!(cpu.$cpu_target_register, 0x99);
                        check_zero_and_neg_flags(&cpu, false, true);

                        // absolute x/y mode page wrap
                        cpu.reset();
                        cpu.$addressing_register = 0xFE;
            
                        cpu.interpret_without_reset(vec![opcodes[&AddressingMode::$addressing_mode], 0x06, 0x00, 0x00, 0x08, 0x03, 0x99, 0x02], 0x00);
                        assert_eq!(cpu.$cpu_target_register, 0x08);
                        check_zero_and_neg_flags(&cpu, false, false);
                    };
                }

                if opcodes.contains_key(&AddressingMode::ZeroPageX) {
                    test_zero_page_addressing_xy!(
                        ZeroPageX, register_x
                    );
                }

                if opcodes.contains_key(&AddressingMode::ZeroPageY) {
                    test_zero_page_addressing_xy!(
                        ZeroPageY, register_y
                    );
                }
            }
        }
    }

    generate_load_test!(
        test_ldx,
        register_x,
        hashmap!{
            AddressingMode::Immediate => 0xA2 as u8,
            AddressingMode::ZeroPage => 0xA6 as u8,
            AddressingMode::ZeroPageY => 0xB6 as u8,
            AddressingMode::Absolute => 0xAE as u8,
            AddressingMode::AbsoluteY => 0xBE as u8,
        }
    );

    generate_load_test!(
        test_ldy,
        register_y,
        hashmap!{
            AddressingMode::Immediate => 0xA0 as u8,
            AddressingMode::ZeroPage => 0xA4 as u8,
            AddressingMode::ZeroPageX => 0xB4 as u8,
            AddressingMode::Absolute => 0xAC as u8,
            AddressingMode::AbsoluteX => 0xBC as u8,
        }
    );

    generate_load_test!(
        test_lda,
        register_a,
        hashmap!{
            AddressingMode::Immediate => 0xA9 as u8,
            AddressingMode::ZeroPage => 0xA5 as u8,
            AddressingMode::ZeroPageX => 0xB5 as u8,
            AddressingMode::Absolute => 0xAD as u8,
            AddressingMode::AbsoluteX => 0xBD as u8,
            AddressingMode::AbsoluteY => 0xB9 as u8,
        }
    );

    #[test]
    fn test_lsr () {
        let mut cpu = create_new_cpu();
        let xydeviation = 5 as u8;
        opcode_test_case!{
            cpu,
            // without carry set
            0x4A, &AddressingMode::NoneAddressing, 0b0000_0100, cpu.register_a = 0b0000_0100, {
                assert_eq!(cpu.register_a, 0b0000_0010);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
            0x4A, &AddressingMode::NoneAddressing, 0b0000_0101, cpu.register_a = 0b0000_0101, {
                assert_eq!(cpu.register_a, 0b0000_0010);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0x46, &AddressingMode::ZeroPage,    0b0101_0110, cpu.register_a = 0b0101_0110, {
                assert_eq!(cpu.last_mem_write_value, 0b0010_1011);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
            0x56, &AddressingMode::ZeroPageX,   0b0000_0100, {cpu.register_a = 0b0000_0100; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0b0000_0010);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, xydeviation,
            0x4E, &AddressingMode::Absolute,    0b1101_1110, cpu.register_a = 0b1101_1110, {
                assert_eq!(cpu.last_mem_write_value, 0b0110_1111);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
            0x5E, &AddressingMode::AbsoluteX,   0b0000_0000, {cpu.register_a = 0b0000_0000; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0b0000_0000);
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, false);
            }, xydeviation,
            0x4A, &AddressingMode::NoneAddressing, 0b0000_0001, {cpu.register_a = 0b0000_0001; cpu.status = 0b0000_0001;}, {
                assert_eq!(cpu.register_a, 0b0000_0000);
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, true);
            }, 0,
        }
    }

    #[test]
    fn test_nop() {
        let mut  cpu = create_new_cpu();

        cpu.interpret(vec![0xEA, 0xEA, 0xEA, 0xEA, 0x00]);
        assert_eq!(cpu.program_counter, 0x0605);
        assert_eq!(cpu.register_s, 0xFD);
        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.register_y, 0x00);
    }

    #[test]
    fn test_ora () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            0x09, &AddressingMode::Immediate,   0b0101_0101, cpu.register_a = 0b0000_0101, assert_eq!(cpu.register_a, 0b0101_0101), 0,
            0x09, &AddressingMode::Immediate,   0b0111_0101, cpu.register_a = 0b0000_0001, {assert_eq!(cpu.register_a, 0b0111_0101); check_zero_and_neg_flags(&cpu, false, false);}, 0,
            0x09, &AddressingMode::Immediate,   0b1000_0010, cpu.register_a = 0b1000_0001, {assert_eq!(cpu.register_a, 0b1000_0011); check_zero_and_neg_flags(&cpu, false, true);}, 0,
            0x09, &AddressingMode::Immediate,   0b0000_0000, cpu.register_a = 0b0000_0000, {assert_eq!(cpu.register_a, 0b0000_0000); check_zero_and_neg_flags(&cpu, true, false);}, 0,
            0x05, &AddressingMode::ZeroPage,    0b0101_0111, cpu.register_a = 0b0101_0000, assert_eq!(cpu.register_a, 0b0101_0111), 0,
            0x15, &AddressingMode::ZeroPageX,   0b0101_0111, {cpu.register_a = 0b0011_0011; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0b0111_0111), xydeviation,
            0x01, &AddressingMode::IndirectX,   0b0101_0111, {cpu.register_a = 0b0101_0000; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0b0101_0111), xydeviation,
            0x11, &AddressingMode::IndirectY,   0b0101_0110, {cpu.register_a = 0b0011_0010; cpu.register_y = xydeviation;}, assert_eq!(cpu.register_a, 0b0111_0110), xydeviation,
            0x0D, &AddressingMode::Absolute,    0b0101_1110, cpu.register_a = 0b0000_0011, assert_eq!(cpu.register_a, 0b0101_1111), 0,
            0x1D, &AddressingMode::AbsoluteX,   0b0010_0111, {cpu.register_a = 0b0000_0011; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0b0010_0111), xydeviation,
            0x19, &AddressingMode::AbsoluteY,   0b0001_0101, {cpu.register_a = 0b0000_1000; cpu.register_y = xydeviation;}, assert_eq!(cpu.register_a, 0b0001_1101), xydeviation,
        }
    }

    #[test]
    fn test_pha_php_pla_plp () {
        let mut cpu = create_new_cpu();
        opcode_test_case!{
            cpu,
            // without carry set
            0x48, &AddressingMode::NoneAddressing, 0b0000_0101, cpu.register_a = 0b0000_0101, {
                assert_eq!(cpu.last_mem_write_value, 0b0000_0101);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
            0x08, &AddressingMode::NoneAddressing, 0b0000_0011, cpu.status = 0b0000_0011, {
                assert_eq!(cpu.last_mem_write_value, 0b0000_0011);
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0x68, &AddressingMode::NoneAddressing, 0b1000_0111, cpu.register_a = 0b0000_0111, {
                assert_eq!(cpu.register_a, 0b0000_0000);
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, false);
            }, 0,
            0x28, &AddressingMode::NoneAddressing, 0b0000_0101, cpu.status = 0b0000_0101, {
                assert_eq!(cpu.status, 0b0000_0000);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
        }
    }

    #[test]
    fn test_rol () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            // without carry set
            0x2A, &AddressingMode::NoneAddressing, 0b0000_0101, cpu.register_a = 0b0000_0101, {
                assert_eq!(cpu.register_a, 0b0000_1010);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
            0x26, &AddressingMode::ZeroPage,    0b0101_0111, cpu.register_a = 0b0101_0111, {
                assert_eq!(cpu.last_mem_write_value, 0b1010_1110);
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
            }, 0,
            0x36, &AddressingMode::ZeroPageX,   0b0000_0100, {cpu.register_a = 0b0000_0100; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0b0000_1000);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, xydeviation,
            0x2E, &AddressingMode::Absolute,    0b1101_1110, cpu.register_a = 0b1101_1110, {
                assert_eq!(cpu.last_mem_write_value, 0b1011_1100);
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, true);
            }, 0,
            0x3E, &AddressingMode::AbsoluteX,   0b0000_0000, {cpu.register_a = 0b0000_0000; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0b0000_0000);
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, false);
            }, xydeviation,
            // with carry set
            0x2A, &AddressingMode::NoneAddressing, 0b0000_0101, {cpu.register_a = 0b0000_0101; cpu.status = 0b0000_0001;}, {
                assert_eq!(cpu.register_a, 0b0000_1011);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
            0x2A, &AddressingMode::NoneAddressing, 0b1000_0101, {cpu.register_a = 0b1000_0101; cpu.status = 0b0000_0001;}, {
                assert_eq!(cpu.register_a, 0b0000_1011);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0x2A, &AddressingMode::NoneAddressing, 0b0000_0000, {cpu.register_a = 0b0000_0000; cpu.status = 0b0000_0001;}, {
                assert_eq!(cpu.register_a, 0b0000_0001);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
        }
    }

    #[test]
    fn test_ror () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            // without carry set
            0x6A, &AddressingMode::NoneAddressing, 0b0000_0101, cpu.register_a = 0b0000_0101, {
                assert_eq!(cpu.register_a, 0b0000_0010);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
            }, 0,
            0x66, &AddressingMode::ZeroPage,    0b0101_0110, cpu.register_a = 0b0101_0110, {
                assert_eq!(cpu.last_mem_write_value, 0b0010_1011);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
            0x76, &AddressingMode::ZeroPageX,   0b0000_0100, {cpu.register_a = 0b0000_0100; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0b0000_0010);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, xydeviation,
            0x6E, &AddressingMode::Absolute,    0b1101_1110, cpu.register_a = 0b1101_1110, {
                assert_eq!(cpu.last_mem_write_value, 0b0110_1111);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
            }, 0,
            0x7E, &AddressingMode::AbsoluteX,   0b0000_0000, {cpu.register_a = 0b0000_0000; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0b0000_0000);
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, false);
            }, xydeviation,
            // with carry set
            0x6A, &AddressingMode::NoneAddressing, 0b0000_0101, {cpu.register_a = 0b0000_0101; cpu.status = 0b0000_0001;}, {
                assert_eq!(cpu.register_a, 0b1000_0010);
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, true);
            }, 0,
            0x6A, &AddressingMode::NoneAddressing, 0b1000_0101, {cpu.register_a = 0b1000_0101; cpu.status = 0b0000_0001;}, {
                assert_eq!(cpu.register_a, 0b1100_0010);
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, true);
            }, 0,
            0x6A, &AddressingMode::NoneAddressing, 0b0000_0000, {cpu.register_a = 0b0000_0000; cpu.status = 0b0000_0001;}, {
                assert_eq!(cpu.register_a, 0b1000_0000);
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
            }, 0,
        }
    }

    #[test]
    fn test_rti () {
        todo!()
    }

    #[test]
    fn test_rts () {
        let mut cpu = create_new_cpu();

        cpu.interpret(vec![0x20, 0x07, 0x06, 0x00, 0x02, 0x02, 0x02, 0x60, 0x02, 0x02]);
        assert_eq!(cpu.program_counter, 0x0604);
        assert_eq!(cpu.register_s, 0xFD);
    }

    #[test]
    fn test_sbc () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            0xE9, &AddressingMode::Immediate, 0x23, cpu.register_a = 0x25, {
                assert_eq!(cpu.register_a, 0x01);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
                check_overflow_flag(&cpu, false);
            }, 0,
            0xE9, &AddressingMode::Immediate, 0x22, cpu.register_a = 0x23, {
                assert_eq!(cpu.register_a, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, true);
                check_overflow_flag(&cpu, false);
            }, 0,
            0xE9, &AddressingMode::Immediate, 0x20, { cpu.register_a = 0x23; cpu.status = 0b0000_0001; }, {
                assert_eq!(cpu.register_a, 0x03);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, true);
                check_overflow_flag(&cpu, false);
            }, 0,
            0xE9, &AddressingMode::Immediate, 0x23, {cpu.register_a = 0x23; cpu.status = 0b0000_0001;}, {
                assert_eq!(cpu.register_a, 0x00);
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, true);
                check_overflow_flag(&cpu, false);
            }, 0,
            0xE9, &AddressingMode::Immediate, 0x23, {cpu.register_a = 0x23; cpu.status = 0b0000_0000;}, {
                assert_eq!(cpu.register_a, 0xFF);
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
                check_overflow_flag(&cpu, false);
            }, 0,
            0xE9, &AddressingMode::Immediate, 0xA1, {cpu.register_a = 0x10; cpu.status = 0b0000_0000;}, {
                assert_eq!(cpu.register_a, 0x6E);
                check_zero_and_neg_flags(&cpu, false, false);
                check_carry_flag(&cpu, false);
                check_overflow_flag(&cpu, true);
            }, 0,
            0xED, &AddressingMode::Absolute, 0x22, cpu.register_a = 0x23, assert_eq!(cpu.register_a, 0x00), 0,
            0xFD, &AddressingMode::AbsoluteX, 0x02, {cpu.register_a = 0x23; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0x20), xydeviation,
            0xF9, &AddressingMode::AbsoluteY, 0x04, {cpu.register_a = 0x23; cpu.register_y = xydeviation;}, assert_eq!(cpu.register_a, 0x1E), xydeviation,
            0xE5, &AddressingMode::ZeroPage, 0x20, cpu.register_a = 0x23, assert_eq!(cpu.register_a, 0x02), 0,
            0xF5, &AddressingMode::ZeroPageX, 0x10, {cpu.register_a = 0x23; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0x12), xydeviation,
            0xE1, &AddressingMode::IndirectX, 0x00, {cpu.register_a = 0x23; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0x22), xydeviation,
            0xF1, &AddressingMode::IndirectY, 0x01, {cpu.register_a = 0x23; cpu.register_y = xydeviation;}, assert_eq!(cpu.register_a, 0x21), xydeviation,
        }
    }

    #[test]
    fn test_slo () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            0x07, &AddressingMode::ZeroPage,  0b0001_0101, cpu.register_a = 0b0101_0000, assert_eq!(cpu.register_a, 0b0111_1010), 0,
            0x17, &AddressingMode::ZeroPageX, 0b0001_0101, {cpu.register_a = 0b1011_0011; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.register_a, 0b1011_1011);
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, false);
            }, xydeviation,
            0x03, &AddressingMode::IndirectX, 0b1001_0101, {cpu.register_a = 0b1011_0011; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.register_a, 0b1011_1011);
                check_zero_and_neg_flags(&cpu, false, true);
                check_carry_flag(&cpu, true);
            }, xydeviation,
            0x13, &AddressingMode::IndirectY, 0b1000_0000, {cpu.register_a = 0b0000_0000; cpu.register_y = xydeviation;}, {
                assert_eq!(cpu.register_a, 0b00);
                check_zero_and_neg_flags(&cpu, true, false);
                check_carry_flag(&cpu, true);
            }, xydeviation,
            0x0F, &AddressingMode::Absolute,  0b0001_0101, cpu.register_a = 0b1011_0011, assert_eq!(cpu.register_a, 0b1011_1011), 0,
            0x1F, &AddressingMode::AbsoluteX, 0b0001_0101, {cpu.register_a = 0b1011_0011; cpu.register_x = xydeviation;}, assert_eq!(cpu.register_a, 0b1011_1011), xydeviation,
            0x1B, &AddressingMode::AbsoluteY, 0b0001_0101, {cpu.register_a = 0b1011_0011; cpu.register_y = xydeviation;}, assert_eq!(cpu.register_a, 0b1011_1011), xydeviation,
        }
    }

    #[test]
    fn test_sta () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            0x85, &AddressingMode::ZeroPage,  0x77, cpu.register_a = 0x42, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, 0,
            0x95, &AddressingMode::ZeroPageX, 0x77, {cpu.register_a = 0x42; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, xydeviation,
            0x81, &AddressingMode::IndirectX, 0x77, {cpu.register_a = 0x42; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, xydeviation,
            0x91, &AddressingMode::IndirectY, 0x77, {cpu.register_a = 0x42; cpu.register_y = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, xydeviation,
            0x8D, &AddressingMode::Absolute,  0x77, cpu.register_a = 0x42, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, 0,
            0x9D, &AddressingMode::AbsoluteX, 0x77, {cpu.register_a = 0x42; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, xydeviation,
            0x99, &AddressingMode::AbsoluteY, 0x77, {cpu.register_a = 0x42; cpu.register_y = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, xydeviation,
        }
    }

    #[test]
    fn test_stx_and_sty () {
        let mut cpu = create_new_cpu();
        let xydeviation = 7 as u8;
        opcode_test_case!{
            cpu,
            0x86, &AddressingMode::ZeroPage,  0x77, cpu.register_x = 0x42, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, 0,
            0x96, &AddressingMode::ZeroPageY, 0x77, {cpu.register_x = 0x42; cpu.register_y = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, xydeviation,
            0x8E, &AddressingMode::Absolute,  0x77, cpu.register_x = 0x42, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, 0,
            0x84, &AddressingMode::ZeroPage,  0x77, cpu.register_y = 0x42, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, 0,
            0x94, &AddressingMode::ZeroPageX, 0x77, {cpu.register_y = 0x42; cpu.register_x = xydeviation;}, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, xydeviation,
            0x8C, &AddressingMode::Absolute,  0x77, cpu.register_y = 0x42, {
                assert_eq!(cpu.last_mem_write_value, 0x42);
            }, 0,
        }
    }

    // transfer between register opcodes follow
    macro_rules! generate_transfer_test {
        ($test_name:ident, $cpu_source_register:ident, $cpu_target_register:ident, $opcode:expr, $status_affected:expr) => {
            #[test]
            fn $test_name() {
                let mut cpu = create_new_cpu();
                cpu.$cpu_source_register = 0x13;
                cpu.interpret_without_reset(vec![$opcode, 0x00], 0x00);
                assert_eq!(cpu.$cpu_target_register, 0x13);
                if $status_affected {
                    check_zero_and_neg_flags(&cpu, false, false);
                }

                cpu.$cpu_source_register = 0x00;
                cpu.interpret_without_reset(vec![$opcode, 0x00], 0x00);
                assert_eq!(cpu.$cpu_target_register, 0x00);
                if $status_affected {
                    check_zero_and_neg_flags(&cpu, true, false);
                }

                cpu.$cpu_source_register = 0x92;
                cpu.interpret_without_reset(vec![$opcode, 0x00], 0x00);
                assert_eq!(cpu.$cpu_target_register, 0x92);
                if $status_affected {
                    check_zero_and_neg_flags(&cpu, false, true);
                }
            }
        };
    }

    generate_transfer_test!(
        test_tax, register_a, register_x, 0xAA, true
    );

    generate_transfer_test!(
        test_tay, register_a, register_y, 0xA8, true
    );

    generate_transfer_test!(
        test_tsx, register_s, register_x, 0xBA, true
    );

    generate_transfer_test!(
        test_txa, register_x, register_a, 0x8A, true
    );

    generate_transfer_test!(
        test_txs, register_x, register_s, 0x9A, false
    );

    generate_transfer_test!(
        test_tya, register_y, register_a, 0x98, true
    );
}