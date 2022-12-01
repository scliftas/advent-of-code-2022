use calorie_counter_lib::get_highest_calorie_totals_in_list;
use std::fs;

fn main() {
    let calories_list =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let highest_total = get_highest_calorie_totals_in_list(&calories_list, 3);

    println!("Highest total calories: {}", highest_total);
}
