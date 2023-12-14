advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
        let digits = input
            .lines()
            .map(|l| {
                l.chars()
                    .filter(|c| c.is_numeric())
                    .map(|c| c.to_digit(10).expect("Filter should be numeric only"))
                    .collect()
            });

        let answer = digits
            .map(|arr: Vec<u32>| 10 * arr.first().unwrap() + arr.last().unwrap())
            .sum();

        Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
