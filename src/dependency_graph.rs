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
//! use depth::dependency_graph::{DependencyGraph, fetch_package_info, Package};
//! use crates_io_api::SyncClient;
//! ```
//!
//! 2. Create an instance of `DependencyGraph` and use it to fetch and visualize dependency trees:
//!
//! ```rust
//! let mut graph = DependencyGraph::new();
//! graph.fetch_dependency_tree("your_package_name", 2);
//! graph.print_dependencies_at_level(/* ... */);
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
//! let mut graph = DependencyGraph::new();
//! graph.fetch_dependency_tree("your_package_name", 2);
//! ```
//!
//! ## Visualizing Dependencies
//!
//! Utilize the `print_dependencies_at_level` method to print dependencies at a specified depth in the dependency tree:
//!
//! ```rust
//! let package = Package::new(/* ... */);
//! graph.print_dependencies_at_level(&package, 0, 2);
//! ```
//!
//! # Examples
//!
//! ```rust
//! use depth::dependency_graph::{DependencyGraph, Package};
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
use std::collections::HashMap;

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
                self.graph.add_node(dependency.clone());
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
        if depth < max_depth {
            println!(
                "{:indent$} ├── {} - ({})",
                "",
                package.name,
                package.url,
                indent = depth * 2
            );

            for dependency in &package.dependencies {
                let child_index = self
                    .graph
                    .node_indices()
                    .find(|&index| &self.graph[index] == dependency)
                    .unwrap_or_else(NodeIndex::end);

                if child_index != NodeIndex::end() {
                    let child_package = Package::new(
                        self.graph[child_index].clone().0,
                        self.graph[child_index].clone().1,
                        vec![("".to_string(), "".to_string())],
                        false,
                    );
                    self.print_dependencies_at_level(&child_package, depth + 1, max_depth);
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
