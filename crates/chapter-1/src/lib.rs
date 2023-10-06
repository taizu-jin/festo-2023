mod puzzle_one;
mod puzzle_three;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(puzzle_one::solve(), Some("AFDFCDAFFE".into()))
    }

    #[test]
    fn part_three() {
        assert_eq!(puzzle_three::solve(), Some(55541))
    }
}
