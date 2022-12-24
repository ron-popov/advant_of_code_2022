use log::{info, error, warn, trace, debug};
use std::{collections::HashSet};

fn calc_priority(item: &char) -> usize {
    debug!("Calculating priority for {}", item);
    assert!((item >= &'a' && item <= &'z') || (item >= &'A' && item <= &'Z'));
    
    if item >= &'a' && item <= &'z' {
        return (item.clone() as usize - 'a' as usize) + 1;
    } else {
        return (item.clone() as usize - 'A' as usize) + 27;
    }
}

fn share_char(a:&String, b:&String) -> char {
    let a_set: HashSet<char> = HashSet::from_iter(a.chars());
    let b_set: HashSet<char> = HashSet::from_iter(b.chars());

    let mut intersection = a_set.intersection(&b_set);
    return *intersection.next().unwrap();
}

fn share_chars(a:&String, b:&String) -> Vec<char> {
    let a_set: HashSet<char> = HashSet::from_iter(a.chars());
    let b_set: HashSet<char> = HashSet::from_iter(b.chars());

    let intersection = a_set.intersection(&b_set);
    let mut return_vector: Vec<char> = Vec::new();
    for c in intersection.into_iter() {
        return_vector.push(c.clone());
    }

    return return_vector;
}

pub fn solve(input_data: Vec<String>) {
    let mut total_priority: usize = 0;
    let mut elfs_to_process = input_data.clone();

    while elfs_to_process.len() != 0 {
        let first_elf: String = elfs_to_process.remove(0);
        let second_elf: String = elfs_to_process.remove(0);
        let third_elf: String = elfs_to_process.remove(0);
        
        let common_badge: char = share_char(
            &share_chars(&first_elf, &second_elf).into_iter().collect(), 
            &third_elf);
        
        let badge_priority = calc_priority(&common_badge);
        total_priority += badge_priority;
    
        debug!("Common Badge for {}, {} and {} is {} and it's priority is {}", 
            first_elf, second_elf, third_elf, common_badge, badge_priority);
    }

    info!("Total badges priorities sum is {}", total_priority);
}
