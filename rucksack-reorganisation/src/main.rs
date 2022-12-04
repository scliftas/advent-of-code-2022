use rucksack_reorganizer_lib::{calculate_sum_of_groups, calculate_sum_of_items_to_reorganize};
use std::fs;

fn main() {
    let rucksack_list =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let sum_of_items_to_reorganize = calculate_sum_of_items_to_reorganize(&rucksack_list);
    let sum_of_groups = calculate_sum_of_groups(&rucksack_list);

    println!(
        "Total sum of all items to reorganize: {}",
        sum_of_items_to_reorganize
    );
    println!("Total sum of all groups: {}", sum_of_groups);
}
