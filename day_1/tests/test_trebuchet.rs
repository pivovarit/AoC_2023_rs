#[path = "../src/day_1.rs"]
mod day1;

#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn test_example() {
        let input: Vec<String> = Vec::new();
        let result = day1::trebuchet_part1(&input);
    }
}
