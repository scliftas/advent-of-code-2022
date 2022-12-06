use std::fs;
use tuner_lib::find_index_of_first_unique_set;

fn main() {
    let buffer =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let index_of_first_marker = find_index_of_first_unique_set(&buffer, 4);
    let index_of_first_message = find_index_of_first_unique_set(&buffer, 14);

    println!("Index of first marker: {}", index_of_first_marker);
    println!("Index of first message: {}", index_of_first_message);
}
