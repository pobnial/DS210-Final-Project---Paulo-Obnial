
pub fn test_it(){
    let contents = fs::read_to_string("data.txt").expect("Error reading file");
    let lines: Vec<&str> = contents.lines().collect();

    let route = lines[0].parse::<usize>().unwrap();

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    // Construct graph
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