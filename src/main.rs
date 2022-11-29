use vm16::memory::Memory;
use vm16::cpu::Cpu;
use vm16::instructions::*;

fn main() {
    let program = vec![
        MOV_LIT_R1, 0x12, 0x34,  // mov 0x1234, r1
        MOV_LIT_R2, 0xAB, 0xCD,  // mov 0xABCD, r2
        ADD_REG_REG, 0x02, 0x03, // add r1, r2
    ];
    
    let mut memory = Memory::new(0xFF);
    memory.write_bytes(0, program);
    
    let mut cpu = Cpu::new(memory);
    let observed_regs = ["ip".to_string(), "r1".to_string(), "r2".to_string(), "acc".to_string()];
    
    cpu.debug(&observed_regs);
    for _ in 0..3 {
        cpu.step();
        cpu.debug(&observed_regs);
    }
}
