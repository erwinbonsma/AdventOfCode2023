use regex::Regex;
use std::error::Error;
use std::collections::HashMap;
use itertools::Itertools;

fn find_number(s: &str, regex: &Regex) -> u32 {
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
                find_number(s, &regex_first) * 10 + find_number(s, &regex_last)
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
    let regex = Regex::new(&(r"\d|".to_owned() + &pattern))?;
    let mut total = 0;
    for line in input.lines() {
        let matches: Vec<_> = regex.find_iter(line).map(|m| m.as_str()).collect();
        let value =
            value_of(matches[0], &digits) * 10
            + value_of(matches.last().unwrap(), &digits);
        println!("{} {:?} {}", line, matches, value);
        total += value;
    }

    println!("{}", total);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");

    part1(input)?;

    part2(input)
    // Wrong: 54970
    // Problem: It does not correctly handle overlapping matches. In particular, when it
    // encounters "oneight", it will only return "one" as a match, not "eight".
}
