mod dict;
mod wordle;

use dict::Trie;
use rand::Rng;

fn main() {    
    let t: Box<Trie> = Box::new(Trie::create());
    Trie::insert(t, "bla");
    let boo = Trie::lookup(t, "bla");
    print!("{}", boo);
}
