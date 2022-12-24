use log::{info, error, warn, trace, debug};
use std::collections::HashMap;

pub fn solve(input_data: Vec<String>) {
	info!("Solving challenge 01");

	let mut elf_calories_count: Vec<Vec<usize>> = Vec::new();
	let mut temp_elf: Vec<usize> = Vec::new();

	for line in input_data {
		trace!("Line length is {}, Line is {}", line.len(), line);
		if line == "" {
			trace!("Line is newline");

			elf_calories_count.push(temp_elf);
			temp_elf = Vec::new();

		} else {
			trace!("Line is an actual value");

			let value: usize = line.parse::<usize>().unwrap();
			temp_elf.push(value);
		}
	}

	trace!("Pushing last data");
	elf_calories_count.push(temp_elf);

	trace!("Elf calories count is {:?}", elf_calories_count);

	let mut sums: Vec<usize> = Vec::new();
	for i in 0..elf_calories_count.len() {

		let sum = elf_calories_count[i].iter().sum::<usize>();

		let mut j = 0;
		for _ in 0..sums.len() {
			if sums[j] > sum {
				trace!("{} is bigger than {}", sums[j], sum);
				break;
			} else {
				j += 1;
			}
		}
		sums.insert(j, sum);
		trace!("Sums State : {:?}", sums);
	}

	trace!("Elf calories sums is {:?}", sums);
	info!("Top 3 calories count is {}, {} and {}",
		sums[sums.len() - 1],
		sums[sums.len() - 2],
		sums[sums.len() - 3]
	);

	info!("The top 3 calories count sum is {}", 
		sums[sums.len() - 1] +
		sums[sums.len() - 2] +
		sums[sums.len() - 3]);
}