# constraint-builder: A simple computational graph builder for constraint checking. 

## Overview

## Features

- Create computational graphs with various mathematical operations
- Define relationships between nodes with constraints
- Execute computations in proper dependency order
- Verify that constraints between nodes are satisfied
- Support for providing hints to guide computation

## Usage Example

```rust
use constraint_builder::comp_graph::CompGraph;
use std::collections::HashMap;

fn main() {
    // Create a new computational graph
    let mut graph = CompGraph::new();

    // Initialize variables and operations
    let x = graph.init();                    // Create an input variable
    let x_squared = graph.mul(x, x);         // x^2
    let five = graph.constant(5);            // Constant value 5
    let x_squared_plus_5 = graph.add(x_squared, five); // x^2 + 5
    let result = graph.add(x_squared_plus_5, x);       // x^2 + 5 + x

    // Set input values
    let mut input_nodes = HashMap::new();
    input_nodes.insert(x, 2);

    // Compute results
    graph.fill_nodes(input_nodes);

    // Verify constraints
    assert!(graph.check_constraints());

    // Access the final result
    let final_value = graph.get_value(result).unwrap();
    println!("Result: {}", final_value);  // Should output: Result: 11 (2^2 + 5 + 2)
}
```

## API Documentation

### `CompGraph`

The main structure representing a computational graph.

#### Methods

- `new() -> Self`: Create a new empty computational graph
- `init() -> usize`: Create a new input node and return its ID
- `constant(value: u32) -> usize`: Create a constant node with a fixed value
- `add(a: usize, b: usize) -> usize`: Create an addition node that adds the values of nodes a and b
- `mul(a: usize, b: usize) -> usize`: Create a multiplication node that multiplies the values of nodes a and b
- `fill_nodes(inputs: HashMap<usize, u32>) -> Result<(), String>`: Fill the graph with input values and compute all node values
- `check_constraints() -> bool`: Check if all constraints between nodes are satisfied
- `get_value(node_id: usize) -> Option<u32>`: Get the computed value of a node

### `Node`

Represents a single node in the computational graph.

## Installation

Since this library is not yet published to crates.io, you can include it in your project by adding it to your `Cargo.toml` as a Git dependency:

```toml
[dependencies]
constraint-builder = { git = "https://github.com/yourusername/constraint-builder.git" }
```

Alternatively, clone the repository and use a path dependency:

```toml
[dependencies]
constraint-builder = { path = "../path/to/constraint-builder" }
```

## Building and Testing

```bash
# Clone the repository
git clone https://github.com/yourusername/constraint-builder.git
cd constraint-builder

# Build the library
cargo build

# Run tests
cargo test

# Run the example
cargo run --example simple_expression
```

## Use Cases

- Building constraint systems for zero-knowledge proofs
- Constructing arithmetic circuits for ZK-SNARKs
- Modeling and verifying relationships in cryptographic protocols
- Creating and validating witness calculations

## License

MIT License

## Contributing

This is an early-stage project. Contributions, suggestions, and feedback are welcome!
