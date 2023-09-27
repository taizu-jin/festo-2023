#![allow(dead_code)]

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

enum Action {
    Activate,
    Deactivate,
    Flip,
}

impl TryFrom<&str> for Action {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let action = match value {
            "inactive" => Self::Deactivate,
            "disabled" => Action::Deactivate,
            "quiet" => Action::Deactivate,
            "standby" => Action::Deactivate,
            "idle" => Action::Deactivate,
            "live" => Action::Activate,
            "armed" => Action::Activate,
            "ready" => Action::Activate,
            "primed" => Action::Activate,
            "active" => Action::Activate,
            "flipped" => Action::Flip,
            "toggled" => Action::Flip,
            "reversed" => Action::Flip,
            "inverted" => Action::Flip,
            "switched" => Action::Flip,
            _ => return Err(()),
        };

        Ok(action)
    }
}

#[derive(PartialEq, Debug)]
enum State {
    Active,
    Inactive,
}

impl State {
    fn flip(&mut self) {
        match self {
            State::Active => *self = State::Inactive,
            State::Inactive => *self = State::Active,
        }
    }
}

fn just_to_suffer_question_mark() -> usize {
    let mut result = 0;

    for line in read_to_string("../../input/03_trap_logs.txt")
        .unwrap()
        .lines()
    {
        let (id, keywords) = line
            .split_once(':')
            .expect("id and log entries are seperated by :");
        let mut logs = keywords.trim().split(' ');

        let mut state = match Action::try_from(
            logs.next()
                .expect("first element always present in the logs"),
        )
        .expect("logs contain only valid ")
        {
            Action::Activate => State::Active,
            Action::Deactivate => State::Inactive,
            Action::Flip => {
                panic!("first log entry is always either activate or deactivate action")
            }
        };

        for entry in logs {
            match Action::try_from(entry).expect("log contains valid keywords") {
                Action::Activate => state = State::Active,
                Action::Deactivate => state = State::Inactive,
                Action::Flip => state.flip(),
            }
        }

        if state == State::Inactive {
            result += id
                .trim()
                .parse::<usize>()
                .expect("logs contain only valid ids");
        }
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

    #[test]
    fn part_2() {
        assert_eq!(just_to_suffer_question_mark(), 498996);
    }
}
