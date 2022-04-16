use std::fs;

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

pub fn get_files() -> Vec<(String, fs::File)> {
    get_paths().iter().map(|s| {
        (s, fs::File::open(s))
    }).filter(|(_, f)| {
        f.is_ok()
    }).map(|(name, f)| {
        (name.clone(), f.unwrap())
    }).collect()
}
