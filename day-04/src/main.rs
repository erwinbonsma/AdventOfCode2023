use std::error::Error;
use std::collections::HashSet;

fn get_numbers(s: &str) -> HashSet<u32> {
    s.split(" ")
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<u32>().unwrap())
        .collect()
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut score = 0;

    for line in input.lines() {
        let colon_pos = line.find(':').unwrap();
        let id = line[5..colon_pos].trim().parse::<u32>().unwrap();
        let numbers = &line[colon_pos + 2..];
        let sep_pos = numbers.find("|").unwrap();

        let winning = get_numbers(&numbers[..sep_pos]);
        let owned = get_numbers(&numbers[sep_pos + 1..]);
        let overlap = winning.intersection(&owned).collect::<HashSet<_>>();

        // println!("{} => {:?}", line, overlap);

        if overlap.len() > 0 {
            score += (2 as usize).pow((overlap.len() - 1) as u32);
        }
    }

    println!("{}", score);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");

    part1(input)
}