use crate::cpu::{AddressingMode, CPU};
use crate::mem::Mem;
use crate::opcodes::OpCode;

fn parse_addressing_information(cpu: &CPU, opcode: &&OpCode) -> String {
    // since we are inside the trace callback, the program_counter still points to the opcode of the instruction and therefore we have to 
    // be careful when using it, inside the cpu.rs logic we are usually already one step further, reading the operand of the opcode
    let result = match opcode.len {
        1 => {
            match opcode.code {
                0x0a | 0x2a | 0x4a | 0x6a => format!("A  "),
                _ => "".to_string()
            }
        },
        2 => {
            let operand_value = cpu.mem_read(cpu.program_counter + 1);
            let (real_address, stored_value) = match opcode.mode {
                AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
                _ => {
                    let address = cpu.get_absolute_address(&opcode.mode, cpu.program_counter + 1);
                    (address, cpu.mem_read(address))
                }
            };
            match opcode.mode {
                // assuming these are the branching opcodes
                AddressingMode::NoneAddressing => {
                    let branch_to_address = if operand_value > 127 {
                        let result = cpu.program_counter + 2 + (operand_value as u16);
                        result.wrapping_sub(256)
                    } else {
                        cpu.program_counter.wrapping_add(2).wrapping_add(operand_value as u16)
                    };
                    format!("${:04x}", branch_to_address)
                },
                AddressingMode::Immediate => format!("#${:02x}", operand_value),
                AddressingMode::ZeroPage => format!("${:02x}", operand_value),
                AddressingMode::ZeroPageX => format!("${:02x},X", operand_value),
                AddressingMode::ZeroPageY => format!("${:02x},Y", operand_value),
                AddressingMode::IndirectX => format!("(${:02x},X)", operand_value),
                AddressingMode::IndirectY => format!("(${:02x}),Y", operand_value),
                _ => "".to_string()
            }
        }
        3 => {
            let operand_value: u16 = cpu.mem_read_u16(cpu.program_counter + 1);
            match opcode.mode {
                AddressingMode::Absolute => format!("${:04x}", operand_value),
                AddressingMode::AbsoluteX => format!("${:04x},X", operand_value),
                AddressingMode::AbsoluteY => format!("${:04x},Y", operand_value),
                AddressingMode::Indirect => format!("(${:04x})", operand_value),
                _ => "".to_string(),
            }
        }
        _ => panic!("Unsupported opcode, cannot parse addressing information!")
    };

    result
}

fn parse_detailed_addressing_information(cpu: &CPU, opcode: &&OpCode) -> String {
    let result = match opcode.len {
        1 => "".to_string(),
        2 => {
            let operand_value = cpu.mem_read(cpu.program_counter + 1);
            let (real_address, stored_value) = match opcode.mode {
                AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
                _ => {
                    let address = cpu.get_absolute_address(&opcode.mode, cpu.program_counter + 1);
                    (address, cpu.mem_read(address))
                }
            };
            match opcode.mode {
                AddressingMode::ZeroPage => format!("= {:02x}", stored_value),
                AddressingMode::ZeroPageX | AddressingMode::ZeroPageY => format!("@ {:02x} = {:02x}", real_address, stored_value),
                AddressingMode::IndirectX => {
                    format!("@ {:02x} = {:04x} = {:02x}", operand_value.wrapping_add(cpu.register_x), real_address, stored_value)
                },
                AddressingMode::IndirectY => {
                    let lo = cpu.mem_read(operand_value as u16);
                    let hi = cpu.mem_read(operand_value.wrapping_add(1) as u16);
                    let indirect_address = (lo as u16) + ((hi as u16) << 8);
                    format!("= {:04x} @ {:04x} = {:02x}", indirect_address, real_address, stored_value)
                },
                _ => "".to_string()
            }
        },
        3 => {
            let (real_address, stored_value) = match opcode.mode {
                AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
                _ => {
                    let address = cpu.get_absolute_address(&opcode.mode, cpu.program_counter + 1);
                    (address, cpu.mem_read(address))
                }
            };
            match opcode.code {
                0x20 | 0x4C => "".to_string(),
                _ => match opcode.mode {
                    AddressingMode::Absolute => format!("= {:02x}", stored_value),
                    AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => format!("@ {:04x} = {:02x}", real_address, stored_value),
                    AddressingMode::Indirect => format!("= {:04x}", real_address),
                    _ => "".to_string()
                }
            }
        },
        _ => panic!("Unsupported opcode, cannot parse addressing information!")
    };

    result
}

fn parse_register_stati(cpu: &CPU) -> String{
    format!("A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        cpu.register_a,
        cpu.register_x,
        cpu.register_y,
        cpu.status,
        cpu.register_s
    )
}

pub fn trace(cpu: &mut CPU, opcode: &&OpCode) -> String {
    let mut full_instruction = Vec::new();
    for i in 0 .. opcode.len as u16 {
        full_instruction.push(cpu.mem_read(cpu.program_counter + i));
    }

    let instruction_str = full_instruction
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");
    
    let addressing_string = parse_addressing_information(cpu, opcode);
    let addressing_details = parse_detailed_addressing_information(cpu, opcode);

    let register_stati = parse_register_stati(cpu);

    let part_one = format!("{:04x}  {:8} {: >4} {} {}", cpu.program_counter, instruction_str, opcode.name, addressing_string, addressing_details);
    format!("{:47} {}", part_one, register_stati).to_uppercase()
}