use std::fs;
use std::io::Read;

/// Bitmap to record which words are used in which files
pub struct WordBitMapRow {
    bytes: Vec<u8>
}

impl WordBitMapRow {
    /// Creates a bit map row from a vector of input words and a vector of dictionary words
    pub fn from_words_and_dict(words: Vec<String>, dict: Vec<String>) -> WordBitMapRow {
        let bytes: Vec<u8> = vec![0 ; dict.len() / 8];
        let mut slf = WordBitMapRow { bytes: bytes };

        for word in words {
            dict.iter()
            .position(|w|
                *w == word
            ).and_then(|index|
                Some(slf.set_bit(index, true))
            );
        }

        slf
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

    let mut words: Vec<String> = Vec::new();
    
    for word in contents.split_ascii_whitespace() {
        let word = String::from(word)
            .chars().filter(|c|
                c.is_alphabetic()
            ).collect();
        
        if !words.contains(&word) {
            words.push(word);
        }
    }

    words
}

/// Returns a vector of each word in the dictionary file
pub fn get_dict_words() -> Vec<String> {
    let contents = fs::read_to_string(get_dict_path())
        .expect("Couldn't reach words file");
    
    contents.lines().map(String::from).collect()
}

/// Returns the path to the words file
pub fn get_dict_path() -> String {
    String::from("/usr/share/dict/words")
}
