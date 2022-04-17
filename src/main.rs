use std::env;

mod lib;

use lib::processor::*;
use lib::searcher::*;
use lib::bitmap::*;

fn main() {
    let search_words: Vec<String> = env::args().map(|s| String::from(s)).collect();
    let search_words: Vec<String> = search_words[2..].to_vec();

    let dict_words = get_dict_words();

    let search_bm = WordBitMapRow::from_words_and_dict(&search_words, &dict_words);

    let mut scores = score_files(&search_bm);
    
    scores = scores.into_iter().filter(|(_,s,_)| *s > 0).collect();
    scores.sort_by_cached_key(|(_, s, _)| *s);
    scores.reverse();

    for (name, matches, _) in scores {
        println!("{name} contains {matches} matches");
    }
}
