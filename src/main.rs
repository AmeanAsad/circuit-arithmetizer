use computational_graph::comp_graph::CompGraph;
use std::collections::HashMap;

fn main() {
    let mut graph = CompGraph::new();
    let x = graph.init();
    let x_squared = graph.mul(x, x);
    let five = graph.constant(5);
    let x_squared_plus_5 = graph.add(x_squared, five);
    let _ = graph.add(x_squared_plus_5, x);

    let mut input_nodes = HashMap::new();
    input_nodes.insert(x, 2);
    graph.fill_nodes(input_nodes);

    assert!(graph.check_constraints());
    println!("Constraints Satisfied");
}
