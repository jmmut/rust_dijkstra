pub const INFINITE_DIST: usize = 100000000;

#[derive(Debug, PartialEq)]
pub struct Edge {
    index_second: usize,
    weight: usize,
}

#[derive(Debug, PartialEq)]
pub struct Graph {
    number_of_nodes: usize,
    edges: Vec<Vec<Edge>>,
}

#[derive(Debug, Clone)]
pub struct GraphNode {
    index: usize,
    node_name: String,
}

pub fn construct_graph_from_edges(graph_nodes: &Vec<GraphNode>, edge_data: &str) -> Graph {
    let edges: Vec<&str> = edge_data.split("\n").collect();
    let num_edges: usize = edges[0]
        .parse::<usize>()
        .expect("Expect an integer number of edges.");

    if edges.len() != num_edges + 1 {
        println!("Unexpected number of edges");
    }

    let num_nodes = graph_nodes.len();

    let mut vec: Vec<Vec<Edge>> = Vec::with_capacity(num_nodes);

    for _ in 0..num_nodes {
        vec.push(Vec::with_capacity(num_nodes));
    }
    let mut graph = Graph {
        number_of_nodes: graph_nodes.len(),
        edges: vec,
    };

    for i in 1..(num_edges + 1) {
        let edge: Vec<&str> = edges[i].split(" ").collect();
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
        let new_edge = Edge {
            index_second: end_index,
            weight: edge_weight,
        };
        let new_edge_reverse = Edge {
            index_second: start_index,
            weight: edge_weight,
        };

        // create Edge and add to graph.
        // todo: make this not dumb
        let mut i = 0;
        let mut old_edge_weight = INFINITE_DIST;
        for e in &graph.edges[start_index] {
            if e.index_second == new_edge.index_second {
                old_edge_weight = e.weight;
                break;
            }
            i += 1;
        }

        if old_edge_weight > new_edge.weight {
            if old_edge_weight != INFINITE_DIST {
                graph.edges[start_index].remove(i);
            }
            graph.edges[start_index].push(new_edge);

            i = 0;
            for e in &graph.edges[end_index] {
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
    let nodes: Vec<&str> = node_data.split("\n").collect();
    let num_nodes: usize = nodes[0]
        .parse::<usize>()
        .expect("Expect an integer number of nodes.");

    if nodes.len() != num_nodes + 1 {
        println!("Unexpected number of nodes");
    }

    let mut graph_nodes = Vec::with_capacity(num_nodes);

    for i in 1..(num_nodes + 1) {
        graph_nodes.push(GraphNode {
            index: i - 1,
            node_name: nodes[i].to_string(),
        });
    }

    if cfg!(debug_assertions) {
        println!("graph nodes: {:?}", graph_nodes);
    }


    return graph_nodes;
}

fn read_input(filepath: String) -> (String, String, String) {
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let data: Vec<&str> = contents.split("\n\n").collect();

    let node_data = data[0].to_string();
    let edge_data = data[1].to_string();
    let routes_to_find = data[2].to_string();

    return (node_data, edge_data, routes_to_find);
}

fn get_route(routes_to_find: &str, graph_nodes: Vec<GraphNode>) -> (usize, usize) {
    let routes: Vec<&str> = routes_to_find.split("\n").collect();
    let first_route: Vec<&str> = routes[0].split(" ").collect(); //todo: other routes
    let start_str = first_route[0];
    let end_str = first_route[1];
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
    return (start_idx, end_idx);
}

#[cfg(test)]
mod graph_only_tests {
    use super::*;
    #[test]
    fn test_parsing_data() {
        assert_eq!(1, 1)
        //todo: add tests for correct parsing of data (low priority)
        // e.g. if num_nodes or num_edges is incorrect
        // e.g. if there are edges to nodes that don't exist
        // e.g. if spacing/formatting of input is incorrect
    }
    #[test]
    fn test_basic_input() {
        let contents = "4\nI\nG\nE\nN\n\n5\nI G 167\nI E 158\nG E 45\nG N 145\nE N 107\n\nG E\nE I\n\n";
        let data: Vec<&str> = contents.split("\n\n").collect();

        //let node_data = data[0].to_string();
        //let edge_data = data[1].to_string();
        let routes_to_find = data[2].to_string();

        assert_eq!(routes_to_find, "G E\nE I");
    }
    #[test]
    fn test_multiple_start_edges_input() {
        let contents = "3\nI\nG\nE\n\n4\nI G 167\nI E 158\nG E 45\nI G 17\n\nG E\nE I\n\n";
        let data: Vec<&str> = contents.split("\n\n").collect();

        let node_data = data[0].to_string();
        let edge_data = data[1].to_string();

        let graph_nodes: Vec<GraphNode> = get_nodes(&node_data);
        let graph = construct_graph_from_edges(&graph_nodes, &edge_data);
        // graph should not contain the I->G 167 path, as this should be updated by the I->G 17 path.
        let expected_graph = Graph {
            number_of_nodes: 3,
            edges: vec![
                vec![
                    Edge {
                        index_second: 2,
                        weight: 158,
                    },
                    Edge {
                        index_second: 1,
                        weight: 17,
                    },
                ],
                vec![
                    Edge {
                        index_second: 2,
                        weight: 45,
                    },
                    Edge {
                        index_second: 0,
                        weight: 17,
                    },
                ],
                vec![
                    Edge {
                        index_second: 0,
                        weight: 158,
                    },
                    Edge {
                        index_second: 1,
                        weight: 45,
                    },
                ],
            ],
        };
        assert_eq!(expected_graph, graph);
    }
    #[test]
    fn test_route_extraction() {
        let input_line = "Glasgow Edinburgh\nEdinburgh Inverness";
        let graph_nodes = vec![
            GraphNode {
                index: 0,
                node_name: "Inverness".to_string(),
            },
            GraphNode {
                index: 1,
                node_name: "Glasgow".to_string(),
            },
            GraphNode {
                index: 2,
                node_name: "Edinburgh".to_string(),
            }
        ];

        let (start_idx, end_idx) = get_route(input_line, graph_nodes);
        assert_eq!(start_idx, 1);
        assert_eq!(end_idx, 2);
    }
    #[test]
    fn test_route_finding() {
        assert_eq!(1, 1)
        //todo: add tests for correct parsing of data (low priority)
        // e.g. if input file contains multiple edges from A->B with diff weights
        // e.g. if all edges result in a loop
        // e.g. no routes can be found
    }
}
