pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    weight: u32,
}

impl Trie {

    pub fn create() -> Trie {
        const INIT : Option<Box<Trie>> = None;
        return Trie {
            children: [INIT; 26],
            weight: 0,
        }
    }

    pub fn insert(t: Box<Trie>, word: &str) {
        if Self::lookup(&t, word) {
            return;
        }

        let mut temp = &t;

        // the horror
        let mut it = word.as_bytes().iter().peekable();

        while let Some(b) = it.next() {
            temp.weight += 1;
            temp = match &temp.children[*b as usize] {
                None => {
                    let new_node = Box::new(Self::create());
                    temp.children[*b as usize] = Some(new_node);
                    &new_node
                },
                Some(t) => t,
            };

            // Weights are very brittle in this implementation.
            if it.peek().is_none() {
                temp.weight += 1;
            }
        }
    }

    pub fn lookup(t: &Box<Trie>, word: &str) -> bool {
        let mut temp = t;
        for b in word.as_bytes() {
            if let Some(t) = temp.children[*b as usize] {
                temp = &t;
            } else {
                return false;
            }
        }
        return true;
    }

    fn pick_child(t: &Trie) -> Option<usize> {
        let rnd: f64 = (t.weight as f64) * rand::random::<f64>();
        let mut sum: f64 = 0.0;
        for i in 0..26 {
            if let Some(t) = &t.children[i] {
                sum += t.weight as f64;
                if rnd < sum {return Some(i)};
            }
        }
        return None;
    }

    fn pick_word(t: Trie, k: u32) -> Option<String> {
        let mut word = String::new();
        let mut temp = &t;
        for _ in 1..k {
            match Self::pick_child(temp) {
                None => return None,
                Some(index) => {
                    word.push((index as u8) as char);
                    if let Some(t) = &temp.children[index] {
                        temp = t;
                    } else {
                        panic!("Child found during pick_child but not during pick_word.")
                    }
                }
            }            
        }
        return Some(word);
    }

    fn read_dict_from_file(filename : &str, k: u32) -> Option<Trie> {
        return None;
    }
}