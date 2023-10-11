mod puzzle_one;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(puzzle_one::solve(), Some("AFBFACDDFFE".into()))
    }
}
