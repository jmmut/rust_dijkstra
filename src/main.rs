use core::cmp::min;
use std::collections::BTreeMap;
use std::fs;
include!("construct_graph.rs");

#[derive(Debug, Clone, PartialEq)]
struct Node {
    index: usize,
    parent_idx: usize,
    dist_to_node: usize,
}

fn get_route_travelled(original_start_idx: usize, end_idx: usize, parents_of_nodes_visited: Vec<usize>) -> Vec<usize> {
    //go backwards through the nodes to find the parent node.
    let mut idx = end_idx;
    let mut nodes_in_order : Vec<usize> = Vec::new();
    nodes_in_order.push(end_idx);
    while idx != original_start_idx {
        idx = parents_of_nodes_visited[idx];
        nodes_in_order.push(idx);
    }

    nodes_in_order.reverse();
    if cfg!(debug_assertions) {
        println!("nodes: {:?}", nodes_in_order);
    }

    return nodes_in_order
}

fn get_human_readable_route(nodes_in_order: Vec<usize>, graph_nodes: Vec<GraphNode>) -> Result<Vec<String>, String> {

    let mut path_travelled : Vec<String> = Vec::new();
    for node_idx in nodes_in_order {
        let node  = &graph_nodes[node_idx];

        if node.index != node_idx {
            return Err("Error in the indexing for the route travelled.".to_string());
        }
        else {
            path_travelled.push(node.node_name.to_string());
        }
    }
    return Ok(path_travelled)
}

fn print_route(route: Vec<String>) -> String {
    let mut final_path : String = route[0].to_string();
    for i in 1..route.len() {
        final_path = format!("{}->{}", final_path, route[i]);
    }

    return final_path
}

fn dijkstra(mut start_idx: usize, end_idx: usize, graph: &Graph) -> (usize, Vec<usize>) {
    let original_start_idx = start_idx;
    let number_of_nodes = graph.number_of_nodes;

    let mut dist = Vec::new();
    for _ in 0..number_of_nodes {
        dist.push(INFINITE_DIST);
    }

    let mut nodes_can_visit: BTreeMap<usize, Node> = BTreeMap::new();
    let mut nodes_visited: Vec<usize> = Vec::new();
    // for now, store the parents in a separate vector, with the idx being the idx of the child.
    // todo; store the node rather than two separate vectors
    let mut parents_of_nodes_visited: Vec<usize> = Vec::with_capacity(number_of_nodes);
    for _ in 0..number_of_nodes {
        parents_of_nodes_visited.push(INFINITE_DIST);
    }

    dist[start_idx] = 0;
    while start_idx != end_idx {
        nodes_visited.push(start_idx);

        // which nodes can we visit
        for i in &graph.edges[start_idx] {
            // if present, minimise weight
            if nodes_can_visit.contains_key(&i.index_second) {
                nodes_can_visit
                    .entry(i.index_second)
                    .and_modify(|curr_node| {
                        curr_node.dist_to_node =
                            min(i.weight + dist[start_idx], curr_node.dist_to_node)
                    });
            } else if !nodes_visited.contains(&i.index_second) && i.index_second != start_idx {
                // if not present, and we haven't visited the node
                nodes_can_visit.insert(
                    i.index_second.clone(),
                    Node {
                        index: i.index_second.clone(),
                        parent_idx: start_idx,
                        dist_to_node: i.weight.clone(),
                    },
                );
            }
        }
        if cfg!(debug_assertions) {
            println!("nodes can visit: {:?}", nodes_can_visit);
        }

        // reverse sort
        let mut min_weight = INFINITE_DIST;
        let mut idx = INFINITE_DIST;
        for (i, node) in &nodes_can_visit {
            if node.dist_to_node < min_weight {
                min_weight = node.dist_to_node;
                idx = node.index;
            }
        }
        let closest_node = nodes_can_visit.remove(&idx).unwrap();

        if (closest_node.index != start_idx) && (!nodes_visited.contains(&closest_node.index)) {
            dist[closest_node.index] = dist[closest_node.parent_idx] + closest_node.dist_to_node;
            start_idx = closest_node.index;
            nodes_visited.push(closest_node.index);
            parents_of_nodes_visited[closest_node.index] = closest_node.parent_idx;
        }

    }

    let nodes_in_order = get_route_travelled(original_start_idx, end_idx, parents_of_nodes_visited);

    return (dist[end_idx], nodes_in_order);
}

