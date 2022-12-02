use rock_paper_scissors_lib::{
    calculate_total_score_of_desired_outcomes, calculate_total_score_of_rounds,
};
use std::fs;

fn main() {
    let rounds_list =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let score_of_rounds = calculate_total_score_of_rounds(&rounds_list);
    let score_of_desired_outcomes = calculate_total_score_of_desired_outcomes(&rounds_list);

    println!("Total score of all rounds: {}", score_of_rounds);
    println!(
        "Total score of desired outcomes: {}",
        score_of_desired_outcomes
    );
}
