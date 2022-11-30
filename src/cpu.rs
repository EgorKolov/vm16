use std::collections::HashMap;

use crate::byte_ops;
use crate::memory::{Memory, Registers};
use crate::constants::*;

pub struct Cpu {
    pub memory: Memory,
    register_names: Vec<String>,
    registers: Registers, //Memory,
    register_map: HashMap<String, u8>,
    halted: bool
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        let register_names = vec![
            "ip".to_string(), "acc".to_string(), 
            "r1".to_string(), "r2".to_string(), "r3".to_string(), "r4".to_string(), 
            "r5".to_string(), "r6".to_string(), "r7".to_string(), "r8".to_string(), 
        ];
        
        let register_map = register_names.iter()
            .enumerate()
            .map(|(i, s,)| (s.clone(), i as u8))
            .collect();
        
        let registers = Registers::new(register_names.len());//Memory::new(2 * &register_names.len());
            
        Self {
            memory,
            registers,
            register_map,
            register_names,
            halted: false,
        }
    }
    
    pub fn get_register_value(&self, name: String) -> u16 {
        if !self.register_map.contains_key(&name) {
            panic!("get_register_value: {name}: no such register")
        }
        let index = self.register_map[&name];
        self.registers.read(index)//self.registers.read_u16(index)
    }
    
    pub fn set_register_value(&mut self, name: String, value: u16) {
        if !self.register_map.contains_key(&name) {
            panic!("set_register_value: {name}: no such register")
        }
        let index = self.register_map[&name];
        self.registers.write(index, value);//self.registers.write_u16(index, value);
    }
    
    fn fetch_u8(&mut self) -> u8 {
        let ip_val = self.registers.read(IP);//self.registers.read_u16(2 * IP as u16); 
        let instruction = self.memory.read_u8(ip_val);
        self.registers.write(IP, ip_val + 1); //self.registers.write_u16(2 * IP as u16, ip + 1); 
        instruction
    }
    
    fn fetch_u16(&mut self) -> u16 {
        let ip_val = self.registers.read(IP);//self.registers.read_u16(2 * IP as u16);
        let instruction = self.memory.read_u16(ip_val);
        self.registers.write(IP, ip_val + 2);//self.registers.write_u16(2 * IP as u16, ip_val + 2);
        instruction
    }
    
    fn execute(&mut self, instruction: u8) {
        use crate::constants::*;
        
        match instruction {
            MOV_LIT_REG => { // move literal to register [3 bytes]
                let val = self.fetch_u16();
                let reg = self.fetch_u8();
                self.registers.write(reg, val);//self.registers.write_u16(2 * reg as u16, val);
            },
            MOV_REG_REG => { // move register to register [2 bytes]
                let (reg_src, reg_dst) = byte_ops::u16_to_u8(self.fetch_u16());
                self.registers.write(reg_dst, self.registers.read(reg_src));
                //self.registers.write_u16(2 * reg_dst as u16, self.registers.read_u16(2 * reg_src as u16));
            },
            MOV_REG_MEM => { // move register to memory [3 bytes]
                let reg = self.fetch_u8();
                let mem = self.fetch_u16();
                self.memory.write_u16(mem, self.registers.read(reg));
                //self.memory.write_u16(mem, self.registers.read_u16(2 * reg as u16));
            },
            MOV_MEM_REG => { // move memory to register [3 bytes]
                let mem = self.fetch_u16();
                let reg = self.fetch_u8();
                self.registers.write(reg, self.memory.read_u16(mem));
                //self.registers.write_u16(2 * reg as u16, self.memory.read_u16(mem));
            },
            ADD_REG_REG => { // add two registers and write result to acc [3 bytes]
                let (reg_a, reg_b) = byte_ops::u16_to_u8(self.fetch_u16());
                let (val_a, val_b) = (self.registers.read(reg_a), self.registers.read(reg_b));
                //let (val_a, val_b) = (self.registers.read_u16(2 * reg_a as u16), self.registers.read_u16(2 * reg_b as u16));
                self.registers.write(ACC, val_a.wrapping_add(val_b));
                //self.set_register_value("acc".to_string(), val_a.wrapping_add(val_b));
            },
            JMP_NOT_EQ => { // jump to address if literal not equal to acc [4 bytes]
                let val = self.fetch_u16();
                let adr = self.fetch_u16();
                if val != self.registers.read(ACC) {
                    self.registers.write(IP, adr);
                }
                //if val != self.registers.read_u16(2 * ACC as u16) {
                //    self.registers.write_u16(2 * IP as u16, adr);
                //}
            }
            HALT => {
                self.halted = true
            }
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
        for line in 0..lines {
            let address = start_at + 16 * line;
            print!("0x{:04X}: ", address);
            for offset in 0..16 {
                print!("{:02X} ", self.memory.read_u8(address + offset));
            }
            println!()
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
        cpu.set_register_value("r1".to_string(), 0x1234);
        cpu.set_register_value("r2".to_string(), 0xFF01);
        
        assert_eq!(0x1234, cpu.get_register_value("r1".to_string()));
        assert_eq!(0xFF01, cpu.get_register_value("r2".to_string()));
        
        cpu.set_register_value("r1".to_string(), 0xF801);
        assert_eq!(0xF801, cpu.get_register_value("r1".to_string()));
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
        assert_eq!(0x1234, cpu.get_register_value("r1".to_string()));
        cpu.step();
        assert_eq!(0xABCD, cpu.get_register_value("r2".to_string()));
        cpu.step();
        assert_eq!(0x1234 + 0xABCD, cpu.get_register_value("acc".to_string()));
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
        assert_eq!(0x1234 + 0xABCD, cpu.get_register_value("r3".to_string()));
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
}
