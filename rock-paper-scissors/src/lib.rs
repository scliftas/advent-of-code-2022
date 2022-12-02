enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score_of_shape(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn score_against(&self, opposing_choice: &Choice) -> u32 {
        match self {
            Choice::Rock => match opposing_choice {
                Choice::Paper => 0,
                Choice::Rock => 3,
                Choice::Scissors => 6,
            },
            Choice::Paper => match opposing_choice {
                Choice::Scissors => 0,
                Choice::Paper => 3,
                Choice::Rock => 6,
            },
            Choice::Scissors => match opposing_choice {
                Choice::Rock => 0,
                Choice::Scissors => 3,
                Choice::Paper => 6,
            },
        }
    }

    fn choice_for_desired_outcome(&self, desired_outcome: &str) -> Choice {
        match self {
            Choice::Rock => match desired_outcome {
                "X" => Choice::Scissors,
                "Y" => Choice::Rock,
                "Z" => Choice::Paper,
                _ => panic!("Invalid outcome"),
            },
            Choice::Paper => match desired_outcome {
                "X" => Choice::Rock,
                "Y" => Choice::Paper,
                "Z" => Choice::Scissors,
                _ => panic!("Invalid outcome"),
            },
            Choice::Scissors => match desired_outcome {
                "X" => Choice::Paper,
                "Y" => Choice::Scissors,
                "Z" => Choice::Rock,
                _ => panic!("Invalid outcome"),
            },
        }
    }
}

fn get_choice_for_input(input: &str) -> Choice {
    match input {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => panic!("Invalid choice"),
    }
}

pub fn calculate_total_score_of_rounds(rounds_list: &str) -> u32 {
    let score_of_round = |round: &str| {
        let choices: Vec<Choice> = round.split(" ").map(get_choice_for_input).collect();

        let opposing_choice = &choices[0];
        let my_choice = &choices[1];

        my_choice.score_of_shape() + my_choice.score_against(opposing_choice)
    };

    rounds_list.lines().map(score_of_round).sum()
}

pub fn calculate_total_score_of_desired_outcomes(rounds_list: &str) -> u32 {
    let score_of_round = |round: &str| {
        let inputs: Vec<&str> = round.split(" ").collect();

        let opposing_choice = inputs[0];
        let desired_outcome = inputs[1];

        let opposing_choice = get_choice_for_input(&opposing_choice);
        let my_choice = opposing_choice.choice_for_desired_outcome(&desired_outcome);

        &my_choice.score_of_shape() + &my_choice.score_against(&opposing_choice)
    };

    rounds_list.lines().map(score_of_round).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_total_score() {
        let result = calculate_total_score_of_rounds("A Y\nB X\nC Z");

        assert_eq!(result, 15);
    }

    #[test]
    fn it_works_for_desired_outcomes() {
        let result = calculate_total_score_of_desired_outcomes("A Y\nB X\nC Z");

        assert_eq!(result, 12);
    }
}
