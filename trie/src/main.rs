use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    nodes: HashMap<char, Trie>,
    terminal: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            nodes: HashMap::new(),
            terminal: false,
        }
    }

    fn insert(&mut self, word: String) {
        let char_list = word.chars();
        let mut curr = self;
        for item in char_list {
            let next = curr.nodes.entry(item.clone()).or_insert(Trie {
                    nodes: HashMap::new(),
                    terminal: false,
            });
            curr = next;
        }
        curr.terminal = true;

    }

    fn search(&self, word: String) -> bool {
        let char_list = word.chars();
        let mut cur = self;
        let mut term= &false;
        // let next;

        for i in char_list {
            if !cur.nodes.contains_key(&i) {
                return false;
            }
            cur = & cur.nodes.get(&i).unwrap();
            term = & cur.terminal;
        }
        
 
        *term

    }

    fn starts_with(&self, prefix: String) -> bool {
        let char_list = prefix.chars();
        let mut cur = self;

        for i in char_list {
            if !cur.nodes.contains_key(&i) {
                return false;
            }
            cur = & cur.nodes.get(&i).unwrap();
        }
        
 
        true

    }
}

// use crate::List::{Cons, Nill};

fn main() {
    let s = "mouts".to_string();
    // let a: Vec<String> = s.chars().map(|x| x.to_string()).collect();
    let mut t = Trie::new();
    t.insert(s.clone());
    let s1 = "mous".to_string();
    t.insert("heya".to_string());

    println!("{:?}", t.search(s.clone()));
    println!("{:?}", t.search("heya".to_string()));
    println!("{:?}", t.starts_with("heyaa".to_string()));

}


// {"m": Trie { nodes: {"o": Trie { nodes: {"u": Trie { nodes: {"t": Trie { nodes: {"s": Trie { nodes: {}, terminal: true }}, terminal: true }}, terminal: true }}, terminal: true }}, terminal: true }}