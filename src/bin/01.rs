use regex::Regex;
advent_of_code::solution!(1);

fn get_first_numeric_digit(l: &str) -> u32 {
    l.chars()
        .find(|c| c.is_numeric())
        .expect("Input must have digit")
        .to_digit(10)
        .expect("Filter should only include digits")
}

fn reverse_string(l: &str) -> String {
    l.chars().rev().collect::<String>()
}

fn get_last_numeric_digit(l: &str) -> u32 {
    get_first_numeric_digit(&reverse_string(l))
}

fn trebuchet(
    input: &str,
    first_digit_strategy: fn(&str) -> u32,
    last_digit_strategy: fn(&str) -> u32,
) -> Option<u32> {
    let answer = input
        .lines()
        .map(|l| first_digit_strategy(l) * 10 + last_digit_strategy(l))
        .sum();

    Some(answer)
}

pub fn part_one(input: &str) -> Option<u32> {
    trebuchet(input, get_first_numeric_digit, get_last_numeric_digit)
}

const WORD_REGEX: &str = r"one|two|three|four|five|six|seven|eight|nine";

fn do_regex_search(l: &str, re: Regex) -> u32 {
    match re
        .find(l)
        .expect("Must be at least one digit in input")
        .as_str()
    {
        "one" | "eno" => 1,
        "two" | "owt" => 2,
        "three" | "eerht" => 3,
        "four" | "ruof" => 4,
        "five" | "evif" => 5,
        "six" | "xis" => 6,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        "nine" | "enin" => 9,
        digit if digit.chars().next().unwrap().is_numeric() => {
            digit.chars().next().unwrap().to_digit(10).unwrap()
        }
        _ => unreachable!(),
    }
}

fn get_first_digit(l: &str) -> u32 {
    do_regex_search(l, Regex::new(&format!("{WORD_REGEX}|[0-9]")).unwrap())
}

fn get_last_digit(l: &str) -> u32 {
    do_regex_search(
        &reverse_string(l),
        Regex::new(&format!("{}|[0-9]", reverse_string(WORD_REGEX))).unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    trebuchet(input, get_first_digit, get_last_digit)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
