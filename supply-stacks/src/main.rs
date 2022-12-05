use std::fs;
use supply_stacks_lib::move_crates_and_get_top;

fn main() {
    let stack_instructions =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let top_crates_after_moves_with_9000 = move_crates_and_get_top(&stack_instructions, "9000");
    let top_crates_after_moves_with_9001 = move_crates_and_get_top(&stack_instructions, "9001");

    println!(
        "Top crates after moves with 9000: {}",
        top_crates_after_moves_with_9000
    );
    println!(
        "Top crates after moves with 9001: {}",
        top_crates_after_moves_with_9001
    );
}
