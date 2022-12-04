fn get_priority_of_item(item_to_check: char) -> u32 {
    let all_items = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let mut all_items = all_items.chars();

    (all_items.position(|item| item == item_to_check).unwrap() as u32) + 1
}

fn calculate_sum_of_rucksack(rucksack: &str) -> u32 {
    let compartments = rucksack.split_at(rucksack.chars().count() / 2);

    let mut duplicate_items: Vec<char> = vec![];

    for item in compartments.1.chars() {
        if compartments.0.contains(item) && !duplicate_items.contains(&item) {
            duplicate_items.push(item);
        }
    }

    duplicate_items.into_iter().map(get_priority_of_item).sum()
}

fn calculate_sum_of_rucksack_group(rucksacks: &[&str]) -> u32 {
    let mut duplicate_items: Vec<char> = vec![];

    for item in rucksacks[2].chars() {
        if rucksacks[0].contains(item)
            && rucksacks[1].contains(item)
            && !duplicate_items.contains(&item)
        {
            duplicate_items.push(item);
        }
    }

    duplicate_items.into_iter().map(get_priority_of_item).sum()
}

pub fn calculate_sum_of_items_to_reorganize(rucksack_list: &str) -> u32 {
    rucksack_list.lines().map(calculate_sum_of_rucksack).sum()
}

pub fn calculate_sum_of_groups(rucksack_list: &str) -> u32 {
    rucksack_list
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(calculate_sum_of_rucksack_group)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_summing_priorities() {
        let result = calculate_sum_of_items_to_reorganize(
            "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw",
        );

        assert_eq!(result, 157);
    }

    #[test]
    fn it_works_for_summing_groups() {
        let result = calculate_sum_of_groups(
            "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw",
        );

        assert_eq!(result, 70);
    }
}