fn main() -> Result<(), String> {
    // read input
    let filepath = "src/uk.txt".to_string();
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    let data = read_input(contents);
    if let Err(e) = data {
        return Err(format!("Graph construction failed due to {e}", e=e));
    }
    let (node_data, edge_data, routes_to_find) = data.unwrap();
    let graph_nodes: Vec<GraphNode> = get_nodes(&node_data);
    let graph_result = construct_graph_from_edges(&graph_nodes, &edge_data);
    if let Err(e) = graph_result {
        return Err(format!("Graph construction failed due to {e}", e=e));
    }
    let graph = graph_result.unwrap();
    if cfg!(debug_assertions) {
        println!("graph: {:?}", graph);
    }
    let route_result = get_route(&routes_to_find, &graph_nodes);
    if let Err(e) = route_result {
        return Err(format!("Graph construction failed due to incorrect route; {e}", e=e));
    }
    let (start_idx, end_idx) = route_result.unwrap();
    if cfg!(debug_assertions) {
        println!("finding route from {} to {}", start_idx, end_idx);
    }
    let (dist, route) = dijkstra(start_idx, end_idx, &graph);
    let human_readable_route = get_human_readable_route(route, graph_nodes);
    if let Err(e) = human_readable_route {
        return Err(format!("Something went wrong with indexing the nodes. {e}", e=e));
    }
    println!("Route travelled: {}", print_route(human_readable_route.unwrap()));
    println!("Dist: {}", dist);
    //todo: find all routes; do in parallel - look at threading
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dijkstra() {
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![Edge {
            index_second: 1,
            weight: 2,
        }];
        let edges_from_middle = vec![
            Edge {
                index_second: 0,
                weight: 2,
            },
            Edge {
                index_second: 2,
                weight: 3,
            },
        ];
        let edges_from_end = vec![Edge {
            index_second: 1,
            weight: 3,
        }];

        let graph = Graph {
            number_of_nodes: 3,
            edges: vec![edges_from_start, edges_from_middle, edges_from_end],
        };

        let (dist, _) = dijkstra(start_idx, end_idx, &graph);
        assert_eq!(dist, 5);
    }
    #[test]
    fn test_multiple_start_edges() {
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![
            Edge {
                index_second: 1,
                weight: 20,
            },
            Edge {
                index_second: 1,
                weight: 2,
            },
        ];
        let edges_from_middle = vec![
            Edge {
                index_second: 0,
                weight: 2,
            },
            Edge {
                index_second: 2,
                weight: 3,
            },
        ];
        let edges_from_end = vec![Edge {
            index_second: 1,
            weight: 3,
        }];

        let graph = Graph {
            number_of_nodes: 3,
            edges: vec![edges_from_start, edges_from_middle, edges_from_end],
        };

        let (dist, _) = dijkstra(start_idx, end_idx, &graph);
        assert_eq!(dist, 5);
    }
    #[test]
    fn test_shorter_initial_route_gets_updated() {
        // assuming bidirectionality, now the edge weight for middle->end should be updated from 3 to 2.

        let contents = "3\nA\nB\nC\n\n4\nA B 2\nB A 2\nB C 3\nC B 2\n\nA C\n\n";
        let data: Vec<&str> = contents.split("\n\n").collect();

        let node_data = data[0].to_string();
        let edge_data = data[1].to_string();

        let graph_nodes: Vec<GraphNode> = get_nodes(&node_data);
        let graph = construct_graph_from_edges(&graph_nodes, &edge_data).expect("");
        let expected_graph = Graph {
            number_of_nodes: 3,
            edges: vec![
                vec![Edge {
                    index_second: 1,
                    weight: 2,
                }],
                vec![
                    Edge {
                        index_second: 0,
                        weight: 2,
                    },
                    Edge {
                        index_second: 2,
                        weight: 2,
                    },
                ],
                vec![Edge {
                    index_second: 1,
                    weight: 2,
                }],
            ],
        };
        assert_eq!(expected_graph, graph);
        let (dist, _) = dijkstra(0, 2, &graph);
        assert_eq!(dist, 4);
    }
    #[test]
    fn test_edges_not_explicitly_in_both_directions() {
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![Edge {
            index_second: 1,
            weight: 2,
        }];
        let edges_from_middle = vec![Edge {
            index_second: 2,
            weight: 3,
        }];

        let graph = Graph {
            number_of_nodes: 3,
            edges: vec![edges_from_start, edges_from_middle],
        };

        let (dist, _) = dijkstra(start_idx, end_idx, &graph);
        assert_eq!(dist, 5);
    }
}
