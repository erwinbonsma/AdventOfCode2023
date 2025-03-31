use std::error::Error;
use std::iter;
use std::cmp::{min, max};

#[derive(Clone, Debug)]
struct Range {
    start_idx: u64,
    len: u64,
}

#[derive(Debug)]
struct Mapping {
    src_range: Range,
    dst_idx: u64,
}

struct MappingTable {
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct ItemCollection {
    item_ranges: Vec<Range>,
}

impl Range {
    fn max_idx(&self) -> u64 {
        self.start_idx + self.len
    }

    fn overlaps_with(&self, other: &Range) -> bool {
        !(self.max_idx() <= other.start_idx || other.max_idx() <= self.start_idx)
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        let start_idx = max(self.start_idx, other.start_idx);
        let end_idx = min(self.max_idx(), other.max_idx());

        if start_idx < end_idx {
            Some(Range { start_idx: start_idx, len: end_idx - start_idx })
        } else {
            None
        }
    }
}

impl Mapping {
    fn apply_shift(&self, range: &mut Range) {
        range.start_idx -= self.src_range.start_idx;
        range.start_idx += self.dst_idx;
    }
}

impl MappingTable {
    fn new(input: &[&str]) -> MappingTable {
        let mut mappings = Vec::new();

        for line in input {
            let mapping = line.split(" ").collect::<Vec<&str>>();

            mappings.push(Mapping {
                src_range: Range {
                    start_idx: mapping[1].parse::<u64>().unwrap(),
                    len: mapping[2].parse::<u64>().unwrap()
                },
                dst_idx: mapping[0].parse::<u64>().unwrap(),
            });
        }

        MappingTable { mappings: mappings }
    }

    fn convert_range(&self, items: &Range) -> Vec<Range> {
        // Find the mappings that cover parts of the range
        let mut overlapping : Vec<usize> =
            (0..self.mappings.len())
                .filter(|i| items.overlaps_with(&self.mappings[*i].src_range))
                .collect();

        // Create new ranges for the items covered by a mapping
        let mut new_items = Vec::new();
        for i in &overlapping {
            let mapping = &self.mappings[*i];
            let mut intersection = items.intersection(&mapping.src_range).unwrap();
            mapping.apply_shift(&mut intersection);
            new_items.push(intersection);
        }

        // Create ranges for the items not covered by a mapping
        if overlapping.is_empty() {
            new_items.push(items.clone());
        } else {
            overlapping.sort_by_key(|i| self.mappings[*i].src_range.start_idx);

            let first_idx = self.mappings[*overlapping.first().unwrap()].src_range.start_idx;
            if first_idx > items.start_idx {
                new_items.push(Range { start_idx: items.start_idx, len: first_idx - items.start_idx });
            }

            let last_idx = self.mappings[*overlapping.last().unwrap()].src_range.max_idx();
            if last_idx < items.max_idx() {
                new_items.push(Range { start_idx: last_idx, len: items.max_idx() - last_idx });
            }

            if overlapping.len() > 1 {
                for i in 0..overlapping.len() - 1 {
                    let start_idx = self.mappings[overlapping[i]].src_range.max_idx();
                    let len = self.mappings[overlapping[i + 1]].src_range.start_idx - start_idx;
                    if len > 0 {
                        new_items.push(Range { start_idx: start_idx, len: len });
                    }
                }
            }
        }

        new_items
    }

    fn convert_items(&self, items: &ItemCollection) -> ItemCollection {
        let mut new_items = ItemCollection::new();

        for range in items.item_ranges.iter() {
            let converted_ranges = self.convert_range(range);

            for converted_range in converted_ranges {
                new_items.add_range(converted_range);
            }
        }

        new_items
    }
}

impl ItemCollection {
    fn new() -> ItemCollection {
        ItemCollection { item_ranges: Vec::new() }
    }

    fn add_range(&mut self, range: Range) {
        // Check if there are one or more overlapping ranges
        let overlapping : Vec<_> =
            (0..self.item_ranges.len())
                .filter(|i| range.overlaps_with(&self.item_ranges[*i]))
                .collect();

        if overlapping.is_empty() {
            // Easy case. No overlapping. Just add new range
            self.item_ranges.push(range);
            return;
        }

        // Merge ranges into one
        let min_idx = overlapping.iter()
            .map(|i| self.item_ranges[*i].start_idx)
            .chain(iter::once(range.start_idx))
            .min().unwrap();
        let len = overlapping.iter()
            .map(|i| self.item_ranges[*i].max_idx() - min_idx)
            .chain(iter::once(range.max_idx() - min_idx))
            .max().unwrap();

        // Remove the old ranges
        for idx in overlapping.into_iter().rev() {
            self.item_ranges.swap_remove(idx);
        }

        // Add the new one
        self.item_ranges.push(Range { start_idx: min_idx, len: len });
    }
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut items: Option<ItemCollection> = None;
    let mut line_block: Vec<&str> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            if let Some(old_items) = items {
                let table = MappingTable::new(&line_block[1..]);

                items = Some(table.convert_items(&old_items));
            } else {
                assert!(line_block.len() == 1);
                let colon_pos = line_block[0].find(':').unwrap();
                let mut ini_items = ItemCollection::new();
                let fields: Vec<_> =  line_block[0][colon_pos + 2..]
                    .split(" ")
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();

                for i in (0..fields.len()).step_by(2) {
                    ini_items.item_ranges.push(Range { start_idx: fields[i], len: fields[i + 1]});
                }

                items = Some(ini_items);
            }

            // println!("items = {:?}", items);

            line_block = Vec::new();
        } else {
            line_block.push(line);
        }
    }

    let table = MappingTable::new(&line_block[1..]);
    if let Some(old_items) = items {
        let final_items = table.convert_items(&old_items);
        let result = final_items.item_ranges.iter()
            .map(|r| r.start_idx)
            .min().unwrap();

        println!("{}", result);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");

    part2(input)
}