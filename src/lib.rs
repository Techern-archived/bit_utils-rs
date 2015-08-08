//! Bit utilities for Rust
//!
//! This will eventually expand to include Nibbles and packing them into bytes and integers.

pub trait BitInformation {
    
    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize;
    
    /// Checks to see if bit X is set
    fn has_x_bit(&self, position: usize) -> bool;
    
    /// Checks to see if the requested bit position is in bounds
    fn is_bit_in_bounds(&self, position: usize) -> bool {
        position <= self.number_of_bits()
    }
    
    /// Checks to see if the most signifigant bit is set
    fn has_most_signifigant_bit(&self) -> bool {
        self.has_x_bit(self.number_of_bits() - 1) //8 bits? position 7.
    }
    
}

impl BitInformation for u8 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        8
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b00000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for i8 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        8
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b00000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for u16 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        16
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for i16 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        16
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for u32 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        32
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b00000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for i32 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        32
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b00000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for u64 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        64
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000000000000000000000000000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for i64 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        64
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000000000000000000000000000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for usize {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        (self.count_ones() + self.count_zeros()) as usize
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000000000000000000000000000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for isize {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        (self.count_ones() + self.count_zeros()) as usize
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000000000000000000000000000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

#[allow(overflowing_literals)] //Just for tests, don't do this in real life, kids!
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_number_of_bits_i8() {
        assert_eq!(8, 1i8.number_of_bits());
    }
    
    #[test]
    fn test_most_signifigant_bit_i8() {
        assert!(1i8.has_most_signifigant_bit() == false);
        assert!(0b10101010i8.has_most_signifigant_bit() == true);
        assert!(0b10111000i8.has_most_signifigant_bit() == true);
    }
    
    #[test]
    fn test_bit_in_bounds_i8() {
        assert!(25i8.is_bit_in_bounds(1));
        assert!(25i8.is_bit_in_bounds(9) == false);
    }
    
    #[test]
    fn test_has_x_bit_i8() {
        let num: i8 = 0b10101010i8;
        
        assert!(num.has_x_bit(0) == false);
        assert!(num.has_x_bit(1) == true);
        assert!(num.has_x_bit(2) == false);
        assert!(num.has_x_bit(3) == true);
        assert!(num.has_x_bit(4) == false);
        assert!(num.has_x_bit(5) == true);
        assert!(num.has_x_bit(6) == false);
        assert!(num.has_x_bit(7) == true);
    }

    #[test]
    fn test_number_of_bits_u8() {
        assert_eq!(8, 1u8.number_of_bits());
    }
    
    #[test]
    fn test_most_signifigant_bit_u8() {
        assert!(1u8.has_most_signifigant_bit() == false);
        assert!(0b10101010u8.has_most_signifigant_bit() == true);
        assert!(0b10111000u8.has_most_signifigant_bit() == true);
    }
    
    #[test]
    fn test_bit_in_bounds_u8() {
        assert!(25u8.is_bit_in_bounds(1));
        assert!(25u8.is_bit_in_bounds(9) == false);
    }
    
    #[test]
    fn test_has_x_bit_u8() {
        let num: u8 = 0b10101010u8;
        
        assert!(num.has_x_bit(0) == false);
        assert!(num.has_x_bit(1) == true);
        assert!(num.has_x_bit(2) == false);
        assert!(num.has_x_bit(3) == true);
        assert!(num.has_x_bit(4) == false);
        assert!(num.has_x_bit(5) == true);
        assert!(num.has_x_bit(6) == false);
        assert!(num.has_x_bit(7) == true);
    }

    #[test]
    fn test_number_of_bits_u16() {
        assert_eq!(16, 1u16.number_of_bits());
    }
    
    #[test]
    fn test_most_signifigant_bit_u16() {
        assert!(1u16.has_most_signifigant_bit() == false);
        assert!(0b1010101010101010u16.has_most_signifigant_bit() == true);
        assert!(0b1011100000000000u16.has_most_signifigant_bit() == true);
    }
    
    #[test]
    fn test_bit_in_bounds_u16() {
        assert!(25u16.is_bit_in_bounds(12));
        assert!(25u16.is_bit_in_bounds(18) == false);
    }
    
    #[test]
    fn test_has_x_bit_u16() {
        let num: u16 = 0b1010101010101010u16;
        
        assert!(num.has_x_bit(0) == false);
        assert!(num.has_x_bit(1) == true);
        assert!(num.has_x_bit(2) == false);
        assert!(num.has_x_bit(3) == true);
        assert!(num.has_x_bit(4) == false);
        assert!(num.has_x_bit(5) == true);
        assert!(num.has_x_bit(6) == false);
        assert!(num.has_x_bit(7) == true);
        assert!(num.has_x_bit(8) == false);
        assert!(num.has_x_bit(9) == true);
        assert!(num.has_x_bit(10) == false);
        assert!(num.has_x_bit(11) == true);
        assert!(num.has_x_bit(12) == false);
        assert!(num.has_x_bit(13) == true);
        assert!(num.has_x_bit(14) == false);
        assert!(num.has_x_bit(15) == true);
    }

    #[test]
    fn test_number_of_bits_i16() {
        assert_eq!(16, 1i16.number_of_bits());
    }
    
    #[test]
    fn test_most_signifigant_bit_i16() {
        assert!(1i16.has_most_signifigant_bit() == false);
        assert!(0b1010101010101010i16.has_most_signifigant_bit() == true);
        assert!(0b1011100000000000i16.has_most_signifigant_bit() == true);
    }
    
    #[test]
    fn test_bit_in_bounds_i16() {
        assert!(25i16.is_bit_in_bounds(12));
        assert!(25i16.is_bit_in_bounds(18) == false);
    }
    
    #[test]
    fn test_has_x_bit_i16() {
        let num: i16 = 0b1010101010101010i16;
        
        assert!(num.has_x_bit(0) == false);
        assert!(num.has_x_bit(1) == true);
        assert!(num.has_x_bit(2) == false);
        assert!(num.has_x_bit(3) == true);
        assert!(num.has_x_bit(4) == false);
        assert!(num.has_x_bit(5) == true);
        assert!(num.has_x_bit(6) == false);
        assert!(num.has_x_bit(7) == true);
        assert!(num.has_x_bit(8) == false);
        assert!(num.has_x_bit(9) == true);
        assert!(num.has_x_bit(10) == false);
        assert!(num.has_x_bit(11) == true);
        assert!(num.has_x_bit(12) == false);
        assert!(num.has_x_bit(13) == true);
        assert!(num.has_x_bit(14) == false);
        assert!(num.has_x_bit(15) == true);
    }

    #[test]
    fn test_number_of_bits_u32() {
        assert_eq!(32, 1u32.number_of_bits());
    }
    
    #[test]
    fn test_most_signifigant_bit_u32() {
        assert!(1u32.has_most_signifigant_bit() == false);
        assert!(0b10101010101010101010101010101010u32.has_most_signifigant_bit() == true);
        assert!(0b10111000000000000000000000000000u32.has_most_signifigant_bit() == true);
    }
    
    #[test]
    fn test_bit_in_bounds_u32() {
        assert!(25u32.is_bit_in_bounds(30));
        assert!(25u32.is_bit_in_bounds(57) == false);
    }
    
    #[test]
    fn test_has_x_bit_u32() {
        let num: u32 = 0b10101010101010101010101010101010u32;
        
        assert!(num.has_x_bit(0) == false);
        assert!(num.has_x_bit(1) == true);
        assert!(num.has_x_bit(2) == false);
        assert!(num.has_x_bit(3) == true);
        assert!(num.has_x_bit(4) == false);
        assert!(num.has_x_bit(5) == true);
        assert!(num.has_x_bit(6) == false);
        assert!(num.has_x_bit(7) == true);
        assert!(num.has_x_bit(8) == false);
        assert!(num.has_x_bit(9) == true);
        assert!(num.has_x_bit(10) == false);
        assert!(num.has_x_bit(11) == true);
        assert!(num.has_x_bit(12) == false);
        assert!(num.has_x_bit(13) == true);
        assert!(num.has_x_bit(14) == false);
        assert!(num.has_x_bit(15) == true);
        assert!(num.has_x_bit(16) == false);
        assert!(num.has_x_bit(17) == true);
        assert!(num.has_x_bit(18) == false);
        assert!(num.has_x_bit(19) == true);
        assert!(num.has_x_bit(20) == false);
        assert!(num.has_x_bit(21) == true);
        assert!(num.has_x_bit(22) == false);
        assert!(num.has_x_bit(23) == true);
        assert!(num.has_x_bit(24) == false);
        assert!(num.has_x_bit(25) == true);
        assert!(num.has_x_bit(26) == false);
        assert!(num.has_x_bit(27) == true);
        assert!(num.has_x_bit(28) == false);
        assert!(num.has_x_bit(29) == true);
        assert!(num.has_x_bit(30) == false);
        assert!(num.has_x_bit(31) == true);
    }

    #[test]
    fn test_number_of_bits_i32() {
        assert_eq!(32, 1i32.number_of_bits());
    }
    
    #[test]
    fn test_most_signifigant_bit_i32() {
        assert!(1i32.has_most_signifigant_bit() == false);
        assert!(0b10101010101010101010101010101010i32.has_most_signifigant_bit() == true);
        assert!(0b10111000000000000000000000000000i32.has_most_signifigant_bit() == true);
    }
    
    #[test]
    fn test_bit_in_bounds_i32() {
        assert!(25i32.is_bit_in_bounds(30));
        assert!(25i32.is_bit_in_bounds(57) == false);
    }
    
    #[test]
    fn test_has_x_bit_i32() {
        let num: i32 = 0b10101010101010101010101010101010i32;
        
        assert!(num.has_x_bit(0) == false);
        assert!(num.has_x_bit(1) == true);
        assert!(num.has_x_bit(2) == false);
        assert!(num.has_x_bit(3) == true);
        assert!(num.has_x_bit(4) == false);
        assert!(num.has_x_bit(5) == true);
        assert!(num.has_x_bit(6) == false);
        assert!(num.has_x_bit(7) == true);
        assert!(num.has_x_bit(8) == false);
        assert!(num.has_x_bit(9) == true);
        assert!(num.has_x_bit(10) == false);
        assert!(num.has_x_bit(11) == true);
        assert!(num.has_x_bit(12) == false);
        assert!(num.has_x_bit(13) == true);
        assert!(num.has_x_bit(14) == false);
        assert!(num.has_x_bit(15) == true);
        assert!(num.has_x_bit(16) == false);
        assert!(num.has_x_bit(17) == true);
        assert!(num.has_x_bit(18) == false);
        assert!(num.has_x_bit(19) == true);
        assert!(num.has_x_bit(20) == false);
        assert!(num.has_x_bit(21) == true);
        assert!(num.has_x_bit(22) == false);
        assert!(num.has_x_bit(23) == true);
        assert!(num.has_x_bit(24) == false);
        assert!(num.has_x_bit(25) == true);
        assert!(num.has_x_bit(26) == false);
        assert!(num.has_x_bit(27) == true);
        assert!(num.has_x_bit(28) == false);
        assert!(num.has_x_bit(29) == true);
        assert!(num.has_x_bit(30) == false);
        assert!(num.has_x_bit(31) == true);
    }

    #[test]
    fn test_number_of_bits_u64() {
        assert_eq!(64, 1u64.number_of_bits());
    }
    
    #[test]
    fn test_most_signifigant_bit_u64() {
        assert!(1u64.has_most_signifigant_bit() == false);
        assert!(0b1010101010101010101010101010101010101010101010101010101010101010u64.has_most_signifigant_bit() == true);
        assert!(0b1011100000000000000000000000000000000000000000000000000000000000u64.has_most_signifigant_bit() == true);
    }
    
    #[test]
    fn test_bit_in_bounds_u64() {
        assert!(25u64.is_bit_in_bounds(60));
        assert!(25u64.is_bit_in_bounds(69) == false);
    }
    
    #[test]
    fn test_has_x_bit_u64() {
        let num: u64 = 0b1010101010101010101010101010101010101010101010101010101010101010u64;
        
        assert!(num.has_x_bit(0) == false);
        assert!(num.has_x_bit(1) == true);
        assert!(num.has_x_bit(2) == false);
        assert!(num.has_x_bit(3) == true);
        assert!(num.has_x_bit(4) == false);
        assert!(num.has_x_bit(5) == true);
        assert!(num.has_x_bit(6) == false);
        assert!(num.has_x_bit(7) == true);
        assert!(num.has_x_bit(8) == false);
        assert!(num.has_x_bit(9) == true);
        assert!(num.has_x_bit(10) == false);
        assert!(num.has_x_bit(11) == true);
        assert!(num.has_x_bit(12) == false);
        assert!(num.has_x_bit(13) == true);
        assert!(num.has_x_bit(14) == false);
        assert!(num.has_x_bit(15) == true);
        assert!(num.has_x_bit(16) == false);
        assert!(num.has_x_bit(17) == true);
        assert!(num.has_x_bit(18) == false);
        assert!(num.has_x_bit(19) == true);
        assert!(num.has_x_bit(20) == false);
        assert!(num.has_x_bit(21) == true);
        assert!(num.has_x_bit(22) == false);
        assert!(num.has_x_bit(23) == true);
        assert!(num.has_x_bit(24) == false);
        assert!(num.has_x_bit(25) == true);
        assert!(num.has_x_bit(26) == false);
        assert!(num.has_x_bit(27) == true);
        assert!(num.has_x_bit(28) == false);
        assert!(num.has_x_bit(29) == true);
        assert!(num.has_x_bit(30) == false);
        assert!(num.has_x_bit(31) == true);
        assert!(num.has_x_bit(32) == false);
        assert!(num.has_x_bit(33) == true);
        assert!(num.has_x_bit(34) == false);
        assert!(num.has_x_bit(35) == true);
        assert!(num.has_x_bit(36) == false);
        assert!(num.has_x_bit(37) == true);
        assert!(num.has_x_bit(38) == false);
        assert!(num.has_x_bit(39) == true);
        assert!(num.has_x_bit(40) == false);
        assert!(num.has_x_bit(41) == true);
        assert!(num.has_x_bit(42) == false);
        assert!(num.has_x_bit(43) == true);
        assert!(num.has_x_bit(44) == false);
        assert!(num.has_x_bit(45) == true);
        assert!(num.has_x_bit(46) == false);
        assert!(num.has_x_bit(47) == true);
        assert!(num.has_x_bit(48) == false);
        assert!(num.has_x_bit(49) == true);
        assert!(num.has_x_bit(50) == false);
        assert!(num.has_x_bit(51) == true);
        assert!(num.has_x_bit(52) == false);
        assert!(num.has_x_bit(53) == true);
        assert!(num.has_x_bit(54) == false);
        assert!(num.has_x_bit(55) == true);
        assert!(num.has_x_bit(56) == false);
        assert!(num.has_x_bit(57) == true);
        assert!(num.has_x_bit(58) == false);
        assert!(num.has_x_bit(59) == true);
        assert!(num.has_x_bit(60) == false);
        assert!(num.has_x_bit(61) == true);
        assert!(num.has_x_bit(62) == false);
        assert!(num.has_x_bit(63) == true);
    }

    #[test]
    fn test_number_of_bits_i64() {
        assert_eq!(64, 1i64.number_of_bits());
    }
    
    #[test]
    fn test_most_signifigant_bit_i64() {
        assert!(1i64.has_most_signifigant_bit() == false);
        assert!(0b1010101010101010101010101010101010101010101010101010101010101010i64.has_most_signifigant_bit() == true);
        assert!(0b1011100000000000000000000000000000000000000000000000000000000000i64.has_most_signifigant_bit() == true);
    }
    
    #[test]
    fn test_bit_in_bounds_i64() {
        assert!(25i64.is_bit_in_bounds(60));
        assert!(25i64.is_bit_in_bounds(69) == false);
    }
    
    #[test]
    fn test_has_x_bit_i64() {
        let num: i64 = 0b1010101010101010101010101010101010101010101010101010101010101010i64;
        
        assert!(num.has_x_bit(0) == false);
        assert!(num.has_x_bit(1) == true);
        assert!(num.has_x_bit(2) == false);
        assert!(num.has_x_bit(3) == true);
        assert!(num.has_x_bit(4) == false);
        assert!(num.has_x_bit(5) == true);
        assert!(num.has_x_bit(6) == false);
        assert!(num.has_x_bit(7) == true);
        assert!(num.has_x_bit(8) == false);
        assert!(num.has_x_bit(9) == true);
        assert!(num.has_x_bit(10) == false);
        assert!(num.has_x_bit(11) == true);
        assert!(num.has_x_bit(12) == false);
        assert!(num.has_x_bit(13) == true);
        assert!(num.has_x_bit(14) == false);
        assert!(num.has_x_bit(15) == true);
        assert!(num.has_x_bit(16) == false);
        assert!(num.has_x_bit(17) == true);
        assert!(num.has_x_bit(18) == false);
        assert!(num.has_x_bit(19) == true);
        assert!(num.has_x_bit(20) == false);
        assert!(num.has_x_bit(21) == true);
        assert!(num.has_x_bit(22) == false);
        assert!(num.has_x_bit(23) == true);
        assert!(num.has_x_bit(24) == false);
        assert!(num.has_x_bit(25) == true);
        assert!(num.has_x_bit(26) == false);
        assert!(num.has_x_bit(27) == true);
        assert!(num.has_x_bit(28) == false);
        assert!(num.has_x_bit(29) == true);
        assert!(num.has_x_bit(30) == false);
        assert!(num.has_x_bit(31) == true);
        assert!(num.has_x_bit(32) == false);
        assert!(num.has_x_bit(33) == true);
        assert!(num.has_x_bit(34) == false);
        assert!(num.has_x_bit(35) == true);
        assert!(num.has_x_bit(36) == false);
        assert!(num.has_x_bit(37) == true);
        assert!(num.has_x_bit(38) == false);
        assert!(num.has_x_bit(39) == true);
        assert!(num.has_x_bit(40) == false);
        assert!(num.has_x_bit(41) == true);
        assert!(num.has_x_bit(42) == false);
        assert!(num.has_x_bit(43) == true);
        assert!(num.has_x_bit(44) == false);
        assert!(num.has_x_bit(45) == true);
        assert!(num.has_x_bit(46) == false);
        assert!(num.has_x_bit(47) == true);
        assert!(num.has_x_bit(48) == false);
        assert!(num.has_x_bit(49) == true);
        assert!(num.has_x_bit(50) == false);
        assert!(num.has_x_bit(51) == true);
        assert!(num.has_x_bit(52) == false);
        assert!(num.has_x_bit(53) == true);
        assert!(num.has_x_bit(54) == false);
        assert!(num.has_x_bit(55) == true);
        assert!(num.has_x_bit(56) == false);
        assert!(num.has_x_bit(57) == true);
        assert!(num.has_x_bit(58) == false);
        assert!(num.has_x_bit(59) == true);
        assert!(num.has_x_bit(60) == false);
        assert!(num.has_x_bit(61) == true);
        assert!(num.has_x_bit(62) == false);
        assert!(num.has_x_bit(63) == true);
    }

}