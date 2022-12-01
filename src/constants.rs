pub const IP: u8 = 0x00;
pub const ACC: u8 = 0x01;
pub const R1: u8 = 0x02;
pub const R2: u8 = 0x03;
pub const R3: u8 = 0x04;
pub const R4: u8 = 0x05;
pub const R5: u8 = 0x06;
pub const R6: u8 = 0x07;
pub const R7: u8 = 0x08;
pub const R8: u8 = 0x09;
pub const SP: u8 = 0x0A;
pub const FP: u8 = 0x0B;

pub const MOV_LIT_REG: u8 = 0x10;
pub const MOV_REG_REG: u8 = 0x11;
pub const MOV_REG_MEM: u8 = 0x12;
pub const MOV_MEM_REG: u8 = 0x13;

pub const ADD_REG_REG: u8 = 0x14;

pub const JMP_NOT_EQ: u8  = 0x15;

pub const PSH_LIT: u8     = 0x17;
pub const PSH_REG: u8     = 0x18;
pub const POP: u8         = 0x1A;

pub const CAL_LIT: u8     = 0x5E;
pub const CAL_REG: u8     = 0x5F;
pub const RET: u8         = 0x60;

pub const HALT: u8 = 0xFF;
