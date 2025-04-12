# constraint-builder: A simple computational graph builder for constraint checking.

## Overview

This library uses computational graphs for building, evaluating, and verifying constraint systems. It has applications in various domains including:

- Zero-knowledge proof systems
- Circuit-based cryptographic protocols
- Arithmetic circuit modeling
- Symbolic computation and verification
- Constraint satisfaction problems

## How It Works

The computational graph is built around the following core concepts:

1. **Nodes**: The fundamental building blocks that represent values or operations
   - Input nodes: Values provided at evaluation time
   - Constant nodes: Fixed values
   - Derived nodes: Results of operations on other nodes
   - Hint nodes: Special nodes that compute values through custom functions

2. **Operations**: Mathematical operations that connect nodes
   - Addition: Combines two nodes with addition
   - Multiplication: Combines two nodes with multiplication
   - Hint functions: Custom operations (division, square root, etc.)

3. **Constraints**: Assertions that two nodes must have equal values

4. **Evaluation**: The process of calculating values for all nodes based on inputs

## Installation

This isn't published on crates yet, so only installation option is directly through git.

```toml
[dependencies]
comp_graph = { git = "https://github.com/ameanasad/comp_graph" }
```

## Usage Examples

### Basic Arithmetic: Polynomial Evaluation

```rust
use computational_graph::comp_graph::CompGraph;
use std::collections::HashMap;

// Compute f(x) = x^2 + x + 5
fn main() {
    let mut graph = CompGraph::new();

    // Create nodes for our computation
    let x = graph.init();                      // Input variable
    let x_squared = graph.mul(x, x);           // x^2
    let five = graph.constant(5);              // Constant 5
    let x_plus_5 = graph.add(x, five);         // x + 5
    let result = graph.add(x_squared, x_plus_5); // x^2 + x + 5

    // Evaluate with x = 3
    let mut inputs = HashMap::new();
    inputs.insert(x, 3);
    graph.fill_nodes(inputs);

    // Get the result: 3^2 + 3 + 5 = 9 + 3 + 5 = 17
    let value = graph.nodes[&result].get_value().unwrap();
    println!("f(3) = {}", value); // Should output 17

    assert_eq!(value, 17);
}
```

### Constraint Satisfaction: Verifying Square Root

```rust
use computational_graph::comp_graph::CompGraph;
use std::collections::HashMap;

fn main() {
    let mut graph = CompGraph::new();

    // We want to compute sqrt(y) and verify it's correct
    let y = graph.init();  // Input: the number we want to find the square root of

    // Create a hint node that computes the square root
    let sqrt_y = graph.hint(y, |val| {
        // Calculate integer square root
        let sqrt_val = (val as f64).sqrt() as u32;
        Ok(sqrt_val)
    });

    // Create constraint: sqrt_y * sqrt_y == y
    let sqrt_squared = graph.mul(sqrt_y, sqrt_y);
    graph.assert_equal(y, sqrt_squared);

    // Test with y = 16
    let mut inputs = HashMap::new();
    inputs.insert(y, 16);
    graph.fill_nodes(inputs);

    // Verify constraints are satisfied
    assert!(graph.check_constraints());

    // Get the computed square root
    let result = graph.nodes[&sqrt_y].get_value().unwrap();
    println!("sqrt(16) = {}", result); // Should output 4

    assert_eq!(result, 4);
}
```


### Cryptographic Use Case: Merkle Tree Verification

