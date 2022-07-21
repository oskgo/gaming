use crate::utils::{Game, Actor};

#[derive(Debug, Copy, Clone)]
enum Player {
    One,
    Two
}

impl From<Player> for usize {
    fn from(player: Player) -> Self {
        match player {
            Player::One => 0,
            Player::Two => 1
        }
    }
}

enum Outcome {
    OneWins,
    TwoWins
}

enum HasFaceDown {
    Yes,
    No
}

struct Card;

struct MidGameView {
    current_player: Player,
    player_one_cards: [Option<(Card, HasFaceDown)>; 8],
    player_two_cards: [Option<(Card, HasFaceDown)>; 8],
}

struct SelectTrumpView {
    cards: [Card; 4]
}

enum View {
    MidGame(MidGameView),
    SelectTrump(SelectTrumpView)
}

struct MidGameState {
    current_player: Player,
    player_one_cards: [Option<(Card, Option<Card>)>; 8],
    player_two_cards: [Option<(Card, Option<Card>)>; 8],
}

struct SelectTrumpState {
    cards: [Card; 4]
}

enum State {
    MidGame(MidGameState),
    SelectTrump(SelectTrumpState)
}

struct OffiziersSkat {
    state: State,
    players: [Box<dyn Actor<2, OffiziersSkat>>; 2]
}

impl Iterator for OffiziersSkat {
    type Item = Player;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::MidGame(ref mut state) => {
                let old_player = state.current_player;
                state .current_player = match state.current_player {
                    Player::One => Player::Two,
                    Player::Two => Player::One
                };
                Some(old_player)
            }
            State::SelectTrump(ref mut state) => {
                Some(Player::One)
            }
        }
    }
}

type Action = ();

impl Game<2> for OffiziersSkat {
    type Action = Action;

    type PlayerView = View;

    type PlayerId = Player;

    type Outcome = Outcome;

    fn try_act(&mut self, action: Self::Action, player_id: Self::PlayerId) -> Result<(), crate::utils::InvalidActionError<2, Self>> {
        todo!()
    }

    fn player_view(&self, player_id: Self::PlayerId) -> Self::PlayerView {
        todo!()
    }

    fn outcome(&self) -> Option<Self::Outcome> {
        todo!()
    }
}