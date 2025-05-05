mod matchtree;
mod isoteam;

use std::error::Error;
use matchtree::read_csv_graph_data;
use isoteam::two_dfs;
use isoteam::team_match_comparison;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "EuropeanData.csv";
    let (teams, graph) = read_csv_graph_data(filename)?;
    println!("{} teams.", teams.len());

    let (isolated_index, distance) = two_dfs(&graph);
    println!("Longest Infrequent Path (inverted weight): {}", distance);
    println!("Most isolated teams from each other:");
    for i in isolated_index {
        println!("{}", teams[i]);
    }

    team_match_comparison(&graph, &teams);

    Ok(())
}
