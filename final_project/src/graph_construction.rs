use std::collections::HashMap;

pub struct Graph {
   pub adjacency_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }
    // Add a node (airport) to the graph
    pub fn add_node(&mut self, airport_id: u32) {
        self.adjacency_list.entry(airport_id).or_insert_with(Vec::new);
    }

    // Add a directed edge (route) from one node (airport) to another
    pub fn add_directed_edge(&mut self, from: u32, to: u32) { 
        self.adjacency_list.entry(from).or_default().push(to);
    }
}


