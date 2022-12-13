const FILE_NAME: &str = "input.txt";

#[derive(PartialEq, Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

enum RoundOutcome {
    Win,
    Draw,
    Loss,
}

impl Action {
    fn to_action(input: char) -> Self {
        match input {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => {
                unreachable!()
            }
        }
    }

    fn get_value(&self) -> u32 {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn get_weakness(&self) -> Self {
        match *self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn get_strengths(&self) -> Self {
        match *self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

impl RoundOutcome {
    fn get_outcome(opponent_action: &Action, player_action: &Action) -> Self {
        if opponent_action.get_weakness() == *player_action {
            return Self::Win;
        }

        if opponent_action == player_action {
            return Self::Draw;
        }

        return Self::Loss;
    }

    fn get_value(&self) -> u32 {
        match *self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    fn to_outcome(input: char) -> Self {
        match input {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => {
                unreachable!()
            }
        }
    }

    fn get_action(&self, opponent_action: &Action) -> Action {
        match *self {
            Self::Win => opponent_action.get_weakness(),
            Self::Draw => opponent_action.clone(),
            Self::Loss => opponent_action.get_strengths(),
        }
    }
}

fn main() {
    let mut rounds: Vec<(Action, Action, RoundOutcome)> = Vec::new();
    // Get input
    let input = std::fs::read_to_string(FILE_NAME).expect("Failed to read input.txt file");

    // Parse into
    for line in input.lines() {
        let opponent_action = Action::to_action(line.chars().nth(0).unwrap());
        let player_action = Action::to_action(line.chars().nth(2).unwrap());
        let expected_outcome = RoundOutcome::to_outcome(line.chars().nth(2).unwrap());
        rounds.push((opponent_action, player_action, expected_outcome));
    }

    let mut total_assumed_rules = 0;
    let mut total_clarified_rules = 0;
    for round in rounds {
        let action_value = round.1.get_value();
        let round_value = RoundOutcome::get_outcome(&round.0, &round.1).get_value();
        total_assumed_rules += action_value + round_value;

        let round_value = round.2.get_value();
        let action_value = round.2.get_action(&round.0).get_value();
        total_clarified_rules += action_value + round_value;
    }

    println!("Game total: {total_assumed_rules}");
    println!("Game updated total: {total_clarified_rules}");

}
