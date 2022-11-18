use assert_cmd::prelude::*; // Add methods on commands
use core::cmp::min;
use predicates::prelude::*; // Used for writing assertions
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process::Command; // Run programs
use log::{info, debug};
include!("construct_graph.rs");

#[derive(Debug, Clone, PartialEq)]
struct Node {
    index: usize,
    parent_idx: usize,
    dist_to_node: usize,
}

fn get_route_travelled(
    original_start_idx: usize,
    end_idx: usize,
    nodes_visited: &Vec<Node>,
) -> Vec<usize> {
    //go backwards through the nodes to find the parent node.
    let mut idx = end_idx;
    let mut nodes_in_order: Vec<usize> = Vec::new();
    nodes_in_order.push(end_idx);
    while idx != original_start_idx {
        idx = nodes_visited[idx].parent_idx;
        nodes_in_order.push(idx);
    }

    nodes_in_order.reverse();
    debug!("Nodes in order: {:?}", &nodes_in_order);

    return nodes_in_order;
}

fn get_human_readable_route(
    nodes_in_order: Vec<usize>,
    graph_nodes: &Vec<GraphNode>,
) -> Result<Vec<String>, String> {
    let mut path_travelled: Vec<String> = Vec::new();
    for node_idx in nodes_in_order {
        let node = &graph_nodes[node_idx];

        if node.index != node_idx {
            return Err("Error in the indexing for the route travelled.".to_string());
        } else {
            path_travelled.push(node.node_name.to_string());
        }
    }
    return Ok(path_travelled);
}

fn print_route(route: Vec<String>) -> String {
    let mut final_path: String = route[0].to_string();
    for i in 1..route.len() {
        final_path = format!("{}->{}", final_path, route[i]);
    }

    return final_path;
}


fn reverse_sort(nodes_can_visit: &BTreeMap<usize, Node>) -> usize {
    let mut min_weight = INFINITE_DIST;
    let mut index_to_remove = INFINITE_DIST;
    for (_, node) in nodes_can_visit {
        if node.dist_to_node < min_weight {
            min_weight = node.dist_to_node;
            index_to_remove = node.index;
        }
    }
    return index_to_remove;
}

