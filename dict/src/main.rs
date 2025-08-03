
use std::collections::HashMap;


#[derive(Debug)]
struct WordDictionary {
    nodes: HashMap<char, WordDictionary>,
    terminal: bool,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        WordDictionary {
            nodes: HashMap::new(),
            terminal: false,
        }
    }

    fn add_word(&mut self, word: String) {
        let char_list = word.chars();
        let mut curr = self;
        for item in char_list {
            let next = curr.nodes.entry(item.clone()).or_insert(WordDictionary {
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
        // let next;

        for i in char_list {
            // false  +      => false
            
            if !(cur.nodes.contains_key(&i) || i == '.') {
                return false;
            }
            println!("{:?}{:?}", i, word);

            cur = & cur.nodes.get(&i).unwrap();
        }
        
 
        true
    }
}



fn main() {
    let mut wordDictionary = WordDictionary::new();
    wordDictionary.add_word("bad".to_string());
    wordDictionary.add_word("dad".to_string());
    wordDictionary.add_word("mad".to_string());
    let a = wordDictionary.search("pad".to_string()); // return False
    let b =wordDictionary.search("bad".to_string()); // return True
    let c=wordDictionary.search(".ad".to_string()); // return True
    let d =wordDictionary.search("b..".to_string()); // return True
    println!("Hello, world!");
    println!("{:?}{:?}{:?}{:?}", a,b,c,d);
}
