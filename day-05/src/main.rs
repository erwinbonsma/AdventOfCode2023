use std::error::Error;

struct Mapping {
    src_idx: u64,
    dst_idx: u64,
    len: u64,
}

fn build_table(input: &[&str]) -> Vec<Mapping> {
    let mut mappings = Vec::new();

    for line in input {
        let mapping = line.split(" ").collect::<Vec<&str>>();
        
        mappings.push(Mapping {
            src_idx: mapping[1].parse::<u64>().unwrap(),
            dst_idx: mapping[0].parse::<u64>().unwrap(),
            len: mapping[2].parse::<u64>().unwrap(),
        });
    }

    mappings
}

fn convert_items(items: &Vec<u64>, table: &Vec<Mapping>) -> Vec<u64> {
    let mut new_items = Vec::new();
    for &item in items.iter() {
        let mut new_item = None;

        for mapping in table.iter() {
            if item >= mapping.src_idx && item < mapping.src_idx + mapping.len {
                new_item = Some(mapping.dst_idx + item - mapping.src_idx);
                break;
            }
        }

        new_items.push(new_item.unwrap_or(item));
    }

    new_items
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut items: Option<Vec<u64>> = None;
    let mut line_block: Vec<&str> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            if let Some(old_items) = items {
                let table = build_table(&line_block[1..]);
                items = Some(convert_items(&old_items, &table));
            } else {
                assert!(line_block.len() == 1);
                let colon_pos = line_block[0].find(':').unwrap();
                items = Some(
                    line_block[0][colon_pos + 2..]
                        .split(" ")
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect()
                );
            }

            line_block = Vec::new();
        } else {
            line_block.push(line);
        }
    }

    let table = build_table(&line_block[1..]);
    if let Some(old_items) = items {
        let final_items = convert_items(&old_items, &table);
        let result = final_items.iter().min().unwrap();

        println!("{}", result);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");

    part1(input)
}