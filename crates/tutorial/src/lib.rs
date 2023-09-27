use std::fs::read_to_string;

fn why_are_we_here() -> String {
    let mut result = String::from("");

    'outer: for line in read_to_string("../../input/01_keymaker_ordered.txt")
        .unwrap()
        .lines()
    {
        let mut prev = 0;
        for c in line.chars() {
            let ascii = c as u32;
            if prev > ascii {
                continue 'outer;
            }

            prev = ascii;
        }

        result = line.to_string()
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1() {
        assert_eq!(why_are_we_here(), "bbbcccceeeffff");
    }
}
