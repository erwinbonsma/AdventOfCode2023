use std::error::Error;
use std::collections::HashSet;
use num_bigint::BigUint;
use num_traits::One;
use std::cmp::min;

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
        let numbers = &line[colon_pos + 2..];
        let sep_pos = numbers.find("|").unwrap();

        let winning = get_numbers(&numbers[..sep_pos]);
        let owned = get_numbers(&numbers[sep_pos + 1..]);
        let overlap = winning.intersection(&owned).collect::<HashSet<_>>();

        if overlap.len() > 0 {
            score += (2 as usize).pow((overlap.len() - 1) as u32);
        }
    }

    println!("{}", score);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut card_counts = vec![BigUint::one(); lines.len()];
    let num_cards = card_counts.len();

    for (card_index, line) in lines.into_iter().enumerate() {
        let colon_pos = line.find(':').unwrap();
        let numbers = &line[colon_pos + 2..];
        let sep_pos = numbers.find("|").unwrap();

        let winning = get_numbers(&numbers[..sep_pos]);
        let owned = get_numbers(&numbers[sep_pos + 1..]);
        let overlap = winning.intersection(&owned).collect::<HashSet<_>>();

        let (before, after) = card_counts.split_at_mut(card_index + 1);
        let count = &before[card_index];
        for i in 0..min(overlap.len(), num_cards - card_index - 1) {
            after[i] += count;
        }
    }

    let total = card_counts.iter().sum::<BigUint>();
    println!("{}", total);
    // Note: Complexity of BitUint not needed for this problem, but useful to
    // retain for future reference.

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");

    part1(input)?;

    part2(input)
}