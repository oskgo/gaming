use gaming::games::rock_paper_scissors;

fn main() {
    println!("{:?}", rock_paper_scissors::game_matrix(10000).unwrap());
}
