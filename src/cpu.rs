use crate::byte_ops;
use crate::memory::{Memory, Registers};

use crate::constants::*;

pub struct Cpu {
    pub memory: Memory,
    pub registers: Registers, 
    register_names: Vec<String>,
    
    stack_frame_size: i16,
    halted: bool
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        let register_names = vec![
            "ip".to_string(), "acc".to_string(), 
            "r1".to_string(), "r2".to_string(), "r3".to_string(), "r4".to_string(), 
            "r5".to_string(), "r6".to_string(), "r7".to_string(), "r8".to_string(), 
            "sp".to_string(), "fp".to_string(),
        ];
        
        let mut registers = Registers::new(register_names.len() as u16);
        registers.write(SP, memory.len() as u16 - 2);
        registers.write(FP, memory.len() as u16 - 2);
            
        Self {
            memory,
            registers,
            register_names,
            
            stack_frame_size: 0,
            halted: false,
        }
    }
    
    fn fetch_u8(&mut self) -> u8 {
        let ip_val = self.registers.read(IP);
        let instruction = self.memory.read_u8(ip_val);
        self.registers.write(IP, ip_val + 1); 
        instruction
    }
    
    fn fetch_u16(&mut self) -> u16 {
        let ip_val = self.registers.read(IP);
        let instruction = self.memory.read_u16(ip_val);
        self.registers.write(IP, ip_val + 2);
        instruction
    }
    
    fn push(&mut self, value: u16) {
        let sp_val = self.registers.read(SP);
        self.memory.write_u16(sp_val, value);
        self.stack_frame_size += 2;
        self.registers.write(SP, sp_val - 2);
    }
    
    fn push_state(&mut self) {
        for reg in R1..=R8 { // saving registers
            self.push(self.registers.read(reg));
        }
        self.push(self.registers.read(IP));
        
        self.push((self.stack_frame_size + 2) as u16);
        self.stack_frame_size = 0;
        
        self.registers.write(FP, self.registers.read(SP));
    }
    
    fn pop(&mut self) -> u16 {
        let sp_val = self.registers.read(SP);
        self.registers.write(SP, sp_val + 2);
        self.stack_frame_size -= 2;
        self.memory.read_u16(sp_val + 2)
    }
    
    fn pop_state(&mut self) {
        let fp_val = self.registers.read(FP);
        self.registers.write(SP, fp_val);
        
        self.stack_frame_size = self.pop() as i16;
        let stack_frame_size = self.stack_frame_size;
        
        let tmp = self.pop(); self.registers.write(IP, tmp); // restoring registers
        for reg in (R1..=R8).rev() {
            let tmp = self.pop(); self.registers.write(reg, tmp);
        }
        
        for _arg_i in 0..self.pop() { // getting rid of passed arguments
            self.pop();
        }
        self.registers.write(FP, (fp_val as i16 + stack_frame_size) as u16);
    }
    
    fn execute(&mut self, instruction: u8) {
        use crate::constants::*;
        
        match instruction {
            MOV_LIT_REG => { // move literal to register [3 bytes]
                let val = self.fetch_u16();
                let reg = self.fetch_u8();
                self.registers.write(reg, val);
            },
            MOV_REG_REG => { // move register to register [2 bytes]
                let (reg_src, reg_dst) = byte_ops::u16_to_u8(self.fetch_u16());
                self.registers.write(reg_dst, self.registers.read(reg_src));
            },
            MOV_REG_MEM => { // move register to memory [3 bytes]
                let reg = self.fetch_u8();
                let mem = self.fetch_u16();
                self.memory.write_u16(mem, self.registers.read(reg));
            },
            MOV_MEM_REG => { // move memory to register [3 bytes]
                let mem = self.fetch_u16();
                let reg = self.fetch_u8();
                self.registers.write(reg, self.memory.read_u16(mem));
            },
            ADD_REG_REG => { // add two registers and write result to acc [3 bytes]
                let (reg_a, reg_b) = byte_ops::u16_to_u8(self.fetch_u16());
                let (val_a, val_b) = (self.registers.read(reg_a), self.registers.read(reg_b));
                self.registers.write(ACC, val_a.wrapping_add(val_b));
            },
            JMP_NOT_EQ => { // jump to address if literal not equal to acc [4 bytes]
                let val = self.fetch_u16();
                let adr = self.fetch_u16();
                if val != self.registers.read(ACC) {
                    self.registers.write(IP, adr);
                }
            },
            PSH_LIT => { // push literal on a stack [2 bytes]
                let val = self.fetch_u16();
                self.push(val);
            },
            PSH_REG => { // push register on a stack [1 bytes]
                let reg = self.fetch_u8();
                let reg_val = self.registers.read(reg);
                self.push(reg_val);
            },
            POP => { // pop stack to register [1 bytes]
                let reg = self.fetch_u8();
                let stack_val = self.pop();
                self.registers.write(reg, stack_val);
            },
            CAL_LIT => { // call subroutine from literal [2 bytes]
                let adr = self.fetch_u16();
                self.push_state();
                self.registers.write(IP, adr);
            },
            CAL_REG => { // call subroutine from register [1 bytes]
                let reg = self.fetch_u8();
                self.push_state();
                let adr = self.registers.read(reg);
                self.registers.write(IP, adr);
            },
            RET => { // return from subroutine [0 bytes]
                self.pop_state();
            },
            HALT => {
                self.halted = true;
            },
            _ => panic!("execute: {instruction}: no such instruction")
        }
    }
    
    pub fn step(&mut self) -> bool {
        if !self.halted {
            let instruction = self.fetch_u8();
            self.execute(instruction);
        }
        self.halted
    }
    
    pub fn view_registers(&self, regs: &[u8]) {
        println!("Registers:");
        for reg in regs {
            print!("{}: 0x{:04X}; ", self.register_names[*reg as usize], self.registers.read(*reg));
        }
        println!();
    }
    
    pub fn view_memory(&self, start_at: u16, lines: u16) {
        print!("Memory: ");
        for i in 0..16 {
            print!("{i:+2X} ");
        }
        println!();
        for line in 0..lines as usize {
            let address = start_at as usize + 16 * line;
            if address >= self.memory.len() as usize {
                break;
            }
            print!("0x{:04X}: ", address);
            for offset in 0..16 {
                if address + offset < self.memory.len() as usize {
                    print!("{:02X} ", self.memory.read_u8((address + offset) as u16));
                }
            }
            println!();
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::memory::Memory;
    use crate::cpu::Cpu;
    use crate::constants::*;
    
    #[test]
    fn basic_register_rw() {
        let memory = Memory::new(0xFF);
        let mut cpu = Cpu::new(memory);
        cpu.registers.write(R1, 0x1234);
        cpu.registers.write(R2, 0xFF01);
        
        assert_eq!(0x1234, cpu.registers.read(R1));
        assert_eq!(0xFF01, cpu.registers.read(R2));
        
        cpu.registers.write(R1, 0xF801);
        assert_eq!(0xF801, cpu.registers.read(R1));
    }
    
    #[test]
    fn basic_addition() {
        let program = vec![
            MOV_LIT_REG, 0x12, 0x34,   R1,  // mov 0x1234, r1
            MOV_LIT_REG, 0xAB, 0xCD,   R2,  // mov 0xABCD, r2
            ADD_REG_REG,   R1,   R2,        // add r1, r2
        ];
        
        let mut memory = Memory::new(0xFF);
        memory.write_bytes(0, program);
        let mut cpu = Cpu::new(memory);
        
        cpu.step();
        assert_eq!(0x1234, cpu.registers.read(R1));
        cpu.step();
        assert_eq!(0xABCD, cpu.registers.read(R2));
        cpu.step();
        assert_eq!(0x1234 + 0xABCD, cpu.registers.read(ACC));
    }
    
    #[test]
    fn basic_memory_rw() {
        let program = vec![
            MOV_LIT_REG, 0x12, 0x34,   R1, // mov 0x1234, r1
            MOV_LIT_REG, 0xAB, 0xCD,   R2, // mov 0xABCD, r2
            ADD_REG_REG,   R1,   R2,       // add r1, r2
            MOV_REG_MEM,  ACC, 0x00, 0x20, // mov acc, #0x0012
            MOV_MEM_REG, 0x00, 0x20,   R3, // mov #0x0012, r3
        ];
        
        let mut memory = Memory::new(0xFF);
        memory.write_bytes(0, program);
        let mut cpu = Cpu::new(memory);
        
        cpu.step();
        cpu.step();
        cpu.step();
        cpu.step();
        assert_eq!(0x1234 + 0xABCD, cpu.memory.read_u16(0x0020));
        cpu.step();
        assert_eq!(0x1234 + 0xABCD, cpu.registers.read(R3));
    }
    
    #[test]
    fn basic_jump_not_equals() {
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
        
        while !cpu.step() { }
        assert_eq!(0x0005, cpu.memory.read_u16(0x0100));
    }
    
    #[test]
    fn basic_stack() {
        let program = vec![
            MOV_LIT_REG, 0x51, 0x42, R1, // mov 0x5151, r1
            MOV_LIT_REG, 0x42, 0x51, R2, // mov 0x4242, r2
            PSH_REG, R1,                 // psh r1
            PSH_REG, R2,                 // psh r2
            POP, R1,                     // pop r1
            POP, R2,                     // pop r2
            HALT,
        ];
        
        let mut memory = Memory::new(0xFFFF);
        memory.write_bytes(0, program);
        let mut cpu = Cpu::new(memory);
        
        while !cpu.step() { }
        assert_eq!(0x4251, cpu.registers.read(R1));
        assert_eq!(0x5142, cpu.registers.read(R2));
    }
    
    #[test]
    fn basic_call() {
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
        
        while !cpu.step() { }
        assert_eq!(0x001E, cpu.registers.read(IP));
        assert_eq!(0x1234, cpu.registers.read(R1));
        assert_eq!(0xABCD, cpu.registers.read(R4));
        assert_eq!(0x0000, cpu.registers.read(R8));
        assert_eq!(0x4444, cpu.memory.read_u16(cpu.registers.read(SP) + 2));
        assert_eq!(0x1111, cpu.memory.read_u16(cpu.registers.read(SP) + 4));
        assert_eq!(0x2222, cpu.memory.read_u16(cpu.registers.read(SP) + 6));
        assert_eq!(0x3333, cpu.memory.read_u16(cpu.registers.read(SP) + 8));
    }
}
