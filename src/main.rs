use core::cmp::min;
use std::collections::BTreeMap;
use std::fs;

const INFINITE_DIST : usize = 100000000;

#[derive(Debug, PartialEq)]
struct Edge
{
    index_second: usize,
    weight: usize,
}

#[derive(Debug, Clone)]
struct GraphNode
{
    index: usize,
    node_name: String,
}

#[derive(Debug, PartialEq)]
struct Graph
{
    number_of_nodes: usize,
    edges: Vec<Vec<Edge>>,
}

#[derive(Debug, Clone, PartialEq)]
struct Node
{
    index: usize,
    parent_idx: usize,
    dist_to_node: usize,
    // dist from start?
}


fn construct_graph_from_edges(graph_nodes: &Vec<GraphNode>, edge_data: &str) -> Graph {
    
    let edges : Vec<&str> = edge_data.split("\n").collect();
    let num_edges : usize = edges[0].parse::<usize>().expect("Expect an integer number of edges.");

    if edges.len() != num_edges + 1 {
        println!("Unexpected number of edges");
    }

    let num_nodes = graph_nodes.len();
  
    let mut vec : Vec<Vec<Edge>> = Vec::with_capacity(num_nodes);
    
    for _ in 0..num_nodes {
        vec.push(Vec::with_capacity(num_nodes));
    }
    let mut graph = Graph{number_of_nodes: graph_nodes.len(), edges: vec};

    for i in 1..(num_edges+1) {
        let edge : Vec<&str> = edges[i].split(" ").collect();
        let start_edge = edge[0];
        let end_edge = edge[1];
        let edge_weight = edge[2].parse::<usize>().expect("");

        let mut start_index = 0;
        let mut end_index = 0;

        for j in graph_nodes {
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
        // todo: make this not dumb
        let mut i = 0;
        let mut old_edge_weight = INFINITE_DIST;
        for e in &graph.edges[start_index] {
            println!("i = {}", i);
            println!("e.index_second == new_edge.index_second, {} == {}", e.index_second, new_edge.index_second);
            if e.index_second == new_edge.index_second {
                old_edge_weight = e.weight;
                break;
            }
            i += 1;
        }

        if old_edge_weight > new_edge.weight {
            println!("removal idx = {}", i);
            if old_edge_weight != INFINITE_DIST {
                graph.edges[start_index].remove(i);
            }
            graph.edges[start_index].push(new_edge);
        
            i = 0;
            for e in &graph.edges[end_index] {
                println!("i = {}", i);
                println!("e.index_second == new_edge.index_second, {} == {}", e.index_second, new_edge_reverse.index_second);
                if e.index_second == new_edge_reverse.index_second {
                    break;
                }
                i += 1;
            }
            if old_edge_weight != INFINITE_DIST {
                graph.edges[end_index].remove(i); 
            }
            graph.edges[end_index].push(new_edge_reverse);
            
        }
       
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

fn get_route(routes_to_find: &str, graph_nodes: Vec<GraphNode>) -> (usize, usize) {
    let routes: Vec<&str> = routes_to_find.split("\n").collect();
    let first_route: Vec<&str>  = routes[0].split(" ").collect(); //todo: other routes
    let start_str = first_route[0];
    let end_str = first_route[1];
    println!("end_str = {}", end_str);
    let mut start_idx = 0;
    let mut end_idx = 0;

    let number_of_nodes = graph_nodes.len();
    for i in 0..number_of_nodes {
        if graph_nodes[i].node_name == start_str {
            start_idx = graph_nodes[i].index;
        }
        if graph_nodes[i].node_name == end_str {
            end_idx = graph_nodes[i].index;
        }
    }
    return (start_idx, end_idx)
}

fn dijkstra(mut start_idx: usize, end_idx: usize, graph: &Graph) -> usize {
    
    let number_of_nodes = graph.number_of_nodes;

    let mut dist = Vec::new();
    for _ in 0..number_of_nodes {
        dist.push(INFINITE_DIST);
    }

    let mut nodes_can_visit : BTreeMap<usize, Node> = BTreeMap::new();
    let mut nodes_visited : Vec<usize> = Vec::new();

    
    dist[start_idx] = 0;
    while start_idx != end_idx {
        
        println!("start_idx = {}; end_idx = {}, current_dist = {}", start_idx, end_idx, dist[start_idx]);
        nodes_visited.push(start_idx);
        
        // which nodes can we visit
        // todo: rather than dont add if contains, check weight and minimise, update parent
        for i in &graph.edges[start_idx] {
            // if present, minimise weight
            // todo: don't use a vec for this, maybe hashmap? 
            if nodes_can_visit.contains_key(&i.index_second) {
                //println!("nodes can visit: {:?}", nodes_can_visit);
                nodes_can_visit.entry(i.index_second).and_modify(|curr_node| curr_node.dist_to_node = min(i.weight + dist[start_idx], curr_node.dist_to_node));
                //println!("nodes can visit: {:?}", nodes_can_visit);
            }
            else if !nodes_visited.contains(&i.index_second) && i.index_second != start_idx {
                // if not present, and we haven't visited the node
                println!("nodes can visit: {:?}", nodes_can_visit);
                nodes_can_visit.insert(i.index_second.clone(), Node{index: i.index_second.clone(), parent_idx:start_idx, dist_to_node:i.weight.clone()});
                println!("nodes can visit: {:?}", nodes_can_visit);
            }
           
        }
        println!("nodes can visit: {:?}", nodes_can_visit);
        // reverse sort
        let mut min_weight = INFINITE_DIST;
        let mut idx = INFINITE_DIST;
        for (i,node) in &nodes_can_visit {
            if node.dist_to_node < min_weight{
                min_weight = node.dist_to_node;
                idx = node.index;
            }
            println!("checking out node {}", i);
        }

       
        
        let closest_node = nodes_can_visit.remove(&idx).unwrap();
        // go to node and update all the connecting nodes.
        println!("closest node: {}", closest_node.index);
        println!("nodes visited: {:?}", nodes_visited);
       
        if (closest_node.index != start_idx) && 
            (!nodes_visited.contains(&closest_node.index)) {
            
            dist[closest_node.index] = dist[closest_node.parent_idx] + closest_node.dist_to_node;
            start_idx = closest_node.index;
            nodes_visited.push(closest_node.index);
        }
        // else continue and remove the next thing from the PQ
    }
    return dist[end_idx];
} 
  

fn main() {
    // read input
    let (node_data, edge_data, routes_to_find) = read_input("src/uk.txt".to_string());
    let graph_nodes : Vec<GraphNode> = get_nodes(&node_data);
    let graph = construct_graph_from_edges(&graph_nodes, &edge_data);
    println!("Graph: {:?}", graph);
    let (start_idx, end_idx) = get_route(&routes_to_find, graph_nodes);
    println!("start_idx = {}; end_idx = {}", start_idx, end_idx);
    let dist = dijkstra(start_idx, end_idx, &graph);
    println!("dist: {}", dist);

    // todo: implement Dijsktra 
    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test_parsing_data(){
        assert_eq!(1,1)
        //todo: add tests for correct parsing of data (low priority)
        // e.g. if num_nodes or num_edges is incorrect
        // e.g. if there are edges to nodes that don't exist
        // e.g. if spacing/formatting of input is incorrect
    }
    #[test]
    fn test_basic_input() {
        let contents = "4\nInverness\nGlasgow\nEdinburgh\nNewcastle\n\n5\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nEdinburgh Newcastle 107\n\nGlasgow Edinburgh\nEdinburgh Inverness\n\n";
        let data : Vec<&str> = contents.split("\n\n").collect();
    
        //let node_data = data[0].to_string();
        //let edge_data = data[1].to_string();
        let routes_to_find = data[2].to_string();

        assert_eq!(routes_to_find, "Glasgow Edinburgh\nEdinburgh Inverness");
    }
    #[test]
    fn test_multiple_start_edges_input() {
        let contents = "4\nInverness\nGlasgow\nEdinburgh\nNewcastle\n\n6\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nEdinburgh Newcastle 107\nInverness Glasgow 17\n\nGlasgow Edinburgh\nEdinburgh Inverness\n\n";
        let data : Vec<&str> = contents.split("\n\n").collect();
    
        let node_data = data[0].to_string();
        let edge_data = data[1].to_string();

        let graph_nodes : Vec<GraphNode> = get_nodes(&node_data);
        let graph = construct_graph_from_edges(&graph_nodes, &edge_data);
        // graph should not contain the Inverness->Glasgow 167 path
        let expected_graph = Graph { number_of_nodes: 4, edges: vec![vec![Edge { index_second: 2, weight: 158 }, Edge { index_second: 1, weight: 17 }], vec![Edge { index_second: 2, weight: 45 }, Edge { index_second: 3, weight: 145 }, Edge { index_second: 0, weight: 17 }], vec![Edge { index_second: 0, weight: 158 }, Edge { index_second: 1, weight: 45 }, Edge { index_second: 3, weight: 107 }], vec![Edge { index_second: 1, weight: 145 }, Edge { index_second: 2, weight: 107 }]] };
        assert_eq!(expected_graph, graph);
    }
    #[test] 
    fn test_route_extraction() {
        let input_line = "Glasgow Edinburgh\nEdinburgh Inverness";
        let graph_nodes = vec![GraphNode { index: 0, node_name: "Inverness".to_string() }, GraphNode { index: 1, node_name: "Glasgow".to_string() }, GraphNode { index: 2, node_name: "Edinburgh".to_string() }, GraphNode { index: 3, node_name: "Newcastle".to_string() }];

        let (start_idx, end_idx) = get_route(input_line, graph_nodes);
        assert_eq!(start_idx, 1);
        assert_eq!(end_idx, 2);

    }
    #[test] 
    fn test_route_finding(){
        assert_eq!(1,1)
        //todo: add tests for correct parsing of data (low priority)
        // e.g. if input file contains multiple edges from A->B with diff weights
        // e.g. if all edges result in a loop
        // e.g. no routes can be found
    }
    #[test] 
    fn test_dijkstra(){
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![Edge{index_second: 1, weight: 2}];
        let edges_from_middle = vec![Edge{index_second: 0, weight: 2}, Edge{index_second: 2, weight: 3}];
        let edges_from_end = vec![Edge{index_second: 1, weight: 3}];

        let graph = Graph{number_of_nodes:3, edges:vec![edges_from_start, edges_from_middle, edges_from_end]};

        let dist = dijkstra(start_idx, end_idx, &graph);
        assert_eq!(dist, 5);
    }
    #[test]
    fn test_multiple_start_edges(){
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![Edge{index_second: 1, weight: 20}, Edge{index_second: 1, weight: 2}];
        let edges_from_middle = vec![Edge{index_second: 0, weight: 2}, Edge{index_second: 2, weight: 3}];
        let edges_from_end = vec![Edge{index_second: 1, weight: 3}];

        let graph = Graph{number_of_nodes:3, edges:vec![edges_from_start, edges_from_middle, edges_from_end]};

        let dist = dijkstra(start_idx, end_idx, &graph);
        assert_eq!(dist, 5);
    }
    #[test]
    fn test_shorter_route_gets_updated(){
        // assuming bidirectionality, now the edge weight for middle->end should be updated from 3 to 2.
  
        let contents = "3\nA\nB\nC\n\n4\nA B 2\nB A 2\nB C 3\nC B 2\n\nA C\n\n";
        let data : Vec<&str> = contents.split("\n\n").collect();
    
        let node_data = data[0].to_string();
        let edge_data = data[1].to_string();

        let graph_nodes : Vec<GraphNode> = get_nodes(&node_data);
        let graph = construct_graph_from_edges(&graph_nodes, &edge_data);
        let expected_graph =  Graph { number_of_nodes: 3, edges: vec![vec![Edge { index_second: 1, weight: 2 }], vec![Edge { index_second: 0, weight: 2 }, Edge { index_second: 2, weight: 2 }], vec![Edge { index_second: 1, weight: 2 }]] };
        assert_eq!(expected_graph, graph);
        let dist = dijkstra(0, 2, &graph);
        assert_eq!(dist, 4);
    }
    //todo: add tests for path finding york to birmingham
    //todo: find all routes; do in parallel - look at threading
    #[test]
    fn test_edges_not_explicitly_in_both_directions(){
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![Edge{index_second: 1, weight: 2}];
        let edges_from_middle = vec![Edge{index_second: 2, weight: 3}];

        let graph = Graph{number_of_nodes:3, edges:vec![edges_from_start, edges_from_middle]};

        let dist = dijkstra(start_idx, end_idx, &graph);
        assert_eq!(dist, 5);
    }
} 