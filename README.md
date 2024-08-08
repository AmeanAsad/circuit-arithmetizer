# CompGraph

CompGraph is a Rust library for building and evaluating computational graphs. It provides a flexible and efficient way to construct graphs with nodes related by addition, multiplication, and custom operations.

## Features

- Create computational graphs with nodes representing u32 values
- Support for constant, input, and derived nodes
- Addition and multiplication operations
- Custom operations through a flexible hinting system
- Equality constraints between nodes
- Parallel evaluation of the graph using Rayon
- Thread-safe operations on node values

## Usage

Here's an example of how to use CompGraph to represent and evaluate the function f(a) = (a + 1) / 8:

```rust
use comp_graph::CompGraph;
use std::collections::HashMap;

fn main() {
    let mut graph = CompGraph::new();

    // Define the graph structure
    let a = graph.init();
    let one = graph.constant(1);
    let b = graph.add(a, one);
    let c = graph.hint(b, |val| Ok(val / 8));
    let eight = graph.constant(8);
    let c_times_8 = graph.mul(c, eight);

    // Assert the constraint: b == c * 8
    graph.assert_equal(b, c_times_8);

    // Evaluate the graph
    let mut inputs = HashMap::new();
    inputs.insert(a, 7); // a = 7
    graph.fill_nodes(inputs);

    // Check constraints and get result
    assert!(graph.check_constraints());
    println!("f(7) = {}", graph.nodes[&c].get_value().unwrap()); // Should print 1
}
```

## Concepts

Building a computational graph library like CompGraph involves several key concepts:

1. **Graph Structure**: The computational graph is represented as a directed acyclic graph (DAG) where nodes represent values or operations, and edges represent data flow.

2. **Node Types**:
   - Constant nodes: Represent fixed values
   - Input nodes: Represent variable inputs to the computation
   - Derived nodes: Represent results of operations on other nodes

3. **Operations**: Basic arithmetic operations (addition, multiplication) are implemented as ways to create new derived nodes from existing nodes.

4. **Constraints**: Equality constraints between nodes can be asserted and later checked for validity.

5. **Graph Evaluation**: The process of assigning concrete values to input nodes and propagating these values through the graph to compute the values of all derived nodes.

6. **Hinting**: A mechanism to introduce custom operations or external computations into the graph structure.

7. **Parallelism**: Evaluation of independent nodes can be parallelized to improve performance on multi-core systems.

8. **Thread Safety**: When parallelizing computations, ensure that shared data structures are accessed in a thread-safe manner.

9. **Topological Ordering**: Nodes are evaluated in an order that respects their dependencies, which can be achieved by assigning levels to nodes based on their position in the graph.

Understanding these concepts is crucial for implementing and extending a computational graph library like CompGraph.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.