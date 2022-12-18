use log::{info, debug, error};
use std::fmt;

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

        
        if first_elf.start <= second_elf.start && first_elf.end >= second_elf.end {
            contains_count += 1;
            debug!("{} is contained in {}", second_elf, first_elf);
        } else if first_elf.start >= second_elf.start && first_elf.end <= second_elf.end {
            contains_count += 1;
            debug!("{} is contained in {}", first_elf, second_elf);
        }
    }

    info!("Total contains {} times", contains_count);
}