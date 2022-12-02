#[derive(Debug)]

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_char(c: char) -> Option<Move> {
        match c {
            'A' => Some(Move::Rock),
            'B' => Some(Move::Paper),
            'C' => Some(Move::Scissors),
            'X' => Some(Move::Rock),
            'Y' => Some(Move::Paper),
            'Z' => Some(Move::Scissors),
            _ => None,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_char(c: char) -> Option<Outcome> {
        match c {
            'Z' => Some(Outcome::Win),
            'Y' => Some(Outcome::Draw),
            'X' => Some(Outcome::Lose),
            _ => None,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    input.lines()
        .map(|line| (line.chars().nth(0), line.chars().nth(2)))
        .filter(|(left, right)| left.is_some() && right.is_some())
        .map(|(left, right)| (left.unwrap(), right.unwrap()))
        .collect()
}

fn parse_moves_pairs(input: &str) -> Vec<(Move, Move)> {
    parse_input(input).iter()
        .map(|(left, right)| (Move::from_char(*left), Move::from_char(*right)))
        .filter(|(left, right)| left.is_some() && right.is_some())
        .map(|(left, right)| (left.unwrap(), right.unwrap()))
        .collect()
}

fn parse_move_outcome_pairs(input: &str) -> Vec<(Move, Outcome)> {
    parse_input(input).iter()
        .map(|(left, right)| (Move::from_char(*left), Outcome::from_char(*right)))
        .filter(|(left, right)| left.is_some() && right.is_some())
        .map(|(left, right)| (left.unwrap(), right.unwrap()))
        .collect()
}

fn find_outcome(my_move: &Move, opponent_move: &Move) -> Outcome {
    match (my_move, opponent_move) {
        (Move::Rock, Move::Rock) => Outcome::Draw,
        (Move::Rock, Move::Paper) => Outcome::Lose,
        (Move::Rock, Move::Scissors) => Outcome::Win,
        (Move::Paper, Move::Rock) => Outcome::Win,
        (Move::Paper, Move::Paper) => Outcome::Draw,
        (Move::Paper, Move::Scissors) => Outcome::Lose,
        (Move::Scissors, Move::Rock) => Outcome::Lose,
        (Move::Scissors, Move::Paper) => Outcome::Win,
        (Move::Scissors, Move::Scissors) => Outcome::Draw,
    }
}

fn find_move(opponent_move: &Move, desired_outcome: &Outcome) -> Move {
    match (opponent_move, desired_outcome) {
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Draw) => Move::Rock,
        (Move::Rock, Outcome::Lose) => Move::Scissors,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Paper, Outcome::Draw) => Move::Paper,
        (Move::Paper, Outcome::Lose) => Move::Rock,
        (Move::Scissors, Outcome::Win) => Move::Rock,
        (Move::Scissors, Outcome::Draw) => Move::Scissors,
        (Move::Scissors, Outcome::Lose) => Move::Paper,
    }
}

pub fn section1(input: &str) -> u32 {
    let moves = parse_moves_pairs(&input);
    let outcomes: Vec<Outcome> = moves.iter()
        .map(|(opponent_move, my_move)| find_outcome(my_move, opponent_move))
        .collect();
    let scores: Vec<u32> = moves.iter().zip(outcomes.iter())
        .map(|((_opponent_move, my_move), outcome)| {
            let my_score = my_move.score();
            let outcome_score = outcome.score();
            my_score + outcome_score
        })
        .collect();
    let total_score: u32 = scores.iter().sum();
    
    total_score
}

pub fn section2(input: &str) -> u32 {
    let move_outcomes = parse_move_outcome_pairs(input);
    let my_moves: Vec<Move> = move_outcomes.iter()
        .map(|(opponent_move, desired_outcome)| find_move(opponent_move, desired_outcome))
        .collect();
    let scores: Vec<u32> = move_outcomes.iter().zip(my_moves.iter())
        .map(|((_opponent_move, desired_outcome), my_move)| {
            let my_score = my_move.score();
            let outcome_score = desired_outcome.score();
            my_score + outcome_score
        })
        .collect();
    let total_score: u32 = scores.iter().sum();

    total_score
}