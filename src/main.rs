use core::cmp::min;
use std::collections::BTreeMap;
use std::fs;
include!("construct_graph.rs");

#[derive(Debug, Clone, PartialEq)]
struct Node
{
    index: usize,
    parent_idx: usize,
    dist_to_node: usize,
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