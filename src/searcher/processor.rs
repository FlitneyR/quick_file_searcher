use std::fs;
use std::io::{Read, Write};
use itertools::Itertools;

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
    /// Returns true if successful, false otherwise
    pub fn set_byte(&mut self, index: usize, val: u8) -> bool {
        if index >= self.bytes.len() { false } else {
            self.bytes[index] = val;
            true
        }
    }
}

/// Returns a vector of filenames in the current directory
pub fn get_paths() -> Vec<String> {
    fs::read_dir("./")
        .expect("Couldn't read file system")
        .filter(|dir_entry| {
            dir_entry.as_ref().and_then(|dir_entry| {
                let path = dir_entry.path();
                let meta = fs::metadata(path).unwrap();
                Ok(meta.is_file())
            }).unwrap_or(false)
        })
        .map(|de| {
            format!("{}", de.unwrap().path().display())
                .chars().skip(2).collect::<String>()
        })
        .filter(|path|
            path.chars().nth(0)
                .and_then(|c| Some(c != '.'))
                .unwrap_or(false)
        ).collect()
}

/// Returns a vector of pairs of file names and std::fs::File objects
pub fn get_files() -> Vec<(String, fs::File)> {
    get_paths().iter().map(|s| {
        (s, fs::File::open(s))
    }).filter(|(_, f)| {
        f.is_ok()
    }).map(|(name, f)| {
        (name.clone(), f.unwrap())
    }).collect()
}

/// Returns a vector of each unique word in a file
/// Removes non-alphabetic characters from each word
pub fn get_unique_words_from_file(file: &fs::File) -> Vec<String> {
    let mut contents = String::new();
    let _ = file.clone().read_to_string(&mut contents);

    get_unique_words_from_string(&contents)
}

/// Returns the unique words in a string
/// i.e. the words in a string in the order they appear first
pub fn get_unique_words_from_string(input: &String) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    
    for word in input.split_ascii_whitespace() {
        let word = filter(&String::from(word));
        
        if !words.contains(&word) {
            words.push(word);
        }
    }

    words
}

/// Removes non-alphabetic characters and moves everything to lower case
pub fn filter(input: &String) -> String {
    input
    .chars()
    .filter(|c| c.is_alphabetic())
    .map(|c| c.to_lowercase().to_string())
    .collect()
}

/// Returns a vector of each word in the dictionary file
pub fn get_dict_words() -> Vec<String> {
    let contents = fs::read_to_string(get_dict_path())
        .expect("Couldn't reach words file");
    
    contents.lines().map(String::from).collect()
}

/// Returns a vector of each word in the specified dictionary file
pub fn get_dict_words_from(path: &String) -> Vec<String> {
    let contents = fs::read_to_string(path)
        .expect("Couldn't reach words file");
    
    contents.lines().map(String::from).collect()
}

/// Returns the path to the words file
pub fn get_dict_path() -> String {
    String::from("/usr/share/dict/words")
}

/// calculates the number of bytes needed to store a certain number of bits
pub fn number_of_bytes(bits: usize) -> usize {
    (bits + bits % 8) / 8
}

/// Loads a cached .srch file
pub fn load_cache() -> Option<Vec<(String, WordBitMapRow)>> {
    let buffer = fs::read(".srch");
    if buffer.is_err() { return None; }
    let buffer = buffer.unwrap();

    let mut sp = buffer.split(|b| *b == b'\n');

    let dict_path = String::from_utf8(sp.next().unwrap().to_vec()).unwrap();
    let file_names: Vec<&[u8]> = sp.take_while_ref(|l| **l != []).collect();

    let data = sp.nth(1).unwrap();

    let dict_words_count = get_dict_words_from(&dict_path).len();
    let file_names = file_names.iter().map(|l| {
        String::from_utf8(l.to_vec()).unwrap()
    });

    println!("data: {} bytes", data.len());
    
    let bitmaps = data.chunks(number_of_bytes(dict_words_count));

    println!("{} bitmaps", bitmaps.len());

    Some(file_names.zip(bitmaps).map(|(name, data)| {
        (name, WordBitMapRow { bytes: data.to_vec() })
    }).collect())
}

/// Creates a .srch cache file in the current directory
pub fn make_cache() -> Option<Vec<(String, WordBitMapRow)>> {
    let dict_path = get_dict_path();
    let dict_words = get_dict_words();
    let mut srch_file = fs::File::create(".srch")
        .expect("Couldn't create .srch file");

    srch_file.write(dict_path.as_bytes())
        .expect("Unable to write dict file path to .srch");

    srch_file.write(&[b'\n'])
        .expect("Unable to write new line to .srch");
    
    let mut data: Vec<u8> = Vec::new();

    let mut output: Vec<(String, WordBitMapRow)> = Vec::new();

    for (file_name, file) in get_files() {
        srch_file.write(file_name.as_bytes())
            .expect("Unable to write to .srch file");
        
        srch_file.write(&[b'\n'])
            .expect("Unable to write new line to .srch");
        
        let file_words = get_unique_words_from_file(&file);
        let bitmap = WordBitMapRow::from_words_and_dict(&file_words, &dict_words);
        for byte in &bitmap.bytes {
            data.write(&[*byte])
                .expect(&*format!("Unable to write byte: {byte:?} to .srch file"));
        }

        output.push((file_name, bitmap));
    }

    srch_file.write(&[b'\n'])
        .expect("Unable to write new line to .srch");

    for byte in data {
        srch_file.write(&[byte])
            .expect(&*format!("Unable to write data byte: {byte}"));
    }

    Some(output)
}
