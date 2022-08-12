use gaming::{games::rock_paper_scissors, utils::drawing::draw_game_matrix};

fn main() {
    draw_game_matrix(rock_paper_scissors::players(), 1000).unwrap();
}
