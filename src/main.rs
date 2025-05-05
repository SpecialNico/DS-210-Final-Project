mod matchtree;
mod isoteam;

use std::error::Error;
use matchtree::read_csv_graph_data;
//use isoteam::two_dfs;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "EuropeanData.csv";

    // Load team names & weighted graph
    let (names, graph) = read_csv_graph_data(filename)?;
    println!("Loaded {} teams.", names.len());

    
}
