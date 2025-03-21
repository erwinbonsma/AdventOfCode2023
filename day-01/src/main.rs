use regex::Regex;
use std::error::Error;

fn find_number(s: &str, regex: &Regex) -> u32 {
    let (_, [digit]) = regex.captures(s)
        .expect("Failed to match regex")
        .extract();

    digit.parse::<u32>().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");

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
