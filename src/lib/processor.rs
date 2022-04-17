use std::fs;
use std::io::{Read, Write};

use crate::lib::bitmap::*;

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
/// 
/// Removes non-alphabetic characters from each word
pub fn get_unique_words_from_file(file: &fs::File) -> Vec<String> {
    let mut contents = String::new();
    let _ = file.clone().read_to_string(&mut contents);

    get_unique_words_from_string(&contents)
}

/// Returns the unique words in a string
/// 
/// The words in a string in the order they appear first
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
/// 
/// Trys to return .words but defaults to /usr/share/dict/words
pub fn get_dict_path() -> String {
    let mut contents = fs::read_dir("./")
        .expect("Couldn't read the contents of the current directory");
        
    if contents.find(|de| {
        de.as_ref()
            .unwrap().path()
            .to_str().unwrap() == "./.words"
    }).is_some() {
        String::from(".words")
    } else {
        String::from("/usr/share/dict/words")
    }
}

/// calculates the number of bytes needed to store a certain number of bits
pub fn number_of_bytes(bits: usize) -> usize {
    (bits + bits % 8) / 8
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
