use rstest::*;

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

fn get_edge_info(edge: &str, graph_nodes: &Vec<GraphNode>) -> (usize, usize, usize) {
    let edge_info: Vec<&str> = edge.split(" ").collect();
    let start_edge = edge_info[0];
    let end_edge = edge_info[1];
    let edge_weight = edge_info[2].parse::<usize>().expect(&format!("Distance between edges should be an integer, {edge_weight} found.", edge_weight=edge_info[2]));

    let start_node = graph_nodes.iter().find(|&x| x.node_name == start_edge);
    let start_index = start_node.expect(&format!("Nodes in edges should be present in node list. {start_edge} not found.", start_edge=start_edge)).index;

    let end_node = graph_nodes.iter().find(|&x| x.node_name == end_edge);
    let end_index = end_node.expect(&format!("Nodes in edges should be present in node list. {end_edge} not found.", end_edge=end_edge)).index;

    return (start_index, end_index, edge_weight)
}


fn create_new_edges(start_index: usize, end_index: usize, weight: usize) -> (Edge, Edge){
    let new_edge = Edge {
        index_second: end_index,
        weight,
    };
    let new_edge_reverse = Edge {
        index_second: start_index,
        weight,
    };
    return (new_edge, new_edge_reverse)
}

fn update_existing_edge(graph: &mut Graph, start_index: usize, end_index: usize) -> usize {

    let edge_index_opt : Option<usize> = graph.edges[start_index].iter().position(|x| x.index_second == end_index);
    if None == edge_index_opt {
        return INFINITE_DIST;
    }
    let edge_index = edge_index_opt.unwrap();
    let old_edge_weight = graph.edges[start_index][edge_index].weight;
    if old_edge_weight != INFINITE_DIST {
        graph.edges[start_index].remove(edge_index);
    }
    return old_edge_weight;

}

fn remove_existing_edges_if_shorter_are_found(graph: &mut Graph, new_edge: &Edge, new_edge_reverse: &Edge) {

    let start_index = new_edge_reverse.index_second;
    let end_index = new_edge.index_second;
    let old_edge_weight = update_existing_edge(graph, start_index, end_index);
    if old_edge_weight >= new_edge.weight {
        update_existing_edge(graph, end_index, start_index);
    }

}

pub fn construct_graph_from_edges(graph_nodes: &Vec<GraphNode>, edge_data: &str) -> Result<Graph, String> {
    let edges: Vec<&str> = edge_data.split("\n").collect();
    let num_edges: usize = edges[0]
        .parse::<usize>()
        .expect("Expect an integer number of edges.");

    if num_edges != edges.len() -1 {
        return Err(format!("Unexpected number of edges. Expected: {num_edges}, actual: {edges_len}",
                            num_edges=num_edges, edges_len=edges.len()-1,));
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

        let (start_index, end_index, weight) = get_edge_info(edges[i], graph_nodes);
        let (new_edge, new_edge_reverse) = create_new_edges(start_index, end_index, weight);

        // todo: make this not dumb
        remove_existing_edges_if_shorter_are_found(&mut graph, &new_edge, &new_edge_reverse);
        graph.edges[start_index].push(new_edge);
        graph.edges[end_index].push(new_edge_reverse);

    }

    return Ok(graph);
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

    let number_of_nodes = graph_nodes.len();

    // todo: remove repeated logic for node-name matching
    let start_node = graph_nodes.iter().find(|&x| x.node_name == start_str);
    let start_idx = start_node.expect(&format!("Nodes in edges should be present in node list. {start_edge} not found.", start_edge=start_str)).index;

    let end_node = graph_nodes.iter().find(|&x| x.node_name == end_str);
    let end_idx = end_node.expect(&format!("Nodes in edges should be present in node list. {end_edge} not found.", end_edge=end_str)).index;

    return (start_idx, end_idx);
}

#[cfg(test)]
mod graph_only_tests {
    use super::*;

    #[fixture]
    fn set_up_tests() -> (String, Graph, Vec<GraphNode>) {
        let contents = "3\nI\nG\nE\n\n4\nI G 167\nI E 158\nG E 45\nI G 17\n\nG E\nE I\n\n".to_string();
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

        let graph_nodes = vec![
            GraphNode {
                index: 0,
                node_name: "I".to_string()
            },
            GraphNode {
                index: 1,
                node_name: "G".to_string()
            },
            GraphNode {
                index: 2,
                node_name: "E".to_string()
            }];
        return (contents, expected_graph, graph_nodes)
    }

    //#[test]
    // fn test_parsing_data() {
    //     //todo: add tests for correct parsing of data (low priority)
    //     // e.g. if num_nodes or num_edges is incorrect
    //     // e.g. if there are edges to nodes that don't exist
    //     // e.g. if spacing/formatting of input is incorrect
    // }
    #[test]
    fn test_basic_input() {
        let contents = "4\nI\nG\nE\nN\n\n5\nI G 167\nI E 158\nG E 45\nG N 145\nE N 107\n\nG E\nE I\n\n";
        let data: Vec<&str> = contents.split("\n\n").collect();

        let routes_to_find = data[2].to_string();

        assert_eq!(routes_to_find, "G E\nE I");
    }
    #[rstest]
    fn test_multiple_start_edges_input() {
        let (contents, expected_graph, _) = set_up_tests();
        let data: Vec<&str> = contents.split("\n\n").collect();

        let node_data = data[0].to_string();
        let edge_data = data[1].to_string();

        let graph_nodes: Vec<GraphNode> = get_nodes(&node_data);
        let graph = construct_graph_from_edges(&graph_nodes, &edge_data);
        // graph should not contain the I->G 167 path, as this should be updated by the I->G 17 path.

        assert_eq!(Ok(expected_graph), graph);
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
    #[rstest]
    fn test_route_finding_with_incorrect_nodes() {
        let (_, expected_graph, graph_nodes) = set_up_tests();
        let edge_data = "4\nI G 167\nI E 158\nG E 45\nI G 17\nE I 1".to_string();

        assert_eq!(Err(format!("Unexpected number of edges. Expected: 4, actual: 5")), construct_graph_from_edges(&graph_nodes, &edge_data))
    }
}
