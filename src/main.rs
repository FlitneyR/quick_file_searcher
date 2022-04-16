mod searcher;
use searcher::processor::*;

fn main() {
    let dict_words = get_dict_words();

    for (name, file) in get_files() {
        let words = get_unique_words_from_file(&file);
        let bitmap = WordBitMapRow::from_words_and_dict(&words, &dict_words);

        let mut sum = 0;

        for index in 0..dict_words.len() {
            if bitmap.get_bit(index).unwrap() {
                sum += 1
            }
        }

        println!("Found {sum} known words in {name:?}");
    }
}
