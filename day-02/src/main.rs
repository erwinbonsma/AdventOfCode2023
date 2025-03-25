use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");

    part1(input)
}