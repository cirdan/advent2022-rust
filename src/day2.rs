use std::fs;
use Move::{Paper, Rock, Scissors};


use crate::advent;
use crate::day2::Player::{Elf, Me};
use crate::day2::RPSStrategy::{Random, React};
use crate::rock_paper_scissors::{Move, RPS};

pub fn day2() {
    advent::day_intro(2);
    const FILE_PATH: &str = "/usr/src/myapp/src/day2.txt";

    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let rounds_random = parse_game(&contents, &Random);
    let rounds_react = parse_game(&contents, &React);
    let scoring_machine = ScoringMachine { rps: RPS { player_1: Elf, player_2: Me } };
    let score_strategy_random = scoring_machine.score_game(rounds_random);
    let score_strategy_react = scoring_machine.score_game(rounds_react);

    println!("Score RockPapersScissors (first strategy) : {score_strategy_random}");
    println!("Score RockPapersScissors (optimized strategy) : {score_strategy_react}");
}


pub(crate) struct Round {
    pub(crate) my_move: Move,
    pub(crate) elf_move: Move,
}

fn parse_game(all_lines_as_str: &str, strategy: &RPSStrategy) -> Vec<Round> {
    all_lines_as_str.trim().split("\n").map(|s| parse_round(s, &strategy)).collect()
}

enum RPSStrategy {
    React,
    Random,
}

fn parse_round(input: &str, strategy: &RPSStrategy) -> Round {
    let split = input.split(" ");
    let elf_move_str = split.clone().nth(0);
    let my_move_str = split.clone().nth(1);
    let elf_move: Move = match elf_move_str.unwrap() {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => Rock,
    };
    let my_move = match strategy {
        Random => match my_move_str.unwrap() {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => Rock,
        },
        React => compute_reactive_move_for(&elf_move, my_move_str.unwrap())
    };


    Round {
        elf_move: elf_move,
        my_move: my_move,
    }
}

fn compute_reactive_move_for(elf_move: &Move,my_move_guide: &str) -> Move {
    match my_move_guide{
        "X" => match elf_move {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
        "Y" => match elf_move {
            Rock => Rock,
            Paper => Paper,
            Scissors => Scissors,
        }
        "Z" => match elf_move {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
        _ => Rock,
    }

}


#[derive(PartialEq, Clone, Copy)]
#[derive(Debug)]
pub enum Player {
    Elf,
    Me,
}

struct ScoringMachine {
    rps: RPS,
}


impl ScoringMachine {
    fn score_round(&self, round: &Round) -> i32 {
        let mut score = 0;
        score += match round.my_move {
            Scissors => 3,
            Paper => 2,
            Rock => 1,
        };

        if self.rps.winner(&round.my_move, &round.elf_move) == Some(self.rps.player_1) {
            score += 6
        } else if self.rps.winner(&round.my_move, &round.elf_move) == None {
            score += 3
        }

        score
    }
    pub(crate) fn score_game(&self, rounds: Vec<Round>) -> i32 {
        rounds.into_iter().fold(0, |acc, round| {
            return acc + self.score_round(&round);
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::day2::{parse_game, parse_round, Round, ScoringMachine};
    use crate::day2::Player::{Elf, Me};
    use crate::day2::RPSStrategy::{Random, React};
    use crate::rock_paper_scissors::Move::{Paper, Rock, Scissors};
    use crate::rock_paper_scissors::RPS;

    #[test]
    fn parsing_one_line_strategy_random() {
        let input = "A Y";
        let round: Round = parse_round(input, &Random);
        assert_eq!(Rock, round.elf_move);
        assert_eq!(Paper, round.my_move);
    }
    #[test]
    fn parsing_one_line_strategy_react() {
        let input = "A Y";
        let round: Round = parse_round(input, &React);
        assert_eq!(Rock, round.elf_move);
        assert_eq!(Rock, round.my_move);
    }

    #[test]
    fn scoring_several_lines_strategy_random() {
        let input = "C Z\nA Y\nB X\n";
        let rounds: Vec<Round> = parse_game(input, &Random);
        assert_eq!(3, rounds.len());
        let scoring_machine = ScoringMachine { rps: RPS { player_1: Elf, player_2: Me } };
        assert_eq!(15, scoring_machine.score_game(rounds));
    }
    #[test]
    fn scoring_several_lines_strategy_react() {
        let input = "C Z\nA Y\nB X\n";
        let rounds: Vec<Round> = parse_game(input, &React);
        let scoring_machine = ScoringMachine { rps: RPS { player_1: Elf, player_2: Me } };
        assert_eq!(12, scoring_machine.score_game(rounds));
    }

    #[test]
    fn round_elf_rock_me_paper_scores_8() {
        let scoring_machine = ScoringMachine { rps: RPS { player_1: Elf, player_2: Me } };
        let round: Round = Round { elf_move: Rock, my_move: Paper };
        assert_eq!(8, scoring_machine.score_round(&round));
    }

    #[test]
    fn round_elf_paper_me_rock_scores_1() {
        let scoring_machine = ScoringMachine { rps: RPS { player_1: Elf, player_2: Me } };
        let round: Round = Round { elf_move: Paper, my_move: Rock };
        assert_eq!(1, scoring_machine.score_round(&round));
    }

    #[test]
    fn round_elf_scissors_me_scissors_scores_6() {
        let scoring_machine = ScoringMachine { rps: RPS { player_1: Elf, player_2: Me } };
        let round: Round = Round { elf_move: Scissors, my_move: Scissors };
        assert_eq!(6, scoring_machine.score_round(&round));
    }
}
