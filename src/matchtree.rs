use std::error::Error;
use std::collections::HashMap;
use csv::ReaderBuilder;

pub type Vertex = usize;
// (team_idx, opp_idx, match_count)
pub type ListOfEdges = Vec<(Vertex, Vertex, usize)>;
// adjacency list: (neighbor_idx, match_count)
pub type AdjacencyLists = Vec<Vec<(Vertex, usize)>>;

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
    pub max_weight: usize,
}

impl Graph {
    pub fn add_undirected_edges(&mut self, edges: &ListOfEdges) {
        for &(u, v, w) in edges {
            self.outedges[u].push((v, w));
            self.outedges[v].push((u, w));
        }
    }

    pub fn sort_graph_lists(&mut self) {
        for nbrs in &mut self.outedges {
            nbrs.sort_by_key(|&(v, _)| v);
        }
    }

    pub fn create_undirected(n: usize, edges: &ListOfEdges) -> Graph {
        let max_w = edges.iter().map(|&(_, _, w)| w).max().unwrap_or(1);
        let mut g = Graph {
            n,
            outedges: vec![Vec::new(); n],
            max_weight: max_w,
        };
        g.add_undirected_edges(edges);
        g.sort_graph_lists();
        g
    }
}

/// Reads your CSV (with headers), where:
///  - Team is in column 4 (zero‑based index 3)
///  - Opponent is in column 7 (zero‑based index 6)
/// Counts repeats as edge weights.
pub fn read_csv_graph_data(
    filename: &str,
) -> Result<(Vec<String>, Graph), Box<dyn Error>> {
    println!("Reading CSV: {}", filename);
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .flexible(true)
        .from_path(filename)?;

    let mut team_idx: HashMap<String, Vertex> = HashMap::new();
    let mut idx_team: Vec<String> = Vec::new();
    let mut edge_map: HashMap<(Vertex, Vertex), usize> = HashMap::new();

    for result in rdr.records() {
        let rec = result?;
        let team = rec.get(3).unwrap().to_string();      
        let opp  = rec.get(6).unwrap().to_string();    

        let u = *team_idx.entry(team.clone()).or_insert_with(|| {
            idx_team.push(team.clone());
            idx_team.len() - 1
        });
        let v = *team_idx.entry(opp.clone()).or_insert_with(|| {
            idx_team.push(opp.clone());
            idx_team.len() - 1
        });

        let key = if u <= v { (u, v) } else { (v, u) };
        *edge_map.entry(key).or_insert(0) += 1;
    }

    let edges: ListOfEdges = edge_map
        .into_iter()
        .map(|((u, v), w)| (u, v, w))
        .collect();

    let n = idx_team.len();
    let graph = Graph::create_undirected(n, &edges);
    Ok((idx_team, graph))
}
