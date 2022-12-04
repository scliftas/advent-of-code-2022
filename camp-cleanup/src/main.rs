use camp_cleanup_lib::{count_groups_with_duplicate_tasks, count_groups_with_overlapping_tasks};
use std::fs;

fn main() {
    let task_list =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let count_of_groups_with_duplicate_tasks = count_groups_with_duplicate_tasks(&task_list);
    let count_of_groups_with_overlapping_tasks = count_groups_with_overlapping_tasks(&task_list);

    println!(
        "Total count of groups with duplicate tasks: {}",
        count_of_groups_with_duplicate_tasks
    );
    println!(
        "Total count of groups with overlapping tasks: {}",
        count_of_groups_with_overlapping_tasks
    );
}
