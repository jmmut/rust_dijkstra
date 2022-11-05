use std::fs;

#[derive(Debug)]
struct Edge
{
    index_second: usize,
    weight: u32,
}

#[derive(Debug)]
struct GraphNode
{
    index: usize,
    node_name: String,
}

#[derive(Debug)]
struct Graph
{
    number_of_nodes: usize,
    edges: Vec<Vec<Edge>>,
}


fn construct_graph_from_edges(graph_nodes: Vec<GraphNode>, edge_data: &str) -> Graph {
    
    let edges : Vec<&str> = edge_data.split("\n").collect();
    let num_edges : usize = edges[0].parse::<usize>().expect("Expect an integer number of edges.");

    if edges.len() != num_edges + 1 {
        println!("Unexpected number of edges");
    }

    let num_nodes = graph_nodes.len();
  
    let mut vec : Vec<Vec<Edge>> = Vec::with_capacity(num_nodes);
    
    for i in 0..num_nodes {
        vec.push(Vec::with_capacity(num_nodes));
    }
    let mut graph = Graph{number_of_nodes: graph_nodes.len(), edges: vec};

    for i in 1..(num_edges+1) {
        let edge : Vec<&str> = edges[i].split(" ").collect();
        let start_edge = edge[0];
        let end_edge = edge[1];
        let edge_weight = edge[2].parse::<u32>().expect("");

        let mut start_index = 0;
        let mut end_index = 0;

        for j in &graph_nodes {
            if j.node_name == start_edge {
                start_index = j.index;
            }
            if j.node_name == end_edge {
                end_index = j.index;
            }
        }
        let new_edge = Edge{index_second:end_index , weight:edge_weight};
        let new_edge_reverse = Edge{index_second:start_index, weight:edge_weight};

        // create Edge and add to graph.
        graph.edges[start_index].push(new_edge);
        graph.edges[end_index].push(new_edge_reverse)
    }
    
    return graph;

}

fn get_nodes(node_data: &str) -> Vec<GraphNode> {

    let nodes : Vec<&str> = node_data.split("\n").collect();
    let num_nodes : usize = nodes[0].parse::<usize>().expect("Expect an integer number of nodes.");

    if nodes.len() != num_nodes + 1 {
        println!("Unexpected number of nodes");
    }

    let mut graph_nodes = Vec::with_capacity(num_nodes);

    for i in 1..(num_nodes+1) {
        graph_nodes.push(GraphNode{index: i-1, node_name: nodes[i].to_string() });
    }

    println!("graph nodes: {:?}", graph_nodes);
    return graph_nodes
}



fn read_input(filepath: String) -> (String, String, String) {
    let contents = fs::read_to_string(filepath)
    .expect("Should have been able to read the file");

    let data : Vec<&str> = contents.split("\n\n").collect();
    
    let node_data = data[0].to_string();
    let edge_data = data[1].to_string();
    let routes_to_find = data[2].to_string();

    return (node_data, edge_data, routes_to_find);

}

fn main() {
    // read input
    let (node_data, edge_data, routes_to_find) = read_input("src/uk.txt".to_string());
    let graph_nodes : Vec<GraphNode> = get_nodes(&node_data);
    let graph = construct_graph_from_edges(graph_nodes, &edge_data);
    println!("graph: {:?}", graph);

    // todo: implement Dijsktra 
    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
        fn test_parsing_data(){
            assert_eq!(1,1)
            //todo: add tests for correct parsing of data (low priority)
        }
    } 