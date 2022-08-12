use gaming::{games::rock_paper_scissors, utils::{get_sorted_stats_and_names, drawing::draw_game_matrix}};
use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, Rectangle, IntoSegmentedCoord},
    style::{self, Color, HSLColor, RGBColor, TextStyle, WHITE},
};

fn main() {
    draw_game_matrix(rock_paper_scissors::players(), 1).unwrap();
}
