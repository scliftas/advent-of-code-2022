fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

pub fn find_index_of_first_unique_set(buffer: &str, set_size: usize) -> usize {
    let buffer = buffer.chars().collect::<Vec<_>>();

    for index in 0..buffer.len() {
        let set_end_index = index + set_size;
        let next_set: String = buffer[index..set_end_index].iter().collect();

        match unique(&next_set) {
            None => return set_end_index,
            Some(..) => continue,
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_correct_index_of_first_marker() {
        let first_result = find_index_of_first_unique_set("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4);
        let second_result = find_index_of_first_unique_set("bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
        let third_result = find_index_of_first_unique_set("nppdvjthqldpwncqszvftbrmjlhg", 4);
        let fourth_result = find_index_of_first_unique_set("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4);
        let fifth_result = find_index_of_first_unique_set("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);

        assert_eq!(first_result, 7);
        assert_eq!(second_result, 5);
        assert_eq!(third_result, 6);
        assert_eq!(fourth_result, 10);
        assert_eq!(fifth_result, 11);
    }

    #[test]
    fn it_returns_the_correct_index_of_first_message() {
        let first_result = find_index_of_first_unique_set("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14);
        let second_result = find_index_of_first_unique_set("bvwbjplbgvbhsrlpgdmjqwftvncz", 14);
        let third_result = find_index_of_first_unique_set("nppdvjthqldpwncqszvftbrmjlhg", 14);
        let fourth_result = find_index_of_first_unique_set("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14);
        let fifth_result = find_index_of_first_unique_set("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14);

        assert_eq!(first_result, 19);
        assert_eq!(second_result, 23);
        assert_eq!(third_result, 23);
        assert_eq!(fourth_result, 29);
        assert_eq!(fifth_result, 26);
    }
}
