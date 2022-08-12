use dyn_clonable::clonable;
use thiserror::Error;
use std::{fmt::{Debug, Display}, collections::HashMap};
pub mod drawing;

#[clonable]
pub trait Actor<const C: usize, G: Game<C>>: Display + Debug + Clone {
    fn act(&mut self, game: &G::PlayerView) -> G::Action;
}

#[derive(Error, Debug)]
#[error("Invalid action: {action:?} for player {player_id:?} with view {view:?}")]
pub struct InvalidActionError<const C: usize, G: Game<C>> {
    pub player_id: G::PlayerId,
    pub action: G::Action,
    pub view: G::PlayerView,
}

#[derive(Error, Debug)]
pub enum PlayError<const C: usize, G: Game<C>> {
    #[error("{0}")]
    InvalidAction(#[from] InvalidActionError<C, G>),
}

pub trait Game<const PLAYER_COUNT: usize> : Debug + Sized + Iterator<Item=Self::PlayerId> {
    type Action: Debug + 'static;

    type PlayerView: Debug + 'static;

    type PlayerId: Copy + Debug + Into<usize> + 'static;

    type Outcome;

    fn try_act(&mut self, action: Self::Action, player_id: Self::PlayerId) -> Result<(), InvalidActionError<PLAYER_COUNT, Self>>;

    fn player_view(&self, player_id: Self::PlayerId) -> Self::PlayerView;

    fn outcome(&self) -> Option<Self::Outcome>;
}

#[derive(Debug, Clone)]
pub struct WithPlayers<const C: usize, G: Game<C>> {
    game: G,
    players: [Box<dyn Actor<C, G>>; C]
}

impl<const C: usize, G: Game<C>> WithPlayers<C, G> {
    fn play(mut self) -> Result<G::Outcome, PlayError<C, G>> {
        loop {
            let player_id = self.game.next().unwrap();
            let action = self.players[player_id.into()].act(&self.game.player_view(player_id));
            self.game.try_act(action, player_id)?;
            if let Some(outcome) = self.game.outcome() {
                return Ok(outcome);
            }
        }
    }
}

impl<const C: usize, G: Game<C> + Default> WithPlayers<C, G> {
    pub fn new(players: [Box<dyn Actor<C, G>>; C]) -> Self {
        Self {
            game: G::default(),
            players
        }
    }
}

pub enum TwoPlayerOutcome {
    OneWins,
    TwoWins,
    Draw
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OutcomeStats {
    one_wins: usize,
    two_wins: usize,
    draws: usize,
}

impl OutcomeStats {
    fn new() -> Self {
        Self::default()
    }

    fn games(&self) -> usize {
        self.one_wins + self.two_wins + self.draws
    }
}

type OutcomeStatsMatrix = Vec<Vec<OutcomeStats>>;

impl<T: Into<TwoPlayerOutcome>, G: Game<2, Outcome = T> + Default> WithPlayers<2, G> {
    pub fn play_matrix(players: Vec<Box<dyn Actor<2, G>>>, game_count: usize) -> Result<OutcomeStatsMatrix, PlayError<2, G>> {
        let mut matrix: Vec<Vec<OutcomeStats>> = players.iter().map(|p1| {
            players.iter().map(|p2| {
                OutcomeStats::new()
            }).collect()
        }).collect();
        for (i, player) in players.clone().into_iter().enumerate() {
            for (j, other_player) in players.clone().into_iter().enumerate() {
                for _ in 0..game_count {
                    let game = WithPlayers::new([player.clone(), other_player.clone()]);
                    match game.play()?.into() {
                        TwoPlayerOutcome::OneWins => matrix[i][j].one_wins += 1,
                        TwoPlayerOutcome::TwoWins => matrix[i][j].two_wins += 1,
                        TwoPlayerOutcome::Draw => matrix[i][j].draws += 1,
                    }
                }
            }
        }
        Ok(matrix)
    }
}

pub fn get_sorted_stats_and_names<G: Game<2> + Default + 'static>(players: &Vec<Box<dyn Actor<2, G>>>, game_count: usize) -> Result<(Vec<String>, OutcomeStatsMatrix), Box<dyn std::error::Error>>
where
    G::Outcome: Into<TwoPlayerOutcome>
{
    let mut player_names: Vec<String> = players.iter().map(|p| p.to_string()).collect();
    let matrix = WithPlayers::<2, G>::play_matrix(players.clone(), game_count)?;
    let mut weight: HashMap<String, usize> = HashMap::new();
    let mut hash_matrix: HashMap<(String, String), OutcomeStats> = HashMap::new();
    for p in players {
        weight.insert(p.to_string(), 0);
    }
    for (i, p1) in players.iter().enumerate() {
        weight.insert(p1.to_string(), 0);
        for (j, p2) in players.iter().enumerate() {
            hash_matrix.insert((p1.to_string(), p2.to_string()), matrix[i][j].clone());
            *weight.get_mut(&*p1.to_string()).unwrap() += 2*matrix[i][j].one_wins + matrix[i][j].draws;
            *weight.get_mut(&*p2.to_string()).unwrap() += 2*matrix[i][j].two_wins + matrix[i][j].draws;
        }
    }
    player_names.sort_by(|p1, p2| {
        weight[&**p1].cmp(&weight[&**p2])
    });
    let matrix: Vec<Vec<OutcomeStats>> = player_names.iter().map(|p1| {
        player_names.iter().map(|p2| {
            hash_matrix[&(p1.clone(), p2.clone())].clone()
        }).collect()
    }).collect();
    Ok((player_names, matrix))
}