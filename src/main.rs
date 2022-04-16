
mod searcher;
use std::io::Read;

use searcher::processor;

fn main() {
    for (name, mut file) in processor::get_files() {
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        println!("CONTENTS OF {name}:\n{contents}");
    }
}
