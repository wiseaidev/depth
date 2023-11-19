//! # dependency_graph
//!
//! The `dependency_graph` module provides functionality to fetch and visualize dependency trees
//! for Rust packages using the Crates.io API and the petgraph crate. With this module, you can
//! explore the dependencies of a given Rust package and visualize the relationship between
//! different packages in a dependency graph.
//!
//! # Quick Start
//!
//! Get started with the `dependency_graph` module by following these simple steps:
//!
//! 1. Import the necessary types and functions into your code:
//!
//! ```rust
//! use depth::package::{fetch_package_info, Package};
//! use depth::dependency_graph::DependencyGraph;
//! use crates_io_api::SyncClient;
//! ```
//!
//! 2. Create an instance of `DependencyGraph` and use it to fetch and visualize dependency trees:
//!
//! ```rust
//! use depth::dependency_graph::DependencyGraph;
//! use depth::package::Package;
//!
//! let package = Package::new("".to_string(), "".to_string(), vec![("name".to_string(), "version".to_string())], false);
//! let mut graph = DependencyGraph::new();
//! graph.fetch_dependency_tree("your_package_name", 2);
//! graph.print_dependencies_at_level(&package, 0, 2);
//! ```
//!
//! # Key Features
//!
//! The `dependency_graph` module offers the following key features:
//!
//! - **Fetch Dependency Tree**: Fetch the dependency tree for a given Rust package using the Crates.io API.
//! - **Visualize Dependencies**: Visualize the dependencies of a package in a graph structure using petgraph.
//! - **Print Dependencies**: Print the dependencies of a package at a specified depth in the dependency tree.
//!
//! # Usage
//!
//! ## Fetching Dependency Tree
//!
//! Use the `fetch_dependency_tree` method to fetch and build the dependency tree for a specific package:
//!
//! ```rust
//! use depth::dependency_graph::DependencyGraph;
//!
//! let mut graph = DependencyGraph::new();
//! graph.fetch_dependency_tree("your_package_name", 2);
//! ```
//!
//! ## Visualizing Dependencies
//!
//! Utilize the `print_dependencies_at_level` method to print dependencies at a specified depth in the dependency tree:
//!
//! ```rust
//! use depth::package::{fetch_package_info, Package};
//! use depth::dependency_graph::DependencyGraph;
//!
//! let package = Package::new("".to_string(), "".to_string(), vec![("name".to_string(), "version".to_string())], false);
//! let mut graph = DependencyGraph::new();
//! graph.print_dependencies_at_level(&package, 0, 2);
//! ```
//!
//! # Examples
//!
//! ```rust
//! use depth::package::Package;
//! use depth::dependency_graph::DependencyGraph;
//! use crates_io_api::SyncClient;
//!
//! let mut graph = DependencyGraph::new();
//! graph.fetch_dependency_tree("your_package_name", 2);
//! // Additional functionality with the dependency graph...
//! ```

use crate::package::{fetch_package_info, Package};
use crates_io_api::SyncClient;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use std::collections::{HashMap, HashSet};

/// A struct representing a dependency graph.
#[derive(Debug)]
pub struct DependencyGraph {
    /// The underlying directed graph.
    graph: DiGraph<(String, String), &'static str>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    /// Creates a new instance of `DependencyGraph`.
    pub fn new() -> Self {
        DependencyGraph {
            graph: DiGraph::new(),
        }
    }

    /// Fetches the dependency tree for a given package.
    ///
    /// # Arguments
    ///
    /// * `package_name` - The name of the package to fetch.
    /// * `depth` - The maximum depth to fetch dependencies.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(package))` if the package is fetched successfully,
    /// `Ok(None)` if the package does not exist, and `Err` on an error.
    pub fn fetch_dependency_tree(
        &mut self,
        package_name: &str,
        depth: usize,
    ) -> Result<Option<Package>, Box<dyn std::error::Error>> {
        let mut visited_packages = HashMap::new();
        let client = SyncClient::new(
            "my-user-agent (my-contact@domain.com)",
            std::time::Duration::from_millis(1000),
        )
        .unwrap();
        fetch_package_info(
            &(package_name.to_string(), "".to_string()),
            &mut visited_packages,
            self,
            &client,
            depth,
        )
    }

