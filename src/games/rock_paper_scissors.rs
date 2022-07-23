use std::{collections::HashMap, fmt::Display};

use rand::{distributions::Uniform, thread_rng, Rng};

use crate::utils::{Actor, Game, PlayError, TwoPlayerOutcome, WithPlayers};

#[derive(Debug, Default)]
pub struct RockPaperScissors(HashMap<Player, Choice>);

impl Iterator for RockPaperScissors {
    type Item = Player;

    fn next(&mut self) -> Option<Self::Item> {
        for player in [Player::One, Player::Two] {
            if self.0.get(&player).is_none() {
                return Some(player);
            }
        }
        self.0.clear();
        Some(Player::One)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    One = 0,
    Two = 1,
}

impl From<Player> for usize {
    fn from(player: Player) -> Self {
        player as usize
    }
}

#[derive(Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Game<2> for RockPaperScissors {
    type Action = Choice;
    type PlayerView = ();
    type PlayerId = Player;
    type Outcome = TwoPlayerOutcome;

    fn try_act(
        &mut self,
        action: Self::Action,
        player_id: Self::PlayerId,
    ) -> Result<(), crate::utils::InvalidActionError<2, Self>> {
        self.0.insert(player_id, action);
        Ok(())
    }

    fn player_view(&self, _: Self::PlayerId) -> Self::PlayerView {}

    fn outcome(&self) -> Option<Self::Outcome> {
        if let Some(choice1) = self.0.get(&Player::One) {
            if let Some(choice2) = self.0.get(&Player::Two) {
                return match (choice1, choice2) {
                    (Choice::Rock, Choice::Scissors) => Some(TwoPlayerOutcome::OneWins),
                    (Choice::Scissors, Choice::Paper) => Some(TwoPlayerOutcome::OneWins),
                    (Choice::Paper, Choice::Rock) => Some(TwoPlayerOutcome::OneWins),
                    (Choice::Rock, Choice::Paper) => Some(TwoPlayerOutcome::TwoWins),
                    (Choice::Paper, Choice::Scissors) => Some(TwoPlayerOutcome::TwoWins),
                    (Choice::Scissors, Choice::Rock) => Some(TwoPlayerOutcome::TwoWins),
                    _ => Some(TwoPlayerOutcome::Draw),
                };
            }
        }
        None
    }
}

#[derive(Debug, Default, Clone)]
struct Rocker;

impl Display for Rocker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Rocker")
    }
}

impl Actor<2, RockPaperScissors> for Rocker {
    fn act(&mut self, _: &()) -> Choice {
        Choice::Rock
    }
}

#[derive(Debug, Default, Clone)]
struct Paperer;

impl Display for Paperer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Paperer")
    }
}

impl Actor<2, RockPaperScissors> for Paperer {
    fn act(&mut self, _: &()) -> Choice {
        Choice::Paper
    }
}

#[derive(Debug, Default, Clone)]
struct Scissorer;

impl Display for Scissorer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Scissorer")
    }
}

impl Actor<2, RockPaperScissors> for Scissorer {
    fn act(&mut self, _: &()) -> Choice {
        Choice::Scissors
    }
}

#[derive(Debug, Default, Clone)]
struct Randomer;

impl Display for Randomer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Randomer")
    }
}

impl Actor<2, RockPaperScissors> for Randomer {
    fn act(&mut self, _: &()) -> Choice {
        match thread_rng().sample(Uniform::new(0u8, 3)) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            2 => Choice::Scissors,
            _ => unreachable!(),
        }
    }
}

pub fn game_matrix(game_count: usize) -> Result<Vec<Vec<Option<f64>>>, PlayError<2, RockPaperScissors>> {
    let players = vec![
        Box::new(Rocker) as Box<dyn Actor<2, _>>,
        Box::new(Paperer),
        Box::new(Scissorer),
        Box::new(Randomer),
    ];
    WithPlayers::<2, RockPaperScissors>::play_matrix(players, game_count)
}
