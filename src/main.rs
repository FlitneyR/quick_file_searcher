use std::env;
use std::process::exit;

mod lib;

use lib::processor::*;
use lib::searcher::*;
use lib::bitmap::*;

fn main() {
    let search_words: Vec<String> = env::args().map(|s| String::from(s)).collect();
    let search_words = search_words[1..].to_vec();

    let dict_words = get_dict_words()
        .unwrap_or_else(|| {
            println!("Couldn't get access to dictionary file: {}", get_dict_path());
            exit(1);
        });

    let search_bm = WordsBitMap::from_words_and_dict(&search_words, &dict_words);

    let search_cache = load_cache() 
        .unwrap_or_else(|| {
            println!("Couldn't load a .srch cache file");
            println!("Attempting to create one");
            let temp = make_cache();

            if temp.is_none() {
                println!("Unable to create a .srch cache file");
                exit(1);
            }

            println!("Successfully created a .srch file");
            temp.unwrap()
        });

    let mut scores = search_cache.score_files(&search_bm)
        .into_iter().filter(|(_,s,_)| *s * 2 >= search_words.len())
        .collect::<Vec<(String, usize, WordsBitMap)>>();
    
    scores.sort_by_cached_key(|(_, s, _)| *s);
    scores.reverse();

    for (name, matches, _bm) in scores {
        println!("{name} contains {matches} matches");
    }
}
