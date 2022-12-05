use regex::Regex;

pub fn move_crates_and_get_top(stack_instructions: &str, crane_model: &str) -> String {
    let (stacks, movements) = stack_instructions.split_once("\n\n").unwrap();
    let number_of_stacks = stacks.lines().rev().collect::<Vec<&str>>()[0]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .len();

    let mut stacks: Vec<Vec<&str>> = vec![Vec::new(); number_of_stacks];

    for line in stack_instructions.lines() {
        if line.contains("[") {
            let regex = Regex::new(r"[[:alpha:]]|    ").unwrap();

            for (index, regex_match) in regex.find_iter(&line).enumerate() {
                if regex_match.as_str() == "    " {
                    continue;
                }

                stacks[index].push(regex_match.as_str());
            }
        }
    }

    for movement in movements.lines() {
        let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let matches: Vec<_> = regex.captures_iter(movement).collect();
        let captures = &matches[0];

        let num_of_crates_to_move = (&captures[1]).parse::<usize>().unwrap();
        let index_of_stack_to_move_from = (&captures[2]).parse::<usize>().unwrap();
        let index_of_stack_to_move_to = (&captures[3]).parse::<usize>().unwrap();

        let stack_to_move_from = stacks[index_of_stack_to_move_from - 1].clone();
        let mut stack_to_move_to = stacks[index_of_stack_to_move_to - 1].clone();
        let crates_to_move = stack_to_move_from.iter().take(num_of_crates_to_move);

        match crane_model {
            "9000" => {
                for crate_to_move in crates_to_move {
                    stack_to_move_to.insert(0, crate_to_move);
                }
            }
            "9001" => {
                for crate_to_move in crates_to_move.rev() {
                    stack_to_move_to.insert(0, crate_to_move);
                }
            }
            _ => panic!("Unexpected crane model"),
        }

        stacks[index_of_stack_to_move_from - 1] = stack_to_move_from
            .into_iter()
            .skip(num_of_crates_to_move)
            .collect();
        stacks[index_of_stack_to_move_to - 1] = stack_to_move_to;
    }

    stacks
        .into_iter()
        .map(|stack| stack[0])
        .collect::<Vec<&str>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_moves_the_crates_with_9000_and_returns_the_top_crates() {
        let result = move_crates_and_get_top(
          "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2", "9000"
        );

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn it_moves_the_crates_with_9001_and_returns_the_top_crates() {
        let result = move_crates_and_get_top(
          "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2", "9001"
        );

        assert_eq!(result, "MCD");
    }
}
