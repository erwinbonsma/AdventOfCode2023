use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");

    for line in input.lines() {
        println!("Line {}", line);
    }

    let regex_first = Regex::new(r"^\D*(\d)")?;
    let regex_last = Regex::new(r"(\d)\D*$")?;

    let result: u32 = input.lines()
            .map(|s| {
                let (_, [d1]) = regex_first.captures(s)
                    .expect("Failed to match regex")
                    .extract();
                let (_, [d2]) = regex_last.captures(s)
                    .expect("Failed to match regex")
                    .extract();
            d1.parse::<u32>().unwrap() * 10 + d2.parse::<u32>().unwrap()
            })
            .sum();
    println!("{}", result);

    Ok(())
}
