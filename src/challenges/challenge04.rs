use log::{info, debug, error};
use std::fmt;
use std::collections::HashSet;

struct Range {
    start: usize,
    end: usize
}

impl fmt::Display for Range {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("{}-{}", self.start, self.end))
    }
}

fn get_range(range_string: String) -> Range {
    let mut range_split = range_string.split("-");
    let start: usize = range_split.next().unwrap().parse::<usize>().unwrap();
    let end: usize = range_split.next().unwrap().parse::<usize>().unwrap();

    Range{start:start, end:end}
}

pub fn solve(user_input: Vec<String>) {
    let mut contains_count: usize = 0;
    for line in user_input {
        // parse line to two range structs
        let mut elfs_split = line.split(",");
        let first_elf: Range = get_range(
            elfs_split.next().unwrap().clone().to_string());
        let second_elf: Range = get_range(
            elfs_split.next().unwrap().clone().to_string());

        let mut first_set: HashSet<usize> = HashSet::new();
        let mut second_set: HashSet<usize> = HashSet::new();
        let mut total_set: HashSet<usize> = HashSet::new();

        for i in first_elf.start..first_elf.end+1 {
            first_set.insert(i);
            total_set.insert(i);
        }

        for i in second_elf.start..second_elf.end+1 {
            second_set.insert(i);
            total_set.insert(i);
        }

        if first_set.len() + second_set.len() != total_set.len() {
            debug!("{} and {} overlap", first_elf, second_elf);
            contains_count += 1;
        }
    }
    info!("Total contains {} times", contains_count);
    
}