use std::fs;
use std::io::Read;

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
pub fn get_unique_words_from_file(file: &fs::File) -> Vec<String> {
    let mut contents = String::new();
    let _ = file.clone().read_to_string(&mut contents);

    let mut words: Vec<String> = Vec::new();
    
    for word in contents.split_ascii_whitespace() {
        let word = String::from(word);
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