```rust
use computational_graph::comp_graph::CompGraph;
use std::collections::HashMap;

fn main() {
    // Simplified Merkle tree path verification
    let mut graph = CompGraph::new();

    let const_7 = graph.constant(7);
    let const_11 = graph.constant(11);
    let _const_1000 = graph.constant(1000);
    let _zero = graph.constant(0);
    let _one = graph.constant(1);

    // Input nodes for the leaf and each hash in the authentication path
    let leaf_value = graph.init();
    let auth_hash_1 = graph.init();
    let auth_hash_2 = graph.init();

    // Direction bits (0 = left, 1 = right)
    let direction_1 = graph.init();
    let direction_2 = graph.init();

    // Root hash (the value we want to verify against)
    let expected_root = graph.init();

    // Create selectors based on direction_1
    let is_left_1 = graph.hint(direction_1, |dir| {
        if dir == 0 { Ok(1) } else { Ok(0) }
    });
    let is_right_1 = graph.hint(direction_1, |dir| {
        if dir == 1 { Ok(1) } else { Ok(0) }
    });

    // Create selectors based on direction_2
    let is_left_2 = graph.hint(direction_2, |dir| {
        if dir == 0 { Ok(1) } else { Ok(0) }
    });
    let is_right_2 = graph.hint(direction_2, |dir| {
        if dir == 1 { Ok(1) } else { Ok(0) }
    });

    // First-level hash computation

    // When leaf is left and auth_hash_1 is right
    let leaf_mul_7 = graph.mul(leaf_value, const_7);
    let auth1_mul_11 = graph.mul(auth_hash_1, const_11);
    let left_hash = graph.add(leaf_mul_7, auth1_mul_11);
    let left_hash_mod = graph.hint(left_hash, |val| Ok(val % 1000));

    // When leaf is right and auth_hash_1 is left
    let auth1_mul_7 = graph.mul(auth_hash_1, const_7);
    let leaf_mul_11 = graph.mul(leaf_value, const_11);
    let right_hash = graph.add(auth1_mul_7, leaf_mul_11);
    let right_hash_mod = graph.hint(right_hash, |val| Ok(val % 1000));

    // Select first-level hash based on direction
    let left_contrib = graph.mul(is_left_1, left_hash_mod);
    let right_contrib = graph.mul(is_right_1, right_hash_mod);
    let selected_hash1 = graph.add(left_contrib, right_contrib);

    // Second-level hash computation

    // When selected_hash1 is left and auth_hash_2 is right
    let sel_hash_mul_7 = graph.mul(selected_hash1, const_7);
    let auth2_mul_11 = graph.mul(auth_hash_2, const_11);
    let root_left = graph.add(sel_hash_mul_7, auth2_mul_11);
    let root_left_mod = graph.hint(root_left, |val| Ok(val % 1000));

    // When selected_hash1 is right and auth_hash_2 is left
    let auth2_mul_7 = graph.mul(auth_hash_2, const_7);
    let sel_hash_mul_11 = graph.mul(selected_hash1, const_11);
    let root_right = graph.add(auth2_mul_7, sel_hash_mul_11);
    let root_right_mod = graph.hint(root_right, |val| Ok(val % 1000));

    // Select final root based on direction
    let left_root_contrib = graph.mul(is_left_2, root_left_mod);
    let right_root_contrib = graph.mul(is_right_2, root_right_mod);
    let computed_root = graph.add(left_root_contrib, right_root_contrib);

    // Constrain that computed root equals expected root
    graph.assert_equal(computed_root, expected_root);

    // Test values
    let mut inputs = HashMap::new();
    let leaf = 42;
    let auth1 = 123;
    let auth2 = 456;

    // Calculate expected root for our test case (leaf is left child, first hash is left child)
    let hash_fn = |a: u32, b: u32| -> u32 {
        (a * 7 + b * 11) % 1000
    };
    let h1 = hash_fn(leaf, auth1);
    let root = hash_fn(h1, auth2);

    inputs.insert(leaf_value, leaf);
    inputs.insert(auth_hash_1, auth1);
    inputs.insert(auth_hash_2, auth2);
    inputs.insert(direction_1, 0);  // 0 = left
    inputs.insert(direction_2, 0);  // 0 = left
    inputs.insert(expected_root, root);

    // Clone before using
    let inputs_clone = inputs.clone();
    graph.fill_nodes(inputs_clone);

    // Check if the constraints are satisfied
    let verified = graph.check_constraints();
    println!("Merkle tree path verification: {}", if verified { "success" } else { "failed" });
    assert!(verified);

    // Now try with an invalid root - verification should fail
    let mut invalid_inputs = inputs.clone();
    invalid_inputs.insert(expected_root, root + 1);  // Wrong root hash

    graph.fill_nodes(invalid_inputs);
    let invalid_verified = graph.check_constraints();
    println!("Invalid Merkle tree verification: {}", if invalid_verified { "success (unexpected!)" } else { "failed (expected)" });
    assert!(!invalid_verified);
}
```

### Zero-Knowledge Proof: Range Proof

