use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub struct Bee{
    source: Option<String>,
    words: HashMap<String, u32>,
    letters: String
}

impl Bee{
    pub fn new_path(path: String)-> Bee{
        let mut words: HashMap<String, u32> = HashMap::new();
        {//Path borrow
            let file = Path::new(&path);
            let mut open_file = File::open(file).unwrap();
            let mut buffer = String::new();
            open_file.read_to_string(&mut buffer).unwrap();
            for word_str in buffer.split_whitespace(){
                let word = word_str.to_string();

                let count = match words.get(&word){
                    Some(count) =>{
                        count + 1
                    }
                    _ => {0}
                };
                words.insert(word, count);
            }
        }
        let letters = String::new();
        Bee{words: words, source: Some(path), letters: letters}
    }
    pub fn word_count(&self, word: &str) -> Option<&u32>{
        self.words.get(word)
    }
    //pub fn suggestions(&self, bad_word: &str)-> Vec<&str>{

    //}
}
#[cfg(test)]
mod tests {
    use super::Bee;
    #[test]
    fn it_works() {
        let bee = Bee::new_path(String::from("example/big.txt"));
        println!("{:?}", bee.word_count("banana"));
    }
}
