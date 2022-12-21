use log::{info, debug, error};
use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

type Stack = Vec<char>;

fn print_stacks(stacks: &HashMap<usize, Vec<char>>) {
    let mut keys: Vec<usize> = stacks.keys().cloned().collect();
    while keys.len() != 0 {
        let min_index = keys.iter().position_min().unwrap();
        let min_value = keys.get(min_index).unwrap().clone();
        keys.remove(min_index);

        let stack = stacks.get(&min_value).unwrap();
        info!("Stack {} looks like this : {:?}", min_value, stack);
    }
}

pub fn solve(user_input: Vec<String>) {
    // Split user input to stacks and operations
    let mut stack_state_input: Vec<String> = vec![];
    let mut operations: Vec<String> = user_input.clone();

    for _ in 0..user_input.len() {
        let temp_line: String = operations.remove(0);

        if temp_line.len() == 0 {
            break;
        } else {
            stack_state_input.push(temp_line);
        }
    }

    debug!("Stack state looks like this :\n{}\n", stack_state_input.join("\n"));
    // debug!("Operations to perform are :\n{}\n", operations.join("\n"));

    // Get data for parsing stack state from index line
    let mut stack_id_to_index: HashMap<usize, usize> = HashMap::new();
    let index_line: String = stack_state_input.remove(stack_state_input.len()-1);

    for (i, v) in index_line.chars().enumerate() {
        if v != ' ' {
            let stack_id = v.to_string().parse::<usize>().unwrap();
            stack_id_to_index.insert(stack_id, i);
        }
    }



    // Init stacks
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    for i in stack_id_to_index.keys() {
        stacks.insert(*i, Vec::new());
    }


    // Parse initial stack state input into stacks
    for line in stack_state_input {
        for (stack_id, index) in &stack_id_to_index {
            let c = line.chars().nth(*index).unwrap();
            if c != ' ' {
                stacks.get_mut(&stack_id).unwrap().push(c);
            }
        }
    }

    info!("-------- INITIAL STATE --------");
    print_stacks(&stacks);


    // Execute operations on stacks
    for line in operations {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for cap in re.captures_iter(&line) {
            let move_count = (&cap[1]).parse::<usize>().unwrap();
            let move_from = (&cap[2]).parse::<usize>().unwrap();
            let move_to = (&cap[3]).parse::<usize>().unwrap();
            debug!("PARSED: Move {} from {} to {}", move_count, move_from, move_to);

            for _ in 0..move_count {
                let stack = stacks.get_mut(&move_from).unwrap();
                let value = stack.remove(0);

                let new_stack = stacks.get_mut(&move_to).unwrap();
                new_stack.insert(0, value);
            }
        }
    }


    info!("-------- FINAL STATE --------");
    print_stacks(&stacks);
}