```rust
use computational_graph::comp_graph::CompGraph;
use std::collections::HashMap;

fn main() {
    // A range proof demonstrates that a secret value is within a specific range
    // (e.g., proving age is between 18 and 65 without revealing the exact age)
    let mut graph = CompGraph::new();

    // Create constants first to avoid multiple mutable borrows
    let lower_bound = graph.constant(18);
    let upper_bound = graph.constant(65);

    // The secret value (the age in our example)
    let secret = graph.init();

    // For lower bound: secret >= lower_bound
    // We can express this as: secret = lower_bound + a² (where a is some value)

    // First create a hint that calculates the difference
    let diff_from_lower = graph.hint(secret, |s| {
        if s >= 18 {
            Ok(s - 18)
        } else {
            Ok(0) // If below range, will cause constraint to fail
        }
    });

    // Then create a hint for the square root of that difference
    let sqrt_diff = graph.hint(diff_from_lower, |diff| {
        // Calculate square root and truncate to integer
        let sqrt = (diff as f64).sqrt() as u32;
        Ok(sqrt)
    });

    // Recalculate the perfect square to account for truncation
    let perfect_square = graph.mul(sqrt_diff, sqrt_diff);

    // Create a "remainder" value to make the equation balance exactly
    let remainder = graph.hint(diff_from_lower, |diff| {
        let sqrt = (diff as f64).sqrt() as u32;
        let perfect_sq = sqrt * sqrt;
        Ok(diff - perfect_sq)
    });

    // Verify that diff_from_lower = perfect_square + remainder
    let perfect_square_plus_remainder = graph.add(perfect_square, remainder);
    graph.assert_equal(diff_from_lower, perfect_square_plus_remainder);

    // Finally verify that lower_bound + diff_from_lower equals secret
    let lower_bound_plus_diff = graph.add(lower_bound, diff_from_lower);
    graph.assert_equal(secret, lower_bound_plus_diff);

    // For upper bound: secret <= upper_bound
    // We can express this as: upper_bound = secret + b² (where b is some value)

    // First create a hint that calculates the difference
    let diff_from_upper = graph.hint(secret, |s| {
        if s <= 65 {
            Ok(65 - s)
        } else {
            Ok(0) // If above range, will cause constraint to fail
        }
    });

    // Then create a hint for the square root of that difference
    let sqrt_upper_diff = graph.hint(diff_from_upper, |diff| {
        // Calculate square root and truncate to integer
        let sqrt = (diff as f64).sqrt() as u32;
        Ok(sqrt)
    });

    // Recalculate the perfect square to account for truncation
    let upper_perfect_square = graph.mul(sqrt_upper_diff, sqrt_upper_diff);

    // Create a "remainder" value to make the equation balance exactly
    let upper_remainder = graph.hint(diff_from_upper, |diff| {
        let sqrt = (diff as f64).sqrt() as u32;
        let perfect_sq = sqrt * sqrt;
        Ok(diff - perfect_sq)
    });

    // Verify that diff_from_upper = upper_perfect_square + upper_remainder
    let upper_square_plus_remainder = graph.add(upper_perfect_square, upper_remainder);
    graph.assert_equal(diff_from_upper, upper_square_plus_remainder);

    // Finally verify that secret + diff_from_upper equals upper_bound
    let secret_plus_diff = graph.add(secret, diff_from_upper);
    graph.assert_equal(upper_bound, secret_plus_diff);

    // Test with a valid age: 30
    let mut inputs = HashMap::new();
    inputs.insert(secret, 30);

    // Clone before passing to fill_nodes
    let inputs_clone = inputs.clone();
    graph.fill_nodes(inputs_clone);

    let valid_range = graph.check_constraints();
    println!("Age verification (30): {}", if valid_range { "in range" } else { "out of range" });
    assert!(valid_range);

    // Test with an invalid age: 15
    let mut invalid_inputs = HashMap::new();
    invalid_inputs.insert(secret, 15);

    graph.fill_nodes(invalid_inputs);
    let invalid_range = graph.check_constraints();
    println!("Age verification (15): {}", if invalid_range { "in range (unexpected!)" } else { "out of range (expected)" });
    assert!(!invalid_range);
}
```

## Advanced Topics

### Topological Evaluation and Parallel Processing

The computational graph processes nodes in levels determined by their dependencies, ensuring that no node is evaluated before its inputs are ready. This topological ordering enables:

1. Correct sequential evaluation of dependent calculations
2. Parallel processing of nodes at the same level with no interdependencies
3. Efficient resource utilization in compute-intensive applications

The implementation uses Rayon for parallel processing of nodes within each level, significantly accelerating computation for large graphs.

### The Hint Mechanism: Extending Computation Capabilities

The `hint` function is a powerful feature that extends the graph beyond basic addition and multiplication operations. It allows:

- Implementation of non-linear functions such as division, square roots, modular arithmetic
- Conditional logic and decision trees
- Complex mathematical operations not directly expressible in constraint systems
- Optimizations and shortcuts in computation paths

Hints work by providing a function that suggests a value based on an input node's value. The system can then verify the correctness of this hint through constraints.

### Applications in Zero-Knowledge Proofs

This computational graph is particularly valuable for developing zero-knowledge proof systems:

1. **Arithmetic Circuits**: The graph naturally represents the arithmetic circuits used in ZKP systems like SNARKs and STARKs
2. **Witness Generation**: Efficiently computes witness values for proof generation
3. **Constraint Satisfaction**: Verifies that all mathematical relationships in the proof system hold
4. **R1CS Conversion**: The graph structure can be translated to Rank-1 Constraint Systems for SNARK generation

## Performance Considerations

- **Memory Usage**: Large graphs with many nodes consume more memory
- **Parallelization**: Performance scales with available CPU cores for level-based execution
- **Constraint Verification**: Performance depends on the number and complexity of constraints
- **Hint Complexity**: Complex hint functions may become bottlenecks

## Contributing

Contributions are welcome! Here are some ways to contribute:

- Report bugs and request features by creating issues
- Improve documentation and examples
- Submit pull requests with bug fixes or new features
- Share interesting use cases and applications

Please follow the existing code style and include tests for new functionality.

## Future Directions

- Support for floating-point operations
- Integration with popular ZKP frameworks
- Addition of more complex operations (matrix operations, trigonometric functions)
- Performance optimizations for very large graphs
- Serialization and deserialization of graphs

## License

This project is licensed under the MIT License
