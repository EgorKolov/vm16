use std::collections::HashMap;

use crate::byte_ops;
use crate::memory::Memory;

pub struct Cpu {
    memory: Memory,
    //register_names: Vec<String>,
    registers: Memory,
    register_map: HashMap<String, u16>
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
            .map(|(i, s,)| (s.clone(), 2 * i as u16))
            .collect();
        
        let registers = Memory::new(2 * &register_names.len());
            
        Self {
            memory,
            registers,
            register_map,
            //register_names,
        }
    }
    
    pub fn get_register_value(&self, name: String) -> u16 {
        if !self.register_map.contains_key(&name) {
            panic!("get_register_value: {name}: no such register")
        }
        let index = self.register_map[&name];
        self.registers.read_u16(index)
    }
    
    pub fn set_register_value(&mut self, name: String, value: u16) {
        if !self.register_map.contains_key(&name) {
            panic!("set_register_value: {name}: no such register")
        }
        let index = self.register_map[&name];
        self.registers.write_u16(index, value);
    }
    
    fn fetch_u8(&mut self) -> u8 {
        let ip = self.get_register_value("ip".to_string());
        let instruction = self.memory.read_u8(ip);
        self.set_register_value("ip".to_string(), ip + 1);
        instruction
    }
    
    fn fetch_u16(&mut self) -> u16 {
        let ip = self.get_register_value("ip".to_string());
        let instruction = self.memory.read_u16(ip);
        self.set_register_value("ip".to_string(), ip + 2);
        instruction
    }
    
    fn execute(&mut self, instruction: u8) {
        use crate::instructions::*;
        
        match instruction {
            MOV_LIT_R1 => { // move two bytes to r1 | consumes 2 bytes
                let value = self.fetch_u16();
                self.set_register_value("r1".to_string(), value);
            },
            MOV_LIT_R2 => { // move two bytes to r2 | consumes 2 bytes
                let value = self.fetch_u16();
                self.set_register_value("r2".to_string(), value);
            },
            ADD_REG_REG => { // add two registers and write result to acc | consumes 2 bytes
                let (ra, rb) = byte_ops::u16_to_u8(self.fetch_u16());
                let (rav, rbv) = (self.registers.read_u16(2 * ra as u16), self.registers.read_u16(2 * rb as u16));
                self.set_register_value("acc".to_string(), rav + rbv);
            },
            _ => panic!("execute: {instruction}: no such instruction")
        }
    }
    
    pub fn step(&mut self) {
        let instruction = self.fetch_u8();
        self.execute(instruction);
    }
    
    pub fn debug(&self, names: &[String]) {
        for name in names {
            print!("{name}: {:04X}; ", self.get_register_value(name.to_string()));
        }
        println!();
    }
}


#[cfg(test)]
mod tests {
    use crate::instructions::*;
    use crate::memory::Memory;
    use crate::cpu::Cpu;
    
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
    fn basic_program() {
        let program = vec![
            MOV_LIT_R1, 0x12, 0x34,  // mov 0x1234, r1
            MOV_LIT_R2, 0xAB, 0xCD,  // mov 0xABCD, r2
            ADD_REG_REG, 0x02, 0x03, // add r1, r2
        ];
        
        let mut memory = Memory::new(0xFF);
        memory.write_bytes(0, program);
        
        let mut cpu = Cpu::new(memory);
        
        cpu.step();
        assert_eq!(0x1234, cpu.get_register_value("r1".to_string()));
        cpu.step();
        assert_eq!(0xABCD, cpu.get_register_value("r2".to_string()));
        cpu.step();
        assert_eq!(0xBE01, cpu.get_register_value("acc".to_string()));
    }
}
