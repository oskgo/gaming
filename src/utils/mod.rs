use dyn_clonable::clonable;
use thiserror::Error;
use std::fmt::{Debug, Display};

#[clonable]
pub trait Actor<const C: usize, G: Game<C>>: Display + Debug + Clone {
    fn act(&mut self, game: &G::PlayerView) -> G::Action;
}

#[derive(Error, Debug)]
pub struct InvalidActionError<const C: usize, G: Game<C>> {
    pub player_id: G::PlayerId,
    pub action: G::Action
}

#[derive(Error, Debug)]
pub enum PlayError<const C: usize, G: Game<C>> {
    InvalidAction(#[from] InvalidActionError<C, G>),
}

pub trait Game<const PLAYER_COUNT: usize> : Debug + Sized + Iterator<Item=Self::PlayerId> {
    type Action: Debug;

    type PlayerView;

    type PlayerId: Copy + Debug + Into<usize>;

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

impl<T: Into<TwoPlayerOutcome>, G: Game<2, Outcome = T> + Default> WithPlayers<2, G> {
    pub fn play_matrix(players: Vec<Box<dyn Actor<2, G>>>, game_count: usize) -> Result<Vec<Vec<Option<f64>>>, PlayError<2, G>> {
        let mut matrix = vec![vec![None; players.len()]; players.len()];
        for (i, player) in players.clone().into_iter().enumerate() {
            for (j, other_player) in players.clone().into_iter().enumerate() {
                let mut wins = 0;
                let mut decicives = 0;
                for _ in 0..game_count {
                    let game = WithPlayers::new([player.clone(), other_player.clone()]);
                    match game.play()?.into() {
                        TwoPlayerOutcome::OneWins => {
                            wins += 1;
                            decicives += 1;
                        }
                        TwoPlayerOutcome::TwoWins => decicives += 1,
                        TwoPlayerOutcome::Draw => (),
                    }
                }
                matrix[i][j] = if decicives == 0 {
                    None
                } else {
                    Some(wins as f64 / decicives as f64)
                };
            }
        }
        Ok(matrix)
    }
}