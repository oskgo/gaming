use std::{error::Error, fmt::Display, ops::Add};

use plotters::{prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, Rectangle, IntoSegmentedCoord, SegmentValue}, style::{WHITE, TextStyle, self, RGBColor, Color, FontTransform, text_anchor::{Pos, HPos, VPos}}};

use super::{Actor, Game, TwoPlayerOutcome, WithPlayers, OutcomeStats, OutcomeStatsMatrix, get_sorted_stats_and_names};

fn outcome_color(stats: OutcomeStats) -> RGBColor {
    let total = stats.games();
    let one_wins_normalized: u8 = (stats.one_wins as f64 / total as f64 * 255.0).round() as u8;
    let two_wins_normalized: u8 = (stats.two_wins as f64 / total as f64 * 255.0).round() as u8;
    RGBColor(two_wins_normalized, one_wins_normalized, 0)
}

fn index_from_segment(segment: &SegmentValue<i32>) -> usize {
    match *segment {
        SegmentValue::Exact(a) => a,
        SegmentValue::CenterOf(a) => a,
        SegmentValue::Last => i32::MAX,
    }.try_into().unwrap()
}

pub fn draw_game_matrix<G: Game<2> + Default + 'static + Display>(
    players: Vec<Box<dyn Actor<2, G>>>,
    game_count: usize,
) -> Result<(), Box<dyn Error>>
where
    G::Outcome: Into<TwoPlayerOutcome>,
{
    let (players, stats) = get_sorted_stats_and_names(&players, game_count)?;
    let n = stats.len();
    let ni = n as i32;
    let game_name = G::default().to_string().to_owned().to_lowercase().replace(|c: char| c.is_whitespace(), "_");
    let out_file_name =  format!("stats/{game_name}.png");
    let root = BitMapBackend::new(&out_file_name, (1024, 1024)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(G::default().to_string(), ("sans-serif", 80))
        .margin(5)
        .x_label_area_size(80)
        .y_label_area_size(80)
        .build_cartesian_2d((0..(ni-1)).into_segmented(), (0..(ni-1)).into_segmented())
        .unwrap();

    chart
        .configure_mesh()
        .x_label_formatter(&|i| {if index_from_segment(i) != n {players[index_from_segment(i)].to_string()} else {"".to_string()}})
        .y_label_formatter(&|i| {if index_from_segment(i) != n {players[index_from_segment(i)].to_string()} else {"".to_string()}})
        .x_labels(n)
        .y_labels(n)
        .max_light_lines(4)
        .disable_x_mesh()
        .disable_y_mesh()
        .x_label_style(
            Into::<TextStyle<'_>>::into(("sans-serif", 20)))
        .y_label_style(("sans-serif", 20))
        .draw()
        .unwrap();



    chart.draw_series(
        (0..ni)
            .map(|x| (0..ni).map(move |y| (x, y)))
            .flatten()
            .map(|(x, y)| {
                Rectangle::new(
                    [(SegmentValue::Exact(x), SegmentValue::Exact(y)), (SegmentValue::Exact(x + 1), SegmentValue::Exact(y + 1))],
                    outcome_color(stats[y as usize][x as usize]).filled(),
                )
            }),
    ).unwrap();

    root.present().expect("Unable to write result to file");
    println!("Result has been saved to {}", out_file_name);
    Ok(())
}
