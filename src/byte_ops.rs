pub fn u8_to_u16(v1: u8, v2: u8) -> u16 {
    (v1 as u16) << 8 | v2 as u16
}

pub fn u16_to_u8(v: u16) -> (u8, u8) {
    (((v & 0xFF00) >> 8) as u8, (v & 0x00FF) as u8)
}

#[cfg(test)]
mod tests {
    use crate::byte_ops;
    
    #[test]
    fn u8_to_u16() {
        assert_eq!(byte_ops::u8_to_u16(0x00, 0xFF), 0x00FF);
        assert_eq!(byte_ops::u8_to_u16(0xFF, 0x00), 0xFF00);
        assert_eq!(byte_ops::u8_to_u16(0x12, 0x34), 0x1234);
    }
    
    #[test]
    fn u16_to_u8() {
        assert_eq!(byte_ops::u16_to_u8(0x00FF), (0x00, 0xFF));
        assert_eq!(byte_ops::u16_to_u8(0xFF00), (0xFF, 0x00));
        assert_eq!(byte_ops::u16_to_u8(0x1234), (0x12, 0x34));
    }
}