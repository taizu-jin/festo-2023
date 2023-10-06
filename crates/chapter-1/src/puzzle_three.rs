#![allow(dead_code)]

use std::{collections::VecDeque, fs::read_to_string};

pub fn solve() -> Option<usize> {
    let mut result = 0;

    for line in read_to_string("../../input/13_trap_balance.txt")
        .unwrap()
        .lines()
    {
        let (id, scale) = line.split_once(':').expect("':' always part of the input");
        let (left, right) = scale.split_once('-').expect("'-' part of scale's notation");

        let left = left.trim().split(' ').collect::<Vec<_>>();
        let right = right.trim().split(' ').collect::<Vec<_>>();

        // check if both sides contain same number of objects
        if left.len() != right.len() {
            continue;
        }

        // check if there are duplicates
        let weights = [left.as_slice(), right.as_slice()].concat();
        if (1..weights.len()).any(|i| weights[i..].contains(&weights[i - 1])) {
            continue;
        }

        let (a, b) = sum(left);
        let (c, d) = sum(right);

        // We have two fractions a/b and c/d.
        // Let Y = (a/b - c/d)
        //       = (ad - bc)/(bd)
        // Now,
        // if Y > 0 then a/b > c/d
        // if Y = 0 then a/b = c/d
        // if Y < o then a/b < c/d

        // Since bd is always positive, the sign of Y depends only on the
        // numerator (ad-bc). So we need to compute (ad-bc) only.
        if a * d != b * c {
            continue;
        }

        let id = id.trim().parse::<usize>().unwrap();
        result += id;
    }

    Some(result)
}

/// Sum a vector of 1/n weights and return a tulpe of numerator and denominator of the result.
fn sum(weights: Vec<&str>) -> (u128, u128) {
    let mut fractions = VecDeque::new();

    for weight in weights {
        let weight = weight.trim().parse::<u128>().unwrap();
        fractions.push_back(weight);
    }

    let denominator = fractions.iter().product();

    let mut numerator = 0;

    for _ in 0..fractions.len() {
        let f = fractions.pop_front().unwrap();
        numerator += fractions.iter().product::<u128>();
        fractions.push_back(f);
    }

    (numerator, denominator)
}
