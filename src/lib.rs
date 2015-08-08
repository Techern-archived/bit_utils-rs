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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_number_of_bits_u8() {
        assert_eq!(8, 1u8.number_of_bits());
        assert_eq!(8, 255u8.number_of_bits());
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

}