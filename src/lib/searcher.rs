use savefile::prelude::*;

use crate::bitmap::*;

/// Loads a cached .srch file
pub fn load_cache() -> Option<Cache> {
    load_file(".srch", 0).ok()
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

impl Cache {
    pub fn score_files(&self, search_bm: &WordsBitMap) -> Vec<(String, usize, WordsBitMap)> {
        let cache = self.file_names.iter().zip(self.bitmaps.iter())
            .map(|(n, bm)| (n.clone(), bm.clone())).collect();

        score_files(search_bm, &cache)
    }
}
