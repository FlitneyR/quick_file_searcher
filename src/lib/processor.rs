use std::fs;
use std::io::Read;

use savefile::prelude::*;

use crate::bitmap::*;

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

    for word in input.split_whitespace() {
        let word = filter(&String::from(word));
        
        if !words.contains(&word) {
            words.push(word);
        }
    }

    words
}

/// Removes non-alphabetic characters and moves everything to lower case
/// 
/// ```
/// # use lib::processor::filter;
/// 
/// let input_string = "Let's".to_string();
/// let expected_string = "lets".to_string();
/// let output_string = filter(&input_string);
/// assert_eq!(expected_string, output_string);
/// ```
pub fn filter(input: &String) -> String {
    input.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().to_string())
        .collect()
}

/// Returns a vector of each word in the dictionary file
pub fn get_dict_words() -> Option<Vec<String>> {
    get_dict_words_from(&get_dict_path())
}

/// Returns a vector of each word in the specified dictionary file
pub fn get_dict_words_from(path: &String) -> Option<Vec<String>> {
    fs::read_to_string(path).ok()
        .and_then(|file| Some(
            file.lines().map(String::from).collect()
        ))
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
pub fn make_cache() -> Option<Cache> {
    let dict_path = get_dict_path();
    let dict_words = get_dict_words_from(&dict_path)?;

    let file_names = get_paths();

    let bitmaps = file_names.iter().map(|file_name| {
        fs::read_to_string(file_name).ok().and_then(|file_contents| {
            let file_words = get_unique_words_from_string(&file_contents);
            Some(WordsBitMap::from_words_and_dict(&file_words, &dict_words))
        })
    }).filter(|bm| bm.is_some())
    .map(|bm| bm.unwrap()).collect();

    let cache = Cache {
        dict_path: dict_path,
        file_names: file_names,
        bitmaps: bitmaps,
    };

    save_file(".srch", 0, &cache).unwrap();

    Some(cache)
}
