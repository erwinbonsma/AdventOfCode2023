use std::error::Error;
use std::cmp::min;
use std::collections::HashMap;


#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
struct Pos(usize, usize);

fn contains_symbol(
    x_min: usize, y_min: usize, x_max: usize, y_max: usize, lines: &Vec<&str>
) -> bool {
    for y in y_min..y_max {
        for (x, ch) in lines[y].chars().enumerate() {
            if x >= x_max { 
                break;
            }
            if x < x_min {
                continue;
            }
            if !ch.is_ascii_digit() && ch != '.' {
                return true;
            }
        }
    }

    false
}

fn find_gears(
    x_min: usize, y_min: usize, x_max: usize, y_max: usize, lines: &Vec<&str>
) -> Vec<Pos> {
    let mut gear_pos = Vec::new();

    for y in y_min..y_max {
        for (x, ch) in lines[y].chars().enumerate() {
            if x >= x_max { 
                break;
            }
            if x < x_min {
                continue;
            }
            if ch == '*' {
                gear_pos.push(Pos(x, y));
            }
        }
    }

    gear_pos
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;

    for (y, line) in lines.iter().enumerate() {
        let mut start_pos: Option<usize> = None;

        for (x, ch) in line.chars().chain(".".chars()).enumerate() {
            if ch.is_ascii_digit() {
                if start_pos.is_none() {
                    start_pos = Some(x);
                }
            } else {
                if let Some(start) = start_pos {
                    let num = line[start..x].parse::<u32>().unwrap();

                    if contains_symbol(
                        if start > 0 { start - 1 } else { 0 },
                        if y > 0 { y - 1} else { 0 },
                        min(x + 1, line.len()),
                        min(y + 2, lines.len()),
                        &lines
                    ) {
                        total += num;
                    }
                    start_pos = None;
                }
            }
        }
    }

    println!("{}", total);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut gear_links = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        let mut start_pos: Option<usize> = None;

        for (x, ch) in line.chars().chain(".".chars()).enumerate() {
            if ch.is_ascii_digit() {
                if start_pos.is_none() {
                    start_pos = Some(x);
                }
            } else {
                if let Some(start) = start_pos {
                    let num = line[start..x].parse::<u32>().unwrap();

                    let gears = find_gears(
                        if start > 0 { start - 1 } else { 0 },
                        if y > 0 { y - 1} else { 0 },
                        min(x + 1, line.len()),
                        min(y + 2, lines.len()),
                        &lines
                    );

                    for gear in gears {
                        gear_links.entry(gear).or_insert(Vec::new()).push(num);
                    }

                    start_pos = None;
                }
            }
        }
    }

    let mut total = 0;
    for (_gear, links) in gear_links.iter() {
        if links.len() == 2 {
            let prod = links[0] * links[1];
            total += prod;
        }
    }

    println!("{}", total);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");

    part1(input)?;

    part2(input)
}
