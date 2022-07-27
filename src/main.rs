mod dict;
mod wordle;

use dict::Trie;
use rand::Rng;

fn main() {    
    let mut t: Box<Trie> = Box::new(Trie::create());
    Trie::insert(&mut *t, "blar").expect("Invalid key");
    Trie::insert(&mut *t, "blub").expect("Invalid key");
    Trie::insert(&mut *t, "bluc").expect("Invalid key");
    Trie::insert(&mut *t, "blud").expect("Invalid key");
    Trie::insert(&mut *t, "blue").expect("Invalid key");
    let boo = Trie::lookup(&*t, "blar").expect("Invalid key");
    print!("{}\n", boo);
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
    print!("{}\n", Trie::pick_word(&*t, 4).expect("Could not find word"));
}
