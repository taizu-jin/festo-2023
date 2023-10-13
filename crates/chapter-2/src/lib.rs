pub mod puzzle_one;
pub mod puzzle_two;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(puzzle_one::solve(), Some("AFBFACDDFFE".into()))
    }

    #[test]
    fn part_two() {
        assert_eq!(puzzle_two::solve(), "10178")
    }
}
