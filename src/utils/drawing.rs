use plotters::backend::{DrawingBackend, self};

use super::{Actor, Game, TwoPlayerOutcome, WithPlayers};

fn draw_game_matrix<Backend: DrawingBackend, G: Game<2> + Default + 'static>(
    backend: &Backend,
    players: Vec<Box<dyn Actor<2, G>>>,
    game_count: usize,
) -> Result<(), Box<dyn std::error::Error>>
where
    G::Outcome: Into<TwoPlayerOutcome>,
{
    let player_names: Vec<String> = players.iter().map(|p| p.to_string()).collect();
    let matrix = WithPlayers::<2, G>::play_matrix(players.clone(), game_count)?;
    //let heights: Vec<u32> = players.iter().map(|p| {
    //    backend.estimate_text_size(p.to_string(), style)
    //}).collect();
    todo!()
}
