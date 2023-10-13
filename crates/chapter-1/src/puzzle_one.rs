use std::{collections::HashMap, fs::read_to_string};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hammer {
    index: usize,
    letter: char,
}

impl From<(usize, char)> for Hammer {
    fn from(value: (usize, char)) -> Self {
        Self {
            index: value.0,
            letter: value.1,
        }
    }
}

#[derive(Clone, Copy)]
struct Output([char; 2]);

impl From<[char; 2]> for Output {
    fn from(value: [char; 2]) -> Self {
        Output(value)
    }
}

fn hammers() -> HashMap<Hammer, Output> {
    HashMap::from([
        ((1, 'A').into(), ['B', 'C'].into()),
        ((2, 'A').into(), ['C', 'B'].into()),
        ((3, 'B').into(), ['D', 'D'].into()),
        ((4, 'B').into(), ['B', 'D'].into()),
        ((5, 'C').into(), ['C', 'D'].into()),
        ((6, 'C').into(), ['F', 'E'].into()),
        ((7, 'D').into(), ['A', 'F'].into()),
        ((8, 'D').into(), ['F', 'A'].into()),
    ])
}

fn make_key(recipe: &str, hammers: &HashMap<Hammer, Output>) -> Option<String> {
    let instructions: Vec<_> = recipe.split('-').collect();
    let mut key = String::with_capacity(instructions.len());
    key.insert(0, 'A');

    for instruction in instructions {
        let (index, position) = instruction.split_once(',')?;

        let index = index.trim()[1..].trim().parse::<usize>().unwrap();
        let position = position.trim();
        let position = position[..position.len() - 1]
            .trim()
            .parse::<usize>()
            .ok()?
            - 1;

        let letter = if position == key.len() {
            key.pop()?
        } else {
            key.remove(position)
        };

        let output = hammers.get(&(index, letter).into())?;

        for (offset, char) in output.0.iter().enumerate() {
            if !key.is_char_boundary(position + offset) {
                return None;
            }
            key.insert(position + offset, *char);
        }
    }

    Some(key)
}

pub fn solve() -> Option<String> {
    let hammers = hammers();

    for line in read_to_string("../../input/11_keymaker_recipe.txt")
        .unwrap()
        .lines()
    {
        match make_key(line, &hammers) {
            None => continue,
            key => return key,
        }
    }

    None
}
