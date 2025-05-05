use crate::matchtree::Graph;

pub fn dfs_weighted(
    u: usize,
    graph: &Graph,
    current_dist: f64,
    visited: &mut Vec<bool>,
    dist: &mut Vec<f64>,
) {
    for &(v, w) in &graph.outedges[u] {
        if !visited[v] && w > 0 {
            visited[v] = true;
            let inv_weight = 1.0 / (w as f64);
            let next_dist = current_dist + inv_weight;
            dist[v] = next_dist;
            dfs_weighted(v, graph, next_dist, visited, dist);
        }
    }
}

pub fn final_dfs(graph: &Graph) -> (Vec<usize>, f64) {
    let mut visited = vec![false; graph.n];
    let mut dist = vec![0.0; graph.n];
    visited[0] = true;
    dfs_weighted(0, graph, 0.0, &mut visited, &mut dist);

    let farthest = (0..graph.n)
        .max_by(|&a, &b| dist[a].partial_cmp(&dist[b]).unwrap())
        .unwrap();

    let mut visited2 = vec![false; graph.n];
    let mut dist2 = vec![0.0; graph.n];
    visited2[farthest] = true;
    dfs_weighted(farthest, graph, 0.0, &mut visited2, &mut dist2);

    let mut max_dist = dist2[0]; 

    for &d in &dist2 {
        if d > max_dist {
            max_dist = d;
        }
    }
    
    let mut teams = Vec::new();
    for (i, &d) in dist2.iter().enumerate() {
        if d == max_dist {
            teams.push(i);
        }
    }

    (teams, max_dist)
}

pub fn match_num_comparison(graph: &Graph, teams: &Vec<String>) {
    let mut total_matches = 0;
    let mut least_matches = usize::MAX;
    let mut most_matches = 0;
    let mut team_least = 0;
    let mut team_most = 0;

    for (i, neigh) in graph.outedges.iter().enumerate() {
        let match_count = neigh.len();
        total_matches += match_count;

        if match_count < least_matches {
            least_matches = match_count;
            team_least = i;
        }

        if match_count > most_matches {
            most_matches = match_count;
            team_most = i;
        }
    }

    let match_avg = total_matches as f64 / graph.n as f64;
    println!("------Team Match Comparison-------");
    println!("Team w/ fewest matches: '{}' ({} matches)", teams[team_least], least_matches);
    println!("Team w/ most matches: '{}' ({} matches)", teams[team_most], most_matches);
    println!("Average # matches per team: {:.2}", match_avg);
}
