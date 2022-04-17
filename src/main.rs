mod searcher;
use searcher::processor::*;

fn main() {
    let cache1 = make_cache()
        .expect("Couldn't save a cache");

    let cache2 = load_cache()
        .expect("Couldn't load a cache");

    println!("cache1 length: {}, cache2 length: {}", cache1.len(), cache2.len());

    for ((name1, bm1), (name2, bm2)) in cache1.iter().zip(cache2.iter()) {
        println!("{name1}, {name2}");
        for (b1, b2) in bm1.bytes.iter().zip(bm2.bytes.iter()) {
            if b1 != b2 { println!("Mismatched bytes: {b1} and {b2}") }
        }
    }
}
