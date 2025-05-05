use crate::matchtree::Graph;

fn dfs_weighted(
    u: usize,
    graph: &Graph,
    cur_dist: f64,
    visited: &mut Vec<bool>,
    distance: &mut Vec<f64>,
) {
    for &(v, w) in &graph.outedges[u] {
        if !visited[v] && w > 0 {
            visited[v] = true;
            let cost = 1.0 / (w as f64);
            let nd = cur_dist + cost;
            distance[v] = nd;
            dfs_weighted(v, graph, nd, visited, distance);
        }
    }
}

/// Two‑DFS using inverted weights (1.0 / weight) to approximate diameter:
pub fn two_dfs(graph: &Graph) -> (Vec<usize>, f64) {
    let mut visited = vec![false; graph.n];
    let mut distance = vec![0.0; graph.n];
    visited[0] = true;
    dfs_weighted(0, graph, 0.0, &mut visited, &mut distance);

    let farthest = (0..graph.n).max_by(|&a, &b| distance[a].partial_cmp(&distance[b]).unwrap()).unwrap();

    let mut visited2 = vec![false; graph.n];
    let mut distance2 = vec![0.0; graph.n];
    visited2[farthest] = true;
    dfs_weighted(farthest, graph, 0.0, &mut visited2, &mut distance2);

    let max_dist = distance2
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let teams = distance2
        .iter()
        .enumerate()
        .filter(|&(_, &d)| (d - max_dist).abs() < 1e-6)
        .map(|(i, _)| i)
        .collect();

    (teams, max_dist)
}

pub fn team_match_comparison(graph: &Graph, team_names: &Vec<String>) {
    let mut total_matches = 0;
    let mut min_matches = usize::MAX;
    let mut max_matches = 0;
    let mut min_team = 0;
    let mut max_team = 0;

    for (i, neighbors) in graph.outedges.iter().enumerate() {
        let match_count = neighbors.len();
        total_matches += match_count;

        if match_count < min_matches {
            min_matches = match_count;
            min_team = i;
        }

        if match_count > max_matches {
            max_matches = match_count;
            max_team = i;
        }
    }

    let avg_matches = total_matches as f64 / graph.n as f64;
    println!("------Team Match Comparison-------");
    println!("Team w/ fewest matches: '{}' ({} matches)", team_names[min_team], min_matches);
    println!("Team w/ most matches: '{}' ({} matches)", team_names[max_team], max_matches);
    println!("Average # matches per team: {:.2}", avg_matches);
}
