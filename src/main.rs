#![allow(unused_imports)]

use std::io::{stdin, Read};

use vm16::memory::Memory;
use vm16::cpu::Cpu;
use vm16::constants::*;

fn main() {
    let program = vec![                      // start:
        MOV_MEM_REG, 0x01, 0x00,   R1,       //   mov #0x0100, r1
        MOV_LIT_REG, 0x00, 0x01,   R2,       //   mov  0x0001, r2
        ADD_REG_REG,   R1,   R2,             //   add r1, r2
        MOV_REG_MEM,  ACC, 0x01, 0x00,       //   mov acc, #0x0100
        JMP_NOT_EQ,  0x00, 0x05, 0x00, 0x00, //   jne 0x0005, #0x0000
        HALT,
    ];
    
    let mut memory = Memory::new(0xFFFF);
    memory.write_bytes(0, program);
    let mut cpu = Cpu::new(memory);
    
    cpu.view_registers(&[IP, ACC, R1, R2]);
    cpu.view_memory(cpu.get_register_value("ip".to_string()), 1);
    cpu.view_memory(0x0100, 1);
    while !cpu.step() {
        stdin().read_line(&mut "".to_owned()).unwrap();
        cpu.view_registers(&[IP, ACC, R1, R2]);
        cpu.view_memory(cpu.get_register_value("ip".to_string()), 1);
        cpu.view_memory(0x0100, 1);
    }
}
