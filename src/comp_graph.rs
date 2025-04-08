use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// A graph for constructing and evaluating computational graphs.
pub struct CompGraph {
    pub nodes: HashMap<usize, Node>,
    constraints: Vec<(usize, usize)>,
    hints: HashMap<usize, Box<dyn Fn(u32) -> Result<u32, String> + Send + Sync>>,
    filled: bool,
    levels: Vec<HashSet<usize>>,
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
enum NodeType {
    Constant(u32),
    Input,
    Derived {
        left: usize,
        right: usize,
        operation: Operation,
    },
    Hint {
        dependent: usize,
    },
}

#[derive(Debug)]
pub struct Node {
    pub index: usize,
    pub value: AtomicU32,
    pub is_some: AtomicBool,
    node_type: NodeType,
    level: usize,
}

impl Node {
    fn new(index: usize, node_type: NodeType, level: usize) -> Self {
        Node {
            index,
            value: AtomicU32::new(0),
            is_some: AtomicBool::new(false),
            node_type,
            level,
        }
    }

    pub fn get_value(&self) -> Option<u32> {
        if self.is_some.load(Ordering::SeqCst) {
            Some(self.value.load(Ordering::SeqCst))
        } else {
            None
        }
    }

    fn set_value(&self, value: u32) {
        self.value.store(value, Ordering::SeqCst);
        self.is_some.store(true, Ordering::SeqCst);
    }
}

impl CompGraph {
    /// Creates a new, empty `CompGraph`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let graph = CompGraph::new();
    /// ```
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            constraints: vec![],
            hints: HashMap::new(),
            filled: false,
            levels: vec![HashSet::new()],
        }
    }

    /// Initializes a new input node in the graph.
    ///
    /// # Returns
    ///
    /// The index of the newly created input node.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut graph = CompGraph::new();
    /// let input_node = graph.init();
    /// ```
    pub fn init(&mut self) -> usize {
        let idx = self.nodes.len();
        let new_node = Node::new(idx, NodeType::Input, 0);
        self.nodes.insert(idx, new_node);
        self.levels[0].insert(idx);
        idx
    }

    /// Initializes a new constant node in the graph.
    ///
    /// # Parameters
    ///
    /// - `value`: The constant value for the node.
    ///
    /// # Returns
    ///
    /// The index of the newly created constant node.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut graph = CompGraph::new();
    /// let const_node = graph.constant(42);
    /// ```
    pub fn constant(&mut self, value: u32) -> usize {
        let idx = self.nodes.len();
        let new_node = Node::new(idx, NodeType::Constant(value), 0);
        new_node.set_value(value);

        self.nodes.insert(idx, new_node);
        self.levels[0].insert(idx);
        idx
    }

    fn add_to_level(&mut self, idx: usize, level: usize) {
        if level >= self.levels.len() {
            self.levels.push(HashSet::new());
        }
        self.levels[level].insert(idx);
    }

    /// Adds two nodes in the graph, returning a new node.
    ///
    /// # Parameters
    ///
    /// - `a`: The index of the first node.
    /// - `b`: The index of the second node.
    ///
    /// # Returns
    ///
    /// The index of the newly created node representing the sum of the two input nodes.
    ///
    /// # Panics
    ///
    /// This function will panic if either of the input nodes do not exist.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut graph = CompGraph::new();
    /// let a = graph.init();
    /// let b = graph.constant(5);
    /// let sum_node = graph.add(a, b);
    /// ```
    pub fn add(&mut self, a: usize, b: usize) -> usize {
        if !self.nodes.contains_key(&a) || !self.nodes.contains_key(&b) {
            panic!("One of the nodes does not exist.");
        }

        let idx = self.nodes.len();
        let a_level = self.nodes[&a].level;
        let b_level = self.nodes[&b].level;
        let new_level = std::cmp::max(a_level, b_level) + 1;

        let new_node = Node::new(
            idx,
            NodeType::Derived {
                left: a,
                right: b,
                operation: Operation::Add,
            },
            new_level,
        );

        self.nodes.insert(idx, new_node);
        self.add_to_level(idx, new_level);
        idx
    }

    /// Multiplies two nodes in the graph, returning a new node.
    ///
    /// # Parameters
    ///
    /// - `a`: The index of the first node.
    /// - `b`: The index of the second node.
    ///
    /// # Returns
    ///
    /// The index of the newly created node representing the product of the two input nodes.
    ///
    /// # Panics
    ///
    /// This function will panic if either of the input nodes do not exist.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut graph = CompGraph::new();
    /// let a = graph.init();
    /// let b = graph.constant(5);
    /// let product_node = graph.mul(a, b);
    /// ```
    pub fn mul(&mut self, a: usize, b: usize) -> usize {
        if !self.nodes.contains_key(&a) || !self.nodes.contains_key(&b) {
            panic!("One of the nodes does not exist.");
        }

        let idx = self.nodes.len();
        let a_level = self.nodes[&a].level;
        let b_level = self.nodes[&b].level;
        let new_level = std::cmp::max(a_level, b_level) + 1;

        let new_node = Node::new(
            idx,
            NodeType::Derived {
                left: a,
                right: b,
                operation: Operation::Mul,
            },
            new_level,
        );

        self.nodes.insert(idx, new_node);
        self.add_to_level(idx, new_level);
        idx
    }

    /// Asserts that two nodes are equal.
    ///
    /// # Parameters
    ///
    /// - `a`: The index of the first node.
    /// - `b`: The index of the second node.
    ///
    /// # Panics
    ///
    /// This function will panic if either of the nodes do not exist.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut graph = CompGraph::new();
    /// let a = graph.constant(5);
    /// let b = graph.constant(5);
    /// graph.assert_equal(a, b);
    /// ```
    pub fn assert_equal(&mut self, a: usize, b: usize) {
        if !self.nodes.contains_key(&a) || !self.nodes.contains_key(&b) {
            panic!("One of the nodes does not exist.");
        }
        self.constraints.push((a, b))
    }

    fn fill_node(&self, node_idx: usize, input_nodes: &HashMap<usize, u32>) -> u32 {
        let node = &self.nodes[&node_idx];
        if let Some(val) = node.get_value() {
            return val;
        }

        let res: u32 = match &node.node_type {
            NodeType::Constant(val) => *val,
            NodeType::Input => *input_nodes
                .get(&node_idx)
                .expect("Input node value not provided."),
            NodeType::Derived {
                left,
                right,
                operation,
            } => {
                let left_value = self.fill_node(*left, input_nodes);
                let right_value = self.fill_node(*right, input_nodes);
                match operation {
                    Operation::Add => left_value + right_value,
                    Operation::Mul => left_value * right_value,
                }
            }
            NodeType::Hint { dependent } => {
                let dep_value = self.fill_node(*dependent, input_nodes);
                let hint_fn = self.hints.get(&node_idx).expect("Hint function not found.");
                match hint_fn(dep_value) {
                    Ok(val) => val,
                    Err(err) => panic!("Hint function error: {}", err),
                }
            }
        };

        node.set_value(res);

        res
    }

    /// Fills in all the nodes of the graph based on some inputs.
    ///
    /// # Parameters
    ///
    /// - `input_nodes`: A map of input node indices to their values.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut graph = CompGraph::new();
    /// let x = graph.init();
    /// let mut input_nodes = HashMap::new();
    /// input_nodes.insert(x, 2);
    /// graph.fill_nodes(input_nodes);
    /// ```
    pub fn fill_nodes(&mut self, input_nodes: HashMap<usize, u32>) {
        // Fill initial input nodes
        for (idx, &val) in &input_nodes {
            if let Some(node) = self.nodes.get(idx) {
                node.set_value(val);
            }
        }

        // Fill derived nodes and hint nodes based on input nodes and other derived nodes
        for level in &self.levels {
            level.par_iter().for_each(|&idx| {
                self.fill_node(idx, &input_nodes);
            });
        }

        self.filled = true;
    }

    /// Given a graph that has `fill_nodes` already called on it,
    /// checks that all the constraints hold.
    ///
    /// # Returns
    ///
    /// `true` if all constraints hold, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut graph = CompGraph::new();
    /// // Add nodes and constraints
    /// let result = graph.check_constraints();
    /// assert!(result);
    /// ```
    pub fn check_constraints(&self) -> bool {
        self.constraints.iter().all(|(n1, n2)| {
            let val1 = self.nodes.get(&n1).unwrap().get_value().unwrap();
            let val2 = self.nodes.get(&n2).unwrap().get_value().unwrap();
            if val1 != val2 {
                eprintln!(
                    "Constraint violation: Node {} with value {} is not equal to Node {} with value {}",
                    n1, val1, n2, val2
                );
                false
            } else {
                true
            }
        })
    }

    /// An API for hinting values that allows enables performing operations
    /// like division or computing square roots.
    ///
    /// # Parameters
    ///
    /// - `dependent_idx`: The index of the dependent node.
    /// - `hint_fn`: A function that takes the value of the dependent node and
    /// returns the hinted value or an error.
    ///
    /// # Returns
    ///
    /// The index of the newly created hint node.
    ///
    /// # Panics
    ///
    /// This function will panic if the dependent node does not exist.
    ///
    /// # Examples
    /// ```ignore
    /// let mut graph = CompGraph::new();
    /// let x = graph.init();
    /// let hinted_node = graph.hint(x, |val| Ok(val / 2));
    /// ```
    pub fn hint<F>(&mut self, dependent_idx: usize, hint_fn: F) -> usize
    where
        F: Fn(u32) -> Result<u32, String> + 'static + Send + Sync,
    {
        if !self.nodes.contains_key(&dependent_idx) {
            panic!("Dependent node does not exist.");
        }

        let dep_level = self.nodes[&dependent_idx].level;

        let idx = self.nodes.len();
        let new_node = Node::new(
            idx,
            NodeType::Hint {
                dependent: dependent_idx,
            },
            dep_level + 1,
        );
        self.nodes.insert(idx, new_node);
        self.hints.insert(idx, Box::new(hint_fn));
        self.add_to_level(idx, dep_level + 1);

        idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Example 1: f(x) = x^2 + x + 5
        let mut graph = CompGraph::new();
        let x = graph.init();
        let x_squared = graph.mul(x, x);
        let five = graph.constant(5);
        let x_squared_plus_5 = graph.add(x_squared, five);
        let y = graph.add(x_squared_plus_5, x);

        // Fill nodes with input values
        let mut input_nodes = HashMap::new();
        input_nodes.insert(x, 2);
        graph.fill_nodes(input_nodes);

        // Check constraints
        assert!(graph.check_constraints());
        assert_eq!(graph.nodes[&y].get_value(), Some(2 * 2 + 2 + 5));
        println!("Example 1 constraints satisfied!");
    }

    #[test]
    fn test_example_2() {
        // Example 2: f(a) = (a + 1) / 8
        let mut graph = CompGraph::new();
        let a = graph.init();
        let constant = graph.constant(1);
        let b = graph.add(a, constant);
        let c = graph.hint(b, |val| {
            if val == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok(val / 8)
            }
        });
        let eight = graph.constant(8);
        let c_times_8 = graph.mul(c, eight);
        graph.assert_equal(b, c_times_8);

        // Fill nodes with input values
        let mut input_nodes = HashMap::new();
        input_nodes.insert(a, 7); // a = 7
        graph.fill_nodes(input_nodes);

        // Check constraints
        assert!(graph.check_constraints());
        assert_eq!(graph.nodes[&c].get_value(), Some((7 + 1) / 8));
        println!("Example 2 constraints satisfied!");
    }

    #[test]
    fn test_example_3() {
        // Example 3: f(x) = sqrt(x + 7)
        let mut graph = CompGraph::new();
        let x = graph.init();
        let seven = graph.constant(7);
        let x_plus_seven = graph.add(x, seven);
        let sqrt_x_plus_7 = graph.hint(x_plus_seven, |val| Ok((val as f64).sqrt() as u32));
        let computed_sq = graph.mul(sqrt_x_plus_7, sqrt_x_plus_7);
        graph.assert_equal(x_plus_seven, computed_sq);

        // Fill nodes with input values
        let mut input_nodes = HashMap::new();
        input_nodes.insert(x, 2); // x = 2
        graph.fill_nodes(input_nodes);

        // Check constraints
        assert!(graph.check_constraints());
        assert_eq!(
            graph.nodes[&sqrt_x_plus_7].get_value(),
            Some(((2 + 7) as f32).sqrt() as u32)
        );
        println!("Example 3 constraints satisfied!");
    }

    #[test]
    #[should_panic(expected = "One of the nodes does not exist.")]
    fn test_non_existent_node_add() {
        // Test adding non-existent nodes
        let mut graph = CompGraph::new();
        let non_existent_node = 999;
        graph.add(non_existent_node, non_existent_node);
    }

    #[test]
    #[should_panic(expected = "Input node value not provided.")]
    fn test_uninitialized_input_node() {
        // Test uninitialized input node
        let mut graph = CompGraph::new();
        let x = graph.init();
        let _ = graph.mul(x, x);
        graph.fill_nodes(HashMap::new());
    }

    #[test]
    #[should_panic(expected = "Dependent node does not exist.")]
    fn test_non_existent_hint_node() {
        // Test hinting non-existent nodes
        let mut graph = CompGraph::new();
        let non_existent_node = 999;
        graph.hint(non_existent_node, |val| Ok(val));
    }
}
