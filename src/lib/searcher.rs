use std::fs;
use itertools::Itertools;

use crate::*;
use crate::lib::bitmap::*;

/// Loads a cached .srch file
pub fn load_cache() -> Option<Vec<(String, WordsBitMap)>> {
    let buffer = fs::read(".srch");
    if buffer.is_err() { return None; }
    let buffer = buffer.unwrap();

    let mut sp = buffer.split(|b| *b == b'\n');

    let dict_path = String::from_utf8(sp.next().unwrap().to_vec()).unwrap();
    let file_names: Vec<&[u8]> = sp.take_while_ref(|l| **l != []).collect();

    let data = sp.nth(1).unwrap();

    let dict_words_count = get_dict_words_from(&dict_path)?.len();
    let file_names = file_names.iter().map(|l| {
        String::from_utf8(l.to_vec()).unwrap()
    });
    
    let bitmaps = data.chunks(number_of_bytes(dict_words_count));

    Some(file_names.zip(bitmaps).map(|(name, data)| {
        (name, WordsBitMap { bytes: data.to_vec() })
    }).collect())
}

/// Compares bitmaps for matching bits
/// 
/// Returns the number of matching bits and which bits matched
pub fn matches(bm1: &WordsBitMap, bm2: &WordsBitMap) -> (usize, WordsBitMap) {
    let bm_match = WordsBitMap::and(&bm1, &bm2);

    let mut sum: usize = 0;

    for index in 0..(bm_match.bytes.len() * 8) {
        sum += if bm_match.get_bit(index).unwrap() { 1 } else { 0 }
    }

    (sum, bm_match)
}

/// Scores files by matches with a bitmap
/// 
/// Returns a sorted vector of matches greater than one word
/// 
/// Each match contains
/// 
/// 1) the name of the matching file
/// 2) the number of matching words
/// 3) a bitmap to represent which words matched
pub fn score_files(search_bm: &WordsBitMap, cache: &Vec<(String, WordsBitMap)>) -> Vec<(String, usize, WordsBitMap)> {
    cache.iter().map(|(name, file_bm)| {
        let (matches, match_bm) = matches(&file_bm, &search_bm);
        (name.clone(), matches, match_bm)
    }).collect()
}
