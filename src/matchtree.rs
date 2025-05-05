//Module matchtree.rs, holds the logic that creates the undirected 
//weighted graph from the data points held in the European Soccer csv file, as well as reads the file

use std::error::Error;
use std::collections::HashMap;
use csv::ReaderBuilder;

pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex, Vertex, usize)>;
pub type AdjacencyLists = Vec<Vec<(Vertex, usize)>>;

#[derive(Debug)]
pub struct Graph {    //Graph struct, defines number of nodes as usize and edges between noes as Adjacency List
    pub n: usize,
    pub outedges: AdjacencyLists,
}

impl Graph {
    //creates edges between adjacency lists u and v (from lecture)
    pub fn add_undirected_edges(&mut self, edges: &ListOfEdges) {
        for &(u, v, w) in edges {
            self.outedges[u].push((v, w));
            self.outedges[v].push((u, w));
        }
    }
    // iterates through every adjacency list in self.outedges based on ID in ascending order
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort_by(|a, b| a.0.cmp(&b.0));  //sorts
        }
    }
    //combines the two previous functions’ utilizations in order to create an undirected graph
    pub fn create_undirected(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n, 
            outedges: vec![vec![]; n],};
        g.add_undirected_edges(edges);
        g.sort_graph_lists();
        g
    }
}
//creates HashMap that forms the edges based on unique IDs of teams
//in the data (parsed), which are then used in create_undirected() to create the undirected graph
pub fn data_read(filename: &str) -> Result<(Vec<String>, Graph), Box<dyn Error>> {
    println!("Reading CSV: {}", filename);
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .flexible(true)
        .from_path(filename)?;

    let mut team_getid: HashMap<String, Vertex> = HashMap::new();
    let mut id_getteam: Vec<String> = Vec::new();
    let mut map_edges: HashMap<(Vertex, Vertex), usize> = HashMap::new();

    for result in rdr.records() {
        let r = result?;
        let team = r.get(3).unwrap().to_string();      
        let opponent  = r.get(6).unwrap().to_string();    //parses on 4th and 7th column for team and opponent name

        let mut a;
        if let Some(id) = team_getid.get(&team) {
            a = *id; 
        } else {
            let new_id = id_getteam.len(); 
            team_getid.insert(team.clone(), new_id); 
            id_getteam.push(team.clone());      //ensures IDs are unique for each team
            a = new_id; 
        }

        let mut b;
        if let Some(id) = team_getid.get(&opponent) {
            b = *id; 
        } else {
            let new_id = id_getteam.len(); 
            team_getid.insert(opponent.clone(), new_id); 
            id_getteam.push(opponent.clone()); 
            b = new_id; 
        }
        let key = if a <= b {(a, b)} else {(b, a)};
        *map_edges.entry(key).or_insert(0) += 1;   
    }

    let edges: ListOfEdges = map_edges
        .into_iter()
        .map(|((u, v), w)| (u, v, w))
        .collect();

    let n = id_getteam.len();
    let graph = Graph::create_undirected(n, &edges);
    Ok((id_getteam, graph))
}
