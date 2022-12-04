fn get_task_range_for_group(group: &str) -> Vec<u32> {
    group
        .split("-")
        .map(|task| task.parse().unwrap())
        .collect::<Vec<u32>>()
}

fn groups_duplicate_each_other(groups: &str) -> bool {
    let groups = groups.split(",").collect::<Vec<&str>>();
    let first_group_task_range = get_task_range_for_group(groups[0]);
    let second_group_task_range = get_task_range_for_group(groups[1]);

    let first_group_contains_second_group = first_group_task_range[0] <= second_group_task_range[0]
        && first_group_task_range[1] >= second_group_task_range[1];

    let second_group_contains_first_group = second_group_task_range[0] <= first_group_task_range[0]
        && second_group_task_range[1] >= first_group_task_range[1];

    first_group_contains_second_group || second_group_contains_first_group
}

fn groups_overlap_each_other(groups: &str) -> bool {
    let groups = groups.split(",").collect::<Vec<&str>>();
    let first_group_task_range = get_task_range_for_group(groups[0]);
    let second_group_task_range = get_task_range_for_group(groups[1]);

    (first_group_task_range[0] <= second_group_task_range[1])
        && (second_group_task_range[0] <= first_group_task_range[1])
}

pub fn count_groups_with_duplicate_tasks(task_list: &str) -> usize {
    task_list
        .lines()
        .filter(|groups| groups_duplicate_each_other(*groups))
        .count()
}

pub fn count_groups_with_overlapping_tasks(task_list: &str) -> usize {
    task_list
        .lines()
        .filter(|groups| groups_overlap_each_other(*groups))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_duplicate_tasks() {
        let result = count_groups_with_duplicate_tasks(
            "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8",
        );

        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_for_overlapping_tasks() {
        let result = count_groups_with_overlapping_tasks(
            "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8",
        );

        assert_eq!(result, 4);
    }
}
