mod searcher;
use searcher::processor::*;

fn main() {
    for (name, file) in get_files() {
        let words = get_unique_words_from_file(&file);
        println!("Words in {name}: {words:?}")
    }

    println!("Found {} words in {}", get_dict_words().len(), get_dict_path());
}
