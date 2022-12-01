#![allow(unused_imports)]

use std::io::{stdin, Read};

use vm16::memory::Memory;
use vm16::cpu::Cpu;
use vm16::constants::*;

fn main() {
    let program = vec![
        PSH_LIT,     0x33, 0x33,     // psh 0x3333
        PSH_LIT,     0x22, 0x22,     // psh 0x2222
        PSH_LIT,     0x11, 0x11,     // psh 0x1111
        
        MOV_LIT_REG, 0x12, 0x34, R1, // mov 0x1234, r1
        MOV_LIT_REG, 0xAB, 0xCD, R4, // mov 0xABCD, r4
        
        PSH_LIT,     0xA2, 0x90,     // psh 0xA290
        PSH_LIT,     0x00, 0x01,     // psh 0x0001
        CAL_LIT,     0x30, 0x00,     // cal my_subroutine:
        
        PSH_LIT,     0x44, 0x44,     // psh 0x4444
        HALT,
    ];
    
    let my_subroutine = vec![            // my_subroutine: ;; 0x3000
        PSH_LIT,     0x01, 0x02,     //   psh 0x0102
        PSH_LIT,     0x03, 0x04,     //   psh 0x0304
        PSH_LIT,     0x05, 0x06,     //   psh 0x0506
        
        MOV_LIT_REG, 0x07, 0x08, R1, // mov 0x0708, r1
        MOV_LIT_REG, 0x09, 0x0A, R8, // mov 0x090A, r8
        
        RET                          //   ret
    ];
    
    let mut memory = Memory::new(0xFFFF);
    memory.write_bytes(0x0000, program);
    memory.write_bytes(0x3000, my_subroutine);
    let mut cpu = Cpu::new(memory);
        
    cpu.view_registers(&[IP, R1, R4, R8, SP, FP]);
    cpu.view_memory(cpu.registers.read(IP), 1);
    cpu.view_memory(cpu.registers.read(SP) + 2, 3);
    while !cpu.step() {
        stdin().read_line(&mut "".to_owned()).unwrap();
        cpu.view_registers(&[IP, R1, R4, R8, SP, FP]);
        cpu.view_memory(cpu.registers.read(IP), 1);
        cpu.view_memory(cpu.registers.read(SP) + 2, 3);
    }
}
