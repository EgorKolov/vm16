use crate::byte_ops;

pub struct Memory {
    vec: Vec<u8>,
}

impl Memory {
    pub fn new(byte_size: usize) -> Self {
        Self {
            vec: vec![0; byte_size]
        }
    }
    
    pub fn write_bytes(&mut self, start: usize, bytes: Vec<u8>) {
        self.vec[start..start + bytes.len()].copy_from_slice(&bytes);
    }
    
    pub fn read_u8(&self, index: u16) -> u8 {
        assert!((index as usize) < self.vec.len(), "read_u8: {index}: memory unreachable");
        self.vec[index as usize]
    }
    
    pub fn read_u16(&self, index: u16) -> u16 {
        assert!((index as usize) + 1 < self.vec.len(), "read_u16: {index}: memory unreachable");
        byte_ops::u8_to_u16(self.vec[index as usize], self.vec[index as usize + 1])
    }
    
    pub fn write_u8(&mut self, index: u16, value: u8) {
        assert!((index as usize) < self.vec.len(), "write_u8: {index}: memory unreachable");
        self.vec[index as usize] = value;
    }
    
    pub fn write_u16(&mut self, index: u16, value: u16) {
        assert!((index as usize) + 1 < self.vec.len(), "write_u16: {index}: memory unreachable");
        (self.vec[index as usize], self.vec[index as usize + 1]) = byte_ops::u16_to_u8(value);
    }
}

