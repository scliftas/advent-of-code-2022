pub fn get_highest_calorie_totals_in_list(calories_list: &str, amount_to_total: usize) -> u32 {
    let parse_calories =
        |calories: &str| calories.trim().parse::<u32>().expect("Cannot parse string");

    let sum_calorie_group = |calorie_group: &str| calorie_group.lines().map(parse_calories).sum();

    let sort_descending = |a: &u32, b: &u32| b.cmp(a);

    let mut counted_calories = calories_list
        .split("\n\n")
        .map(sum_calorie_group)
        .collect::<Vec<u32>>();

    counted_calories.sort_by(sort_descending);

    counted_calories[..amount_to_total].into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_one() {
        let result = get_highest_calorie_totals_in_list(
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000",
            1,
        );

        assert_eq!(result, 24000);
    }

    #[test]
    fn it_works_for_three() {
        let result = get_highest_calorie_totals_in_list(
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000",
            3,
        );

        assert_eq!(result, 45000);
    }
}