fn dijkstra(
    mut start_idx: usize,
    end_idx: usize,
    graph: &Graph,
) -> Result<(usize, Vec<usize>), String> {
    let original_start_idx = start_idx;
    let mut parent_idx = start_idx;

    let number_of_nodes = graph.number_of_nodes;
    //todo: use a binary search tree here to avoid needing to allocate space for the whole vector.
    let mut nodes_visited: Vec<Node> = Vec::with_capacity(number_of_nodes);
    for _ in 0..number_of_nodes {
        nodes_visited.push(Node{index: INFINITE_DIST, parent_idx: INFINITE_DIST, dist_to_node: 0});
    }
    nodes_visited[start_idx] = Node {index: start_idx, parent_idx, dist_to_node: 0};

    let mut nodes_can_visit: BTreeMap<usize, Node> = BTreeMap::new();

    while start_idx != end_idx {

        // which nodes can we visit
        for i in &graph.edges[start_idx] {
            // if present, minimise weight
            if nodes_can_visit.contains_key(&i.index_second) {
                nodes_can_visit
                    .entry(i.index_second)
                    .and_modify(|curr_node| {
                        curr_node.dist_to_node =
                            min(i.weight + nodes_visited[start_idx].dist_to_node, curr_node.dist_to_node)
                    });
            } else if (None == nodes_visited.iter().find(|&x| x.index == i.index_second)) && i.index_second != start_idx {
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
        if nodes_can_visit.is_empty() {
            return Err("Are the start and end disconnected? No path found".to_string());
        }
        debug!("nodes can visit: {:?}", nodes_can_visit);

        let index_to_remove = reverse_sort(&nodes_can_visit);
        let closest_node = nodes_can_visit.remove(&index_to_remove).ok_or("Error in path finding".to_string())?;

        if (closest_node.index != start_idx) && (None == nodes_visited.iter().find(|&x| x.index == closest_node.index)) {
            start_idx = closest_node.index;
            parent_idx = closest_node.parent_idx;
            nodes_visited[start_idx] = Node{index: start_idx, parent_idx, dist_to_node: nodes_visited[parent_idx].dist_to_node + closest_node.dist_to_node};

        }
    }

    let nodes_in_order = get_route_travelled(original_start_idx, end_idx, &nodes_visited);

    return Ok((nodes_visited[end_idx].dist_to_node, nodes_in_order));
}

fn get_human_readable_solution(
    route: &str,
    graph_nodes: &Vec<GraphNode>,
    graph: &Graph,
) -> Result<String, String> {
    let route_names: Vec<&str> = route.split(" ").collect();
    let route_result = get_route(route_names, &graph_nodes)?;
    let (start_idx, end_idx) = route_result;
    debug!("finding route from {} to {}", start_idx, end_idx);

    let (dist, route) = dijkstra(start_idx, end_idx, &graph)?;
    let human_readable_route = get_human_readable_route(route, &graph_nodes)?;
    let result = print_route(human_readable_route);

    return Ok(format!(
        "Route travelled: {}, with distance {}",
        result, dist
    ));
}

fn main() -> Result<(), String> {
    env_logger::init();
    // read input
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(
            "Please provide relative file path as input arg, i.e. `$ cargo run <src/test/uk.txt>`"
                .to_string(),
        );
    }
    let filename = &args[1];
    let contents =
        fs::read_to_string(filename.to_string()).expect("Should have been able to read the file");
    let (node_data, edge_data, routes_to_find) = read_input(contents)?;
    let graph_nodes: Vec<GraphNode> = get_nodes(&node_data);
    let graph = construct_graph_from_edges(&graph_nodes, &edge_data)?;

    debug!("graph: {:?}", graph);

    let routes: Vec<&str> = routes_to_find.trim().split("\n").collect();
    for route in routes {
        // todo: parallelise this &learn how to do threading in rust, for loop is slower
        let result = get_human_readable_solution(route, &graph_nodes, &graph);
        match result {
            Err(err) => println!("An error occured on the path {}. Error: {}", route, err),
            Ok(_) => println!("{}", result.unwrap())
        }
    }

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

        let (dist, _) = dijkstra(start_idx, end_idx, &graph).unwrap();
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

        let (dist, _) = dijkstra(start_idx, end_idx, &graph).unwrap();
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
        let (dist, _) = dijkstra(0, 2, &graph).unwrap();
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

        let (dist, _) = dijkstra(start_idx, end_idx, &graph).unwrap();
        assert_eq!(dist, 5);
    }
    #[test]
    fn find_correct_route_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust_dijkstra")?;
        cmd.arg("src/test/uk.txt".to_string());
        cmd.assert().success().stdout(predicate::str::contains(
            "Route travelled: Glasgow->Edinburgh, with distance 45\n",
        ));

        Ok(())
        //todo test more complex routes than this.
        //test output when multiple paths have the same length.
    }
    #[test]
    fn find_self_referential_route_in_file() -> Result<(), Box<dyn std::error::Error>> {
        //unimplemented
        let mut cmd = Command::cargo_bin("rust_dijkstra")?;
        cmd.arg("src/test/edge-cases.txt".to_string());
        cmd.assert().success().stdout(predicate::str::contains(
            "Route is self referential. Dist from SelfReferential to SelfReferential = 0",
        ));
        Ok(())
    }
    #[test]
    fn find_disconnected_route_in_file() -> Result<(), Box<dyn std::error::Error>> {
        //unimplemented
        let mut cmd = Command::cargo_bin("rust_dijkstra")?;
        cmd.arg("src/test/edge-cases.txt".to_string());

        cmd.assert().success().stdout(predicate::str::contains(
            "Are the start and end disconnected? No path found",
        ));
        Ok(())
    }
}
