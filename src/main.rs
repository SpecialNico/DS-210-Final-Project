//main.rs holds the calling and testing of functions

mod matchtree;
mod isolatedteams;
use crate::matchtree::ListOfEdges;

use std::error::Error;
use matchtree::{data_read, Graph};
use isolatedteams::{final_dfs, dfs_weighted};
use isolatedteams::match_num_comparison;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "EuropeanData.csv";
    let (teams, graph) = data_read(filename)?;
    println!("{} teams.", teams.len());

    let (isolated_index, distance) = final_dfs(&graph);
    println!("Longest Infrequent Path (inverted weight): {}", distance);
    println!("Most isolated teams from each other:");
    for i in isolated_index {
        println!("{}", teams[i]);
    }

    match_num_comparison(&graph, &teams);

    Ok(())
}

#[test]
fn test_graph_creation() {
    let edges: ListOfEdges = vec![
        (0, 1, 3), 
        (1, 2, 2), 
        (2, 0, 1), 
    ];
    let graph = Graph::create_undirected(3, &edges);

    assert_eq!(graph.n, 3);
    assert_eq!(graph.outedges[0].len(), 2);
    assert_eq!(graph.outedges[1].len(), 2);
    assert_eq!(graph.outedges[2].len(), 2);

    assert_eq!(graph.outedges[0][0], (1, 3)); // Team 0 → Team 1 (3 matches)
    assert_eq!(graph.outedges[0][1], (2, 1)); // Team 0 → Team 2 (1 match)
}

#[test]
fn test_path() {
    let edges: ListOfEdges = vec![
        (0, 1, 10), 
        (1, 2, 2), 
        (2, 3, 1),  
        (3, 4, 1), 
    ];

    let max_vertex = edges.iter().flat_map(|&(u, v, _)| vec![u, v]).max().unwrap_or(0);
    let n = max_vertex + 1; 
    let graph = Graph::create_undirected(n, &edges);

    let (isolated_teams, max_dist) = final_dfs(&graph);

    assert!(
        isolated_teams.len() >= 1,
        "Expected at least 1 isolated team, but got {:?}",
        isolated_teams
    );

    assert!(
        max_dist > 0.0,
        "Expected max_dist to be positive, but got {}",
        max_dist
    );
}
