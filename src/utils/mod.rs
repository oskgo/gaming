use thiserror::Error;
use std::fmt::Debug;

pub trait Actor<const C: usize, G: Game<C>> {
    fn act(&mut self, game: &G::PlayerView) -> G::Action;
}

#[derive(Error, Debug)]
pub struct InvalidActionError<const C: usize, G: Game<C>> {
    player: G::PlayerId,
    action: G::Action
}

#[derive(Error, Debug)]
pub enum PlayError<const C: usize, G: Game<C>> {
    InvalidAction(#[from] InvalidActionError<C, G>),
}

pub trait Game<const PLAYER_COUNT: usize> : Sized + Iterator<Item=Self::PlayerId> {
    type Action: Debug;

    type PlayerView;

    type PlayerId: Copy + Debug + Into<usize>;

    type Outcome;

    fn try_act(&mut self, action: Self::Action, player_id: Self::PlayerId) -> Result<(), InvalidActionError<PLAYER_COUNT, Self>>;

    fn player_view(&self, player_id: Self::PlayerId) -> Self::PlayerView;

    fn outcome(&self) -> Option<Self::Outcome>;
}

struct WithPlayers<const C: usize, G: Game<C>> {
    game: G,
    players: [Box<dyn Actor<C, G>>; C]
}

impl<const C: usize, G: Game<C>> WithPlayers<C, G> {
    fn play(&mut self) -> Result<G::Outcome, PlayError<C, G>> {
        loop {
            let mut player_id = self.game.next().unwrap();
            let action = self.players[player_id.into()].act(&self.game.player_view(player_id));
            self.game.try_act(action, player_id)?;
            if let Some(outcome) = self.game.outcome() {
                return Ok(outcome);
            }
        }
    }
}

impl<const C: usize, G: Game<C> + Default> WithPlayers<C, G> {
    fn new(players: [Box<dyn Actor<C, G>>; C]) -> Self {
        Self {
            game: G::default(),
            players
        }
    }
}