    /// Adds a package and its dependencies to the graph.
    ///
    /// # Arguments
    ///
    /// * `package` - The package to add to the graph.
    ///
    /// # Returns
    ///
    /// Returns the `NodeIndex` of the added package.
    pub fn add_package_to_graph(&mut self, package: &Package) -> NodeIndex {
        let node_index = self
            .graph
            .add_node((package.name.clone(), package.url.clone()));

        for dependency in &package.dependencies {
            if !self
                .graph
                .node_indices()
                .any(|i| self.graph[i] == *dependency)
            {
                let index = self.graph.add_node(dependency.clone());
                self.add_dependency_edge(node_index, index);
            }
        }

        node_index
    }

    /// Adds a dependency edge between two packages in the graph.
    ///
    /// # Arguments
    ///
    /// * `source` - The `NodeIndex` of the source package.
    /// * `target` - The `NodeIndex` of the target package.
    pub fn add_dependency_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        self.graph.add_edge(source, target, "depends");
    }

    /// Prints the dependencies of a package up to a specified level.
    ///
    /// # Arguments
    ///
    /// * `package` - The package to print dependencies for.
    /// * `depth` - The current depth in the dependency tree.
    /// * `max_depth` - The maximum depth to print dependencies.
    pub fn print_dependencies_at_level(&self, package: &Package, depth: usize, max_depth: usize) {
        let mut visited_nodes = HashSet::new();
        let mut printed_packages = HashSet::new();
        self.print_dependencies_recursive(
            package,
            depth,
            max_depth,
            &mut visited_nodes,
            &mut printed_packages,
        );
    }

    /// Recursively prints the dependencies of a given package in a tree-like structure,
    /// with optional depth limit and color-coded output.
    ///
    /// # Arguments
    ///
    /// - `self`: A reference to the DependencyGraph struct containing the dependency graph.
    /// - `package`: A reference to the Package for which dependencies are printed.
    /// - `depth`: The current depth in the recursion. Used for indentation and color-coding.
    /// - `max_depth`: The maximum depth to explore in the dependency tree. Set to 0 for unlimited depth.
    /// - `visited_nodes`: A HashSet to keep track of visited nodes to avoid duplicates in the output.
    /// - `printed_packages`: A HashSet to keep track of printed packages to avoid redundant output.
    ///
    /// # Notes
    ///
    /// The function uses a Depth-First Search (DFS) traversal to explore the dependency graph.
    /// The DFS algorithm is chosen for its simplicity and suitability for exploring tree-like structures.
    /// The ANSI escape codes are used for color-coding the output based on the depth.
    ///
    /// - Green (32) is used for even depths.
    /// - White (37) is used for odd depths.
    pub fn print_dependencies_recursive(
        &self,
        package: &Package,
        depth: usize,
        max_depth: usize,
        visited_nodes: &mut HashSet<NodeIndex>,
        printed_packages: &mut HashSet<String>,
    ) {
        if depth < max_depth {
            let node_index = self
                .graph
                .node_indices()
                .find(|&index| self.graph[index] == (package.name.clone(), package.url.clone()))
                .unwrap_or_else(NodeIndex::end);

            if node_index != NodeIndex::end() && visited_nodes.insert(node_index) {
                let package_key = &package.name;
                if printed_packages.insert(package_key.clone()) || max_depth > 2 {
                    // ANSI escape code based on depth
                    // Green or white
                    let color_code = if depth % 2 == 0 { 32 } else { 37 };

                    println!(
                        "{:indent$}\x1b[{}m ├── {} - ({})\x1b[0m",
                        "",
                        color_code,
                        package.name,
                        package.url,
                        indent = depth * 3
                    );

                    let mut dfs = Dfs::new(&self.graph, node_index);
                    // dfs traversal
                    while let Some(neighbor_index) = dfs.next(&self.graph) {
                        let neighbor_package = Package::new(
                            self.graph[neighbor_index].clone().0,
                            self.graph[neighbor_index].clone().1,
                            vec![("".to_string(), "".to_string())],
                            false,
                        );
                        self.print_dependencies_recursive(
                            &neighbor_package,
                            depth + 1,
                            max_depth,
                            visited_nodes,
                            printed_packages,
                        );
                    }
                }
            }
        }
    }

    /// Generates a DOT format representation of the graph.
    ///
    /// # Returns
    ///
    /// Returns a `String` containing the DOT format representation.
    pub fn to_dot(&self) -> String {
        format!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::GraphContentOnly])
        )
    }
}
