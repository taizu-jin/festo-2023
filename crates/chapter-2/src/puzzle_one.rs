#![allow(dead_code)]

use std::{collections::HashMap, fs::read_to_string};

fn hammers() -> HashMap<[char; 2], char> {
    HashMap::from([
        (['B', 'C'], 'A'),
        (['C', 'B'], 'A'),
        (['D', 'D'], 'B'),
        (['B', 'D'], 'B'),
        (['C', 'D'], 'C'),
        (['F', 'E'], 'C'),
        (['A', 'F'], 'D'),
        (['F', 'A'], 'D'),
    ])
}

fn check_key(key: &[char], hammers: &HashMap<[char; 2], char>) -> bool {
    let len = key.len();

    if len == 1 {
        return key[0] == 'A';
    }

    for index in 0..len - 1 {
        let output = [key[index], key[index + 1]];

        match hammers.get(&output) {
            Some(letter) => {
                let new_key = [&key[..index], [*letter].as_slice(), &key[index + 2..]].concat();

                match check_key(&new_key, hammers) {
                    true => return true,
                    false => continue,
                };
            }
            None => continue,
        };
    }

    false
}

pub fn solve() -> Option<String> {
    let hammer_map = hammers();
    for line in read_to_string("../../input/21_keymaker_forge.txt")
        .unwrap()
        .lines()
    {
        let key: Vec<char> = line.chars().collect();

        if check_key(&key, &hammer_map) {
            return Some(line.to_owned());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::puzzle_one::*;

    #[test]
    fn valid_keys() {
        let tests = vec!["A", "BC", "CB", "DDC", "BDC", "BCD", "AFDFCDAFFE"];
        let hammer_map = hammers();

        for test in tests {
            let test: Vec<char> = test.chars().collect();
            assert!(check_key(&test, &hammer_map));
        }
    }
}
