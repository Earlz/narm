
//argument masks
pub const MASK_R3_R3:u16        = 0b0000_0000_0011_1111;
pub const MASK_R3_R3_R3:u16     = 0b0000_0001_1111_1111;
pub const MASK_D1_R4_R3:u16     = 0b0000_0000_1111_1111;
pub const MASK_R3_IMM8:u16      = 0b0000_0111_1111_1111;
pub const MASK_IMM7:u16         = 0b0000_0000_0111_1111;
pub const MASK_IMM8:u16         = 0b0000_0000_1111_1111;
pub const MASK_IMM5_R3_R3:u16   = 0b0000_0111_1111_1111;
pub const MASK_C4_IMM8:u16      = 0b0000_1111_1111_1111;
pub const MASK_R4_Q3:u16        = 0b0000_0000_0111_1111;
pub const MASK_X1_RL8:u16       = 0b0000_0001_1111_1111;
pub const MASK_NONE:u16         = 0b0000_0000_0000_0000;

/// This specifies a register beyond r0-r7
/// It is not strictly necessary to be organized like this, but used to prevent programmer errors
pub struct LongRegister{
    pub register: u32
}


pub fn is_32bit_opcode(opcode: u16) -> bool{
    if (opcode & 0b1110_0000_0000_0000) != 0b1110_0000_0000_0000 {
        return false;
    }
    opcode & 0b0001_1000_0000_0000 != 0
}


pub fn decode_r3_r3(opcode: u16) -> (usize, usize){
    (((opcode & 0b0000_0000_0011_1000) >> 3) as usize,
        (opcode & 0b0000_0000_0000_0111) as usize)
}

pub fn decode_r3_imm8(opcode: u16) -> (usize, u8){
    (((opcode & 0b0000_0111_0000_0000) >> 8) as usize,
        (opcode & 0b0000_0000_1111_1111) as u8)
}


