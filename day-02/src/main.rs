use std::error::Error;
use std::collections::HashMap;
use std::cmp::max;

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut total = 0;

    for line in input.lines() {
        let colon_pos = line.find(':').unwrap();
        let id = line[5..colon_pos].parse::<u32>().unwrap();
        let results = line[colon_pos + 2..].split("; ").collect::<Vec<&str>>();

        let mut ok = true;
        for result in results {
            let items = result.split(", ").collect::<Vec<&str>>();
            for item in items {
                let fields = item.split(" ").collect::<Vec<&str>>();
                let num = fields[0].parse::<u32>().unwrap();
                let color = fields[1];

                match color {
                    "red" => {
                        if num > 12 {
                            ok = false;
                            break;
                        }
                    }
                    "green" => {
                        if num > 13 {
                            ok = false;
                            break;
                        }
                    }
                    "blue" => {
                        if num > 14 {
                            ok = false;
                            break;
                        }
                    }
                    &_ => todo!()
                }
            }
            if !ok {
                println!("Invalid: {}. {}", id, result);
                break;
            }
        }
        if ok {
            total += id;
        }
    }

    println!("{}", total);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut total = 0;

    for line in input.lines() {
        let colon_pos = line.find(':').unwrap();
        let results = line[colon_pos + 2..].split("; ").collect::<Vec<&str>>();

        let mut min_cubes = HashMap::new();
        for result in results {
            let items = result.split(", ").collect::<Vec<&str>>();
            for item in items {
                let fields = item.split(" ").collect::<Vec<&str>>();
                let num = fields[0].parse::<u32>().unwrap();
                let color = fields[1];
                let count = min_cubes.entry(color).or_insert(0);
                *count = max(*count, num);
            }
        }

        if min_cubes.len() == 3 {
            let prod = min_cubes.iter().fold(1, |acc, (_, &value)| acc * value);
            total += prod;
        } else {
            println!("One color missing: {} => {:?}", line, min_cubes);
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