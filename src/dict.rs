use std::error::Error;
use rand::Rng;

pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    size: usize,
    sentinel: bool
}

impl Trie {
    pub fn create() -> Trie {
        const INIT: Option<Box<Trie>> = None;
        return Trie {
            children: [INIT; 26],
            size: 0,
            sentinel: false
        };
    }

    fn to_index(b: char) -> Result<usize, ()> {
        match b {
            'a'..='z' => Ok(b as usize - (b'a' as usize)),
            'A'..='Z' => Ok(b as usize - (b'A' as usize)),
            _ => Err(())
        }
    }

    pub fn lookup(mut t: &Trie, word: &str) -> Result<bool, ()> {
        for b in word.chars() {
            match &t.children[Trie::to_index(b)?] {
                Some(tt) => t = tt,
                None => return Ok(false),
            }
        }
        Ok(t.sentinel)
    }

    pub fn insert(mut t: &mut Trie, word: &str) -> Result<bool, ()> {
        if Trie::lookup(t, word)? {
            return Ok(false);
        }

        for b in word.chars() {
            t.size += 1;
            t = t.children[Trie::to_index(b)?].get_or_insert_with(|| Box::new(Self::create()));
        }
        t.size += 1;
        t.sentinel = true;
        Ok(true)
    }

    pub fn pick_nth(mut t: &Trie, mut nth : usize) -> Option<String> {
        assert!(nth < t.size);
        let mut res = String::new();
        'outer: loop {
            if nth == 0 && t.sentinel {
                return Some(res);
            }
            for (i,tt) in t.children.iter().enumerate() {
                match tt {
                    None => {}
                    Some (ttt) => {
                        if nth < ttt.size {
                            t = ttt;
                            res.push((TryInto::<u8>::try_into(i).unwrap() + b'a') as char);
                            continue 'outer;
                        } else {
                            nth -= ttt.size;
                        }
                    }
                }
            }
            return None
        }
    }


    pub fn pick_word(t: &Trie, k: u32) -> Option<String> {
        Trie::pick_nth(t, rand::thread_rng().gen_range(0..t.size))
    }

    fn read_dict_from_file(filename: &str, k: u32) -> Option<Trie> {
        return None;
    }
}