use crate::byte_ops;

pub struct Memory {
    vec: Vec<u8>,
}

impl Memory {
    pub fn new(byte_size: u16) -> Self {
        Self {
            vec: vec![0; byte_size as usize]
        }
    }
    
    pub fn write_bytes(&mut self, start: usize, bytes: Vec<u8>) {
        self.vec[start..start + bytes.len()].copy_from_slice(&bytes);
    }
    
    pub fn read_u8(&self, index: u16) -> u8 {
        assert!(index < self.len(), "read_u8: {index}: memory unreachable");
        self.vec[index as usize]
    }
    
    pub fn read_u16(&self, index: u16) -> u16 {
        assert!(index < self.len() - 1, "read_u16: {index}: memory unreachable");
        byte_ops::u8_to_u16(self.vec[index as usize], self.vec[index as usize + 1])
    }
    
    pub fn write_u8(&mut self, index: u16, value: u8) {
        assert!(index < self.len(), "write_u8: {index}: memory unreachable");
        self.vec[index as usize] = value;
    }
    
    pub fn write_u16(&mut self, index: u16, value: u16) {
        assert!(index < self.len() - 1, "write_u16: {index}: memory unreachable");
        (self.vec[index as usize], self.vec[index as usize + 1]) = byte_ops::u16_to_u8(value);
    }
    
    pub fn len(&self) -> u16 {
        self.vec.len() as u16
    }
}

pub struct Registers {
    mem: Memory,
}

impl Registers {
    pub fn new(reg_count: u16) -> Self {
        Self {
            mem: Memory::new(2 * reg_count)
        }
    }
    
    pub fn read(&self, reg: u8) -> u16 {
        assert!((2 * reg as u16) < self.mem.len(), "read: {reg}: register unreachable");
        self.mem.read_u16(2 * reg as u16)
    }
    
    pub fn write(&mut self, reg: u8, val: u16) {
        assert!((2 * reg as u16) < self.mem.len(), "write: {reg}: register unreachable");
        self.mem.write_u16(2 * reg as u16, val)
    }
}
