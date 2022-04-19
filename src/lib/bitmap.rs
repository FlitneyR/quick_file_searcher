use crate::*;

use savefile::prelude::*;
use savefile_derive::*;

/// Bitmap to record which words are used in which files
#[derive(Clone)]
pub struct WordsBitMap {
    pub bytes: Vec<u8>,
}

impl WithSchema for WordsBitMap {
    fn schema(_version: u32) -> Schema {
        Schema::Vector(Box::new(
            Schema::Primitive(
                SchemaPrimitive::schema_u8
            )
        ))
    }
}

impl Serialize for WordsBitMap {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SavefileError> {
        self.bytes.serialize(serializer)
    }
}

impl Deserialize for WordsBitMap {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SavefileError> {
        Ok(WordsBitMap {
            bytes: Vec::<u8>::deserialize(deserializer)?
        })
    }
}

impl Introspect for WordsBitMap {
    fn introspect_value(&self) -> String {
        format!("WordsBitMap of {} bits", self.bytes.len() * 8)
    }

    fn introspect_child<'a>(&'a self, _index: usize) -> Option<Box<dyn IntrospectItem<'a> + 'a>> {
        None
    }
}

#[derive(Savefile)]
pub struct Cache {
    pub dict_path: String,
    pub file_names: Vec<String>,
    pub bitmaps: Vec<WordsBitMap>,
}

impl WordsBitMap {
    /// Creates a bit map row from a vector of input words and a vector of dictionary words
    pub fn from_words_and_dict(words: &Vec<String>, dict: &Vec<String>) -> WordsBitMap {
        let bytes_num = number_of_bytes(dict.len());
        let bytes: Vec<u8> = vec![0 ; bytes_num];
        let mut bm = WordsBitMap { bytes: bytes };

        for word in words.iter().map(|w| filter(w)) {
            dict.iter()
            .position(|w|
                *w == word
            ).and_then(|index| Some(
                bm.set_bit(index, true)
            ));
        }

        bm
    }

    /// Performs a bitwise and on each byte in the bytes vector
    pub fn and(bm1: &WordsBitMap, bm2: &WordsBitMap) -> WordsBitMap {
        WordsBitMap {
            bytes: bm1.bytes.iter().zip(bm2.bytes.iter())
                .map(|(b1, b2)| b1 & b2).collect()
        }
    }

    /// Gets the value of a bit in the bitmap
    pub fn get_bit(&self, index: usize) -> Option<bool> {
        if index >= self.bytes.len() * 8 { return None }

        let (byte, bit) = bit_address(index);

        self
        .get_byte(byte)
        .and_then(|byte|
            Some(byte & (1 << bit) > 0)
        )
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

pub fn bit_address(index: usize) -> (usize, u8) {
    (index / 8, index as u8 % 8)
}