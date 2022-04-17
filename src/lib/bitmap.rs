use crate::*;

/// Bitmap to record which words are used in which files
#[derive(Clone)]
pub struct WordBitMapRow {
    pub bytes: Vec<u8>,
}

impl WordBitMapRow {
    /// Creates a bit map row from a vector of input words and a vector of dictionary words
    pub fn from_words_and_dict(words: &Vec<String>, dict: &Vec<String>) -> WordBitMapRow {
        let bites_num = number_of_bytes(dict.len());
        let bytes: Vec<u8> = vec![0 ; bites_num];
        let mut slf = WordBitMapRow { bytes: bytes };

        for word in words {
            dict.iter()
            .position(|w|
                w == word
            ).and_then(|index|
                Some(slf.set_bit(index, true))
            );
        }

        slf
    }

    /// Performs a bitwise and on each byte in the bytes vector
    pub fn and(bm1: &WordBitMapRow, bm2: &WordBitMapRow) -> WordBitMapRow {
        WordBitMapRow {
            bytes: bm1.bytes.iter().zip(bm2.bytes.iter())
                .map(|(b1, b2)| b1 & b2).collect()
        }
    }

    /// Gets the value of a bit in the bitmap
    pub fn get_bit(&self, index: usize) -> Option<bool> {
        if index >= self.bytes.len() * 8 { None } else {
            let bit = index % 8;

            self
            .get_byte(index / 8)
            .and_then(|byte|
                Some(byte & (1 << bit) > 0)
            )
        }
    }

    /// Sets the value of a bit in the bitmap
    /// 
    /// Returns true if successful, otherwise false
    pub fn set_bit(&mut self, index: usize, val: bool) -> bool {
        let byte_index = index / 8;
        let bit = index % 8;

        let value = if !val { 0 } else { 1 << bit };
        let mask = !value;

        let old_byte = self.get_byte(byte_index);
        let new_byte = old_byte.and_then(|byte| Some((byte & mask) | value));
        new_byte.and_then(|byte|
            Some(self.set_byte(byte_index, byte))
        ).unwrap_or(false)
    }

    /// Returns some byte at the specified index, if it exists, otherwise none
    pub fn get_byte(&self, index: usize) -> Option<&u8> {
        self.bytes.get(index)
    }

    /// Sets a byte
    /// 
    /// Returns true if successful, false otherwise
    pub fn set_byte(&mut self, index: usize, val: u8) -> bool {
        if index >= self.bytes.len() { false } else {
            self.bytes[index] = val;
            true
        }
    }
}