use std::collections::{HashMap, HashSet, VecDeque};
use crate::graph_construction::Graph;


// Function to calculate degree centrality
pub fn degree_centrality(graph: &Graph) -> HashMap<u32, usize> {
    let mut centrality_scores = HashMap::new();

    for (node, edges) in &graph.adjacency_list {
        centrality_scores.insert(*node, edges.len());
    }

    centrality_scores
}

pub fn bfs(graph: &Graph, start: u32, end: u32) -> Vec<Vec<u32>> {
    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(vec![start]);
    visited.insert(start);

    while let Some(current_path) = queue.pop_front() {
        let last_node = *current_path.last().unwrap();

        if last_node == end {
            paths.push(current_path);
            continue;
        }

        if let Some(neighbors) = graph.adjacency_list.get(&last_node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);

                    let mut new_path = current_path.clone();
                    new_path.push(neighbor);

                    queue.push_back(new_path);
                }
            }
        }
    }

    paths
}

// Function to calculate betweenness centrality
pub fn betweenness_centrality(graph: &Graph) -> HashMap<u32, f64> {
    let mut centrality_scores = HashMap::new();

    // Initialize centrality scores
    for &node in graph.adjacency_list.keys() {
        centrality_scores.insert(node, 0.0);
    }

    // Iterate over all pairs of nodes
    for &start in graph.adjacency_list.keys() {
        for &end in graph.adjacency_list.keys() {
            if start != end {
                let all_paths = bfs(graph, start, end);

                for path in all_paths {
                    for &node in &path[1..path.len() - 1] { // Exclude start and end nodes
                        if let Some(score) = centrality_scores.get_mut(&node) {
                            *score += 1.0; // Increment the score
                        }
                    }
                }
            }
        }
    }

    centrality_scores
}