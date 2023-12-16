mod read_csv;
mod graph_construction;
mod graph_analysis;

use read_csv::read_file;
use std::collections::HashMap;
use graph_construction::Graph;
use graph_analysis::{degree_centrality, betweenness_centrality};

pub fn find_top_5_betweenness(scores: &HashMap<u32, f64>) -> Vec<(u32, f64)> {
    let mut score_vec: Vec<_> = scores.iter().collect();
    score_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    score_vec.into_iter().take(5).map(|(&node, &score)| (node, score)).collect()
}

fn find_top_5_degree(scores: &HashMap<u32, usize>) -> Vec<(u32, usize)> {
    let mut score_vec: Vec<_> = scores.iter().collect();
    score_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    score_vec.into_iter().take(5).map(|(&node, &score)| (node, score)).collect()
}

fn main() {
    let file_path = "airplane.csv";

    // Read data from CSV file
    let routes = read_file(file_path).expect("Failed to read file");

    // Construct graph
    let mut graph = Graph::new();
    for route in routes {
        graph.add_node(route.from);
        graph.add_node(route.to);
        graph.add_directed_edge(route.from, route.to);
    }

    // Calculate degree centrality
    let centrality_scores = degree_centrality(&graph);
    let betweenness_scores = betweenness_centrality(&graph);
   
    for (airport_id, score) in &centrality_scores {
        println!("Airport {}: Degree Centrality = {}", airport_id, score);
    }
    for (airport_id, score) in &betweenness_scores {
        println!("Airport {}: Betweenness Centrality = {}", airport_id, score);
    }
    // Function to find top 5 nodes with highest scores

    let top_5_betweenness = find_top_5_betweenness(&betweenness_scores);
    let top_5_degree = find_top_5_degree(&centrality_scores);

    println!("Top 5 Betweenness Centrality Scores:");
    for (node, score) in top_5_betweenness {
    println!("Node {}: {}", node, score);
    }

    println!("Top 5 Degree Centrality Scores:");
    for (node, score) in top_5_degree {
    println!("Node {}: {}", node, score);
    }   
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smaller_csv() {
        let file_path = "test.csv";

        // Read data from CSV file
        let routes = read_file(file_path).expect("Failed to read file");

        // Construct graph
        let mut graph = Graph::new();
        for route in routes {
            graph.add_node(route.from);
            graph.add_node(route.to);
            graph.add_directed_edge(route.from, route.to);
        }

        // Calculate degree centrality and betweenness centrality
        let centrality_scores = degree_centrality(&graph);
        let betweenness_scores = betweenness_centrality(&graph);

        // Function to find top 5 nodes with highest scores
        let top_5_betweenness = find_top_5_betweenness(&betweenness_scores);
        let top_5_degree = find_top_5_degree(&centrality_scores);

        // Assertions with expected values
        assert_eq!(top_5_betweenness[0], (6, 6.0));
        assert_eq!(top_5_betweenness[1], (3, 5.0));
        assert_eq!(top_5_betweenness[2].1, 3.0);

        for (_, score) in top_5_degree {
            assert_eq!(score, 1);
        }
    }
}
