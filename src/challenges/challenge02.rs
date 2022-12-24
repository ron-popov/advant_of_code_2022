use core::fmt;

use log::{info, error, warn, trace, debug};

const FIRST_ROCK: char          = 'A';
const FIRST_PAPER: char         = 'B';
const FIRST_SCISSORS: char      = 'C';

const SECOND_LOSE: char         = 'X';
const SECOND_DRAW: char         = 'Y';
const SECOND_WIN: char          = 'Z';

const SCORE_ROCK: usize         = 1;
const SCORE_PAPER: usize        = 2;
const SCORE_SCISSORS: usize     = 3;

const SCORE_LOSE: usize         = 0;
const SCORE_DRAW: usize         = 3;
const SCORE_WIN: usize          = 6;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw
}

fn parse_line(line: String) -> (Move, Outcome) {
    let opponent_move = match line.chars().nth(0).unwrap() {
        FIRST_ROCK => Move::Rock,
        FIRST_PAPER => Move::Paper,
        FIRST_SCISSORS => Move::Scissors,
        _ => panic!("Invalid input")
    };

    let target_outcome = match line.chars().nth(2).unwrap() {
        SECOND_WIN => Outcome::Win,
        SECOND_LOSE => Outcome::Lose,
        SECOND_DRAW => Outcome::Draw,
        _ => panic!("Invalid input")
    };

    return (opponent_move, target_outcome)
}

fn get_outcome(opponent: &Move, mine: &Move) -> Outcome {
    match opponent {
        Move::Rock => match mine {
            Move::Rock => Outcome::Draw,
            Move::Paper => Outcome::Win,
            Move::Scissors => Outcome::Lose
        },
        Move::Paper => match mine {
            Move::Rock => Outcome::Lose,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win
        },
        Move::Scissors => match mine {
            Move::Rock => Outcome::Win,
            Move::Paper => Outcome::Lose,
            Move::Scissors => Outcome::Draw
        }
    }
}

fn calc_score(my_move: &Move, outcome: &Outcome) -> usize {
    let move_score: usize = match my_move {
        Move::Rock => SCORE_ROCK,
        Move::Paper => SCORE_PAPER,
        Move::Scissors => SCORE_SCISSORS
    };

    let outcome_score: usize = match outcome {
        Outcome::Win => SCORE_WIN,
        Outcome::Lose => SCORE_LOSE,
        Outcome::Draw => SCORE_DRAW
    };

    return move_score + outcome_score
}

fn calc_my_move(opponent: &Move, target_outcome: &Outcome ) -> Move {
    match opponent {
        Move::Rock => match target_outcome {
            Outcome::Win => Move::Paper,
            Outcome::Draw => Move::Rock,
            Outcome::Lose => Move::Scissors
        },
        Move::Paper => match target_outcome {
            Outcome::Win => Move::Scissors,
            Outcome::Draw => Move::Paper,
            Outcome::Lose => Move::Rock
        },
        Move::Scissors => match target_outcome {
            Outcome::Win => Move::Rock,
            Outcome::Draw => Move::Scissors,
            Outcome::Lose => Move::Paper
        }
    }
}

pub fn solve(input_data: Vec<String>) {
    let mut total_score: usize = 0;
    for line in input_data {
        let (opponent, target_outcome) = parse_line(line);
        let my_move = calc_my_move(&opponent, &target_outcome);

        // Calc score
        let score: usize = calc_score(&my_move, &target_outcome);
        total_score += score;

        debug!("Opponent used {:?}, the target result is {:?}, you used {:?}, score is {}", 
            opponent, target_outcome, my_move, score);
    }

    info!("Final score is {}", total_score);
}