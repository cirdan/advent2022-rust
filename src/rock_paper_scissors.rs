use Move::{Paper, Rock, Scissors};
use crate::day2::{Player};

#[derive(Clone,Copy)]
pub struct RPS {
    pub player_1: Player,
    pub player_2: Player,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}


impl RPS {
    pub(crate) fn winner(self,
                         move_player_1: Move,
                         move_player_2: Move,
    ) -> Option<Player> {
        match move_player_1 {
            Paper => match move_player_2 {
                Paper => None,
                Scissors => Option::from(self.player_2),
                Rock => Option::from(self.player_1),
            },
            Scissors => match move_player_2 {
                Paper => Option::from(self.player_1),
                Scissors => None,
                Rock => Option::from(self.player_2),
            },
            Rock => match move_player_2 {
                Paper => Option::from(self.player_2),
                Scissors => Option::from(self.player_1),
                Rock => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use Player::{Elf, Me};
    use crate::day2::Player;
    use crate::rock_paper_scissors::{RPS};
    use crate::rock_paper_scissors::Move::{Paper, Rock, Scissors};

    #[test]
    fn rock_looses_against_paper() {
        let rps = RPS { player_1: Me, player_2: Elf };
        assert_eq!(Option::from(Elf), rps.winner(Rock, Paper))
    }

    #[test]
    fn rock_wins_against_scissors() {
        let rps = RPS { player_1: Me, player_2: Elf };
        assert_eq!(Option::from(Me), rps.winner(Rock, Scissors))
    }

    #[test]
    fn rock_rock_draw() {
        let rps = RPS { player_1: Me, player_2: Elf };
        assert_eq!(Option::from(None), rps.winner(Rock, Rock))
    }

    #[test]
    fn paper_looses_against_scissors() {
        let rps = RPS { player_1: Me, player_2: Elf };
        assert_eq!(Option::from(Elf), rps.winner(Paper, Scissors))
    }

    #[test]
    fn paper_paper_draw() {
        let rps = RPS { player_1: Me, player_2: Elf };
        assert_eq!(Option::from(None), rps.winner(Paper, Paper))
    }

    #[test]
    fn scissors_scissors_draw() {
        let rps = RPS { player_1: Me, player_2: Elf };
        assert_eq!(Option::from(None), rps.winner(Scissors, Scissors))
    }
}
