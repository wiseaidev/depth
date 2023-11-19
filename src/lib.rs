//! # depth
//!
//! The `depth` crate provides a command-line tool for fetching and visualizing dependency trees
//! for Rust packages using the Crates.io API and the `petgraph` crate. With this tool, you can explore
//! the dependencies of a given Rust package and visualize the relationship between different packages
//! in a dependency graph.
//!
//! # Quick Start
//!
//! Get started with the `depth` crate by following these simple steps:
//!
//! 1. Install the `depth` tool using Cargo:
//!
//! ```bash
//! cargo install --locked depth
//! ```
//!
//! 2. Use the `depth` command to visualize dependency trees:
//!
//! ```bash
//! # Visualize dependencies at level 1
//! depth -c crate_name -l 1
//!
//! or simply:
//!
//! depth -c crate_name
//! ```
//!
//! # Key Features
//!
//! The `depth` crate offers the following key features:
//!
//! - **Fetch and Visualize Dependency Tree**: Fetch and visualize the dependency tree for a given Rust package using the Crates.io API.
//! - **Command-Line Tool**: Use the `depth` command-line tool to interactively explore and visualize dependencies.
//!
//! # GitHub Repository
//!
//! You can access the source code for this crate on [GitHub](https://github.com/wiseaidev/depth).
//!
//! # Contributing
//!
//! Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement,
//! please engage with the project on [GitHub](https://github.com/wiseaidev/depth).
//! Your contributions help improve this crate for the community.

pub mod cli;
pub mod dependency_graph;
pub mod package;

use std::error::Error;

use dependency_graph::DependencyGraph;

/// Visualizes the dependency tree for a given package.
///
/// # Arguments
///
/// * `package_name` - The name of the package to visualize.
/// * `depth` - The depth up to which dependencies should be visualized.
///
/// # Returns
///
/// A Result indicating success or an error if the visualization process fails.
pub fn visualize_dependency_tree(
    package_name: &str,
    depth: usize,
    optional: bool,
) -> Result<(), Box<dyn Error>> {
    let mut graph = DependencyGraph::new();

    if let Some(root_package) = graph.fetch_dependency_tree(package_name, depth, optional)? {
        // Print dependencies
        println!("Dependencies for package '{}':", package_name);
        graph.print_dependencies_at_level(&root_package, 0, depth);

        // Visualize the graph (commented out for now)
        // println!("{}", graph.to_dot());
    } else {
        eprintln!("Package not found or does not have a Cargo.toml file");
    }

    Ok(())
}
