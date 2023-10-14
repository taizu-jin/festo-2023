use std::fs::read_to_string;

pub fn solve() -> Option<usize> {
    let mut result = 0;

    for line in read_to_string("../../input/13_trap_balance.txt")
        .unwrap()
        .lines()
    {
        let (id, scale) = line.split_once(':').expect("':' always part of the input");

        if !is_balanced(scale)? {
            continue;
        }

        let id = id.trim().parse::<usize>().unwrap();
        result += id;
    }

    Some(result)
}

fn is_balanced(scale: &str) -> Option<bool> {
    let (left, _) = scale.split_once('-').expect("'-' part of scale's notation");

    let left = left.trim().split(' ').collect::<Vec<_>>();
    let count = left.len();

    let left = left
        .iter()
        .map(|v| v.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let sum = left.iter().sum();

    let factors = prime_factorizaton(sum);

    let is_balanced = match count.cmp(&factors.len()) {
        std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Equal => {
            let mut is_factor_part_of_scale = false;
            for f in factors {
                if left.contains(&f) {
                    is_factor_part_of_scale = true;
                };
            }
            is_factor_part_of_scale
        }
        std::cmp::Ordering::Greater => false,
    };

    Some(is_balanced)
}

fn prime_factorizaton(mut number: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut divisor = 2;

    while number > 1 {
        while number % divisor == 0 {
            primes.push(divisor);
            number /= divisor;
        }
        divisor += 1;

        if divisor * divisor > number {
            if number > 1 {
                primes.push(number)
            }
            break;
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_factorization() {
        assert_eq!(prime_factorizaton(105), vec![3, 5, 7]);
        assert_eq!(prime_factorizaton(6), vec![2, 3]);
        assert_eq!(prime_factorizaton(210), vec![2, 3, 5, 7]);
        assert_eq!(prime_factorizaton(310), vec![2, 5, 31]);
        assert_eq!(prime_factorizaton(30), vec![2, 3, 5]);
        assert_eq!(prime_factorizaton(1337), vec![7, 191]);
        assert_eq!(prime_factorizaton(2), vec![2]);
        assert_eq!(prime_factorizaton(7), vec![7]);
        assert_eq!(prime_factorizaton(3), vec![3]);
        assert_eq!(prime_factorizaton(54), vec![2, 3, 3, 3]);
        assert_eq!(prime_factorizaton(8), vec![2, 2, 2]);
        assert_eq!(prime_factorizaton(14), vec![2, 7]);
        assert_eq!(prime_factorizaton(12), vec![2, 2, 3]);
        assert_eq!(prime_factorizaton(39), vec![3, 13]);
        assert_eq!(prime_factorizaton(35), vec![5, 7]);
        assert_eq!(prime_factorizaton(6090), vec![2, 3, 5, 7, 29]);
    }

    #[test]
    fn prime_factorization_balanced() {
        assert_eq!(prime_factorizaton(3 + 105), vec![2, 2, 3, 3, 3]);
        assert_eq!(prime_factorizaton(3 * 105), vec![3, 3, 5, 7]);
        assert_eq!(prime_factorizaton(5 + 30 + 310), vec![3, 5, 23]);
        assert_eq!(prime_factorizaton(5 * 30 * 310), vec![2, 2, 3, 5, 5, 5, 31]);
        assert_eq!(prime_factorizaton(3 + 12), vec![3, 5]);
        assert_eq!(prime_factorizaton(3 * 12), vec![2, 2, 3, 3]);
        assert_eq!(prime_factorizaton(35 + 6090), vec![5, 5, 5, 7, 7]);
        assert_eq!(prime_factorizaton(35 * 6090), vec![2, 3, 5, 5, 7, 7, 29]);
        assert_eq!(prime_factorizaton(2 + 30), vec![2, 2, 2, 2, 2]);
        assert_eq!(prime_factorizaton(2 * 30), vec![2, 2, 3, 5]);
        assert_eq!(prime_factorizaton(5 + 195), vec![2, 2, 2, 5, 5]);
        assert_eq!(prime_factorizaton(5 * 195), vec![3, 5, 5, 13]);
    }

    #[test]
    fn prime_factorization_unbalanced() {
        assert_eq!(prime_factorizaton(2 + 6 + 210), vec![2, 109]);
        assert_eq!(prime_factorizaton(2 * 6 * 210), vec![2, 2, 2, 3, 3, 5, 7]);
        assert_eq!(prime_factorizaton(2 + 5 + 190), vec![197]);
        assert_eq!(prime_factorizaton(2 * 5 * 190), vec![2, 2, 5, 5, 19]);
        assert_eq!(prime_factorizaton(3 + 54), vec![3, 19]);
        assert_eq!(prime_factorizaton(3 * 54), vec![2, 3, 3, 3, 3]);
        assert_eq!(prime_factorizaton(2 + 7 + 70), vec![79]);
        assert_eq!(prime_factorizaton(2 * 7 * 70), vec![2, 2, 5, 7, 7]);
        assert_eq!(prime_factorizaton(8 + 14), vec![2, 11]);
        assert_eq!(prime_factorizaton(8 * 14), vec![2, 2, 2, 2, 7]);
        assert_eq!(prime_factorizaton(2 + 3 + 33), vec![2, 19]);
        assert_eq!(prime_factorizaton(2 * 3 * 33), vec![2, 3, 3, 11]);
        assert_eq!(prime_factorizaton(42 + 1337), vec![7, 197]);
        assert_eq!(prime_factorizaton(42 * 1337), vec![2, 3, 7, 7, 191]);
    }

    #[test]
    fn unbalanced() {
        assert_eq!(is_balanced("2 6 210 - X X X"), Some(false));
        assert_eq!(is_balanced("2 5 190 - X X X"), Some(false));
        assert_eq!(is_balanced("3 54 - X X"), Some(false));
        assert_eq!(is_balanced("2 7 70 - X X X"), Some(false));
        assert_eq!(is_balanced("8 14 - X X"), Some(false));
        assert_eq!(is_balanced("2 3 33 - X X X"), Some(false));
        assert_eq!(is_balanced("42 1337 - X X"), Some(false));
    }

    #[test]
    fn balanced() {
        assert_eq!(is_balanced("3 105 - X X"), Some(true));
        assert_eq!(is_balanced("5 30 310 - X X X"), Some(true));
        assert_eq!(is_balanced("3 12 - X X"), Some(true));
        assert_eq!(is_balanced("35 6090 - X X"), Some(true));
        assert_eq!(is_balanced("2 30 - X X"), Some(true));
        assert_eq!(is_balanced("5 195 - X X"), Some(true));
    }
}
