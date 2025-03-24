use regex::Regex;
use std::error::Error;
use std::collections::HashMap;
use itertools::Itertools;

fn find_number_part1(s: &str, regex: &Regex) -> u32 {
    let (_, [digit]) = regex.captures(s)
        .expect("Failed to match regex")
        .extract();

    digit.parse::<u32>().unwrap()
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let regex_first = Regex::new(r"^\D*(\d)")?;
    let regex_last = Regex::new(r"(\d)\D*$")?;

    let result: u32 = input.lines()
            .map(|s| {
                find_number_part1(s, &regex_first) * 10 + find_number_part1(s, &regex_last)
            })
            .sum();
    println!("{}", result);

    Ok(())
}

fn value_of(digit: &str, digits: &HashMap<&str, i32>) -> i32 {
    if let Some(&value) = digits.get(digit) {
        value
    } else {
        digit.parse::<i32>().unwrap()
    }
}

fn find_number_part2(s: &str, regex: &Regex, digits: &HashMap<&str, i32>) -> i32 {
    let (_, [digit]) = regex.captures(s)
        .expect("Failed to match regex")
        .extract();

    value_of(digit, digits)
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let digits = HashMap::from([
        ("one", 1),
        ("two", 2), 
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let pattern = digits.keys().map(|&key| key).join("|");
    let regex_first = Regex::new(&(r"(\d|".to_owned() + &pattern + ").*"))?;
    let regex_last = Regex::new(&(r".*(\d|".to_owned() + &pattern + ")"))?;
    let mut total = 0;
    for line in input.lines() {
        let first = find_number_part2(line, &regex_first, &digits);
        let last = find_number_part2(line, &regex_last, &digits);

        total += first * 10 + last;
    }

    println!("{}", total);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");

    part1(input)?;

    part2(input)
}
