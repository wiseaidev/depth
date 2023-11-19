//! # package
//!
//! The `package` module provides functionality for parsing and fetching information about Rust packages.
//! It includes tools for parsing dependencies from `Cargo.toml` files, fetching package information from
//! Crates.io, and building a dependency graph.
//!
//! # Quick Start
//!
//! Get started with the `package` module by following these simple steps:
//!
//! 1. Import the necessary types and functions into your code:
//!
//! ```rust
//! use depth::package::{Package, parse_dependencies, fetch_package_info};
//! use depth::dependency_graph::DependencyGraph;
//! use crates_io_api::SyncClient;
//! ```
//!
//! 2. Utilize the provided functionality to parse dependencies and fetch package information:
//!
//! ```rust
//! use depth::package::{Package, parse_dependencies, fetch_package_info};
//! use depth::dependency_graph::DependencyGraph;
//! use std::collections::HashMap;
//!
//! let cargo_toml_content = "..."; // Contents of your Cargo.toml file
//! // let dependencies = parse_dependencies(&cargo_toml_content)?;
//! let mut visited_packages: HashMap<String, Package> = HashMap::new();
//! let mut graph = DependencyGraph::new();
//!
//! // for dep in dependencies {
//!     // fetch_package_info(&dep, &mut visited_packages, &mut graph, &client, 2)?;
//! // }
//! ```
//!
//! # Key Features
//!
//! The `package` module offers the following key features:
//!
//! - **Parsing Dependencies**: Parse dependencies from `Cargo.toml` files using the `parse_dependencies` function.
//! - **Fetching Package Information**: Fetch detailed package information from Crates.io using the `fetch_package_info` function.
//!
//! # Usage
//!
//! ## Parsing Dependencies
//!
//! Use the `parse_dependencies` function to parse dependencies from `Cargo.toml` content:
//!
//! ```rust
//! use depth::package::parse_dependencies;
//!
//! let cargo_toml_content = "..."; // Contents of your Cargo.toml file
//! // let dependencies = parse_dependencies(&cargo_toml_content)?;
//! ```
//!
//! ## Fetching Package Information
//!
//! Utilize the `fetch_package_info` function to fetch and build the dependency tree for a specific package:
//!
//! ```rust
//! use depth::package::{Package, fetch_package_info};
//! use depth::dependency_graph::DependencyGraph;
//! use std::collections::HashMap;
//! use crates_io_api::SyncClient;
//!
//! let mut visited_packages: HashMap<String, Package> = HashMap::new();
//! let mut graph = DependencyGraph::new();
//! // let client = SyncClient::new(
//! //     "my-user-agent (my-contact@domain.com)",
//! //   std::time::Duration::from_millis(1000),
//! // )?;
//!
//! // fetch_package_info(
//! //     &("package_name".to_string(), "homepage_url".to_string()),
//! //     &mut visited_packages,
//! //     &mut graph,
//! //     &client,
//! //     2,
//! // )?;
//! ```
//!
//! # Examples
//!
//! ```rust
//! use depth::package::{Package, parse_dependencies, fetch_package_info};
//! use depth::dependency_graph::DependencyGraph;
//! use crates_io_api::SyncClient;
//! use std::collections::HashMap;
//!
//! let cargo_toml_content = "..."; // Contents of your Cargo.toml file
//! // let dependencies = parse_dependencies(&cargo_toml_content).unwrap();
//!
//! let mut visited_packages: HashMap<String, Package> = HashMap::new();
//! let mut graph = DependencyGraph::new();
//! // let client = SyncClient::new(
//! //     "my-user-agent (my-contact@domain.com)",
//! //     std::time::Duration::from_millis(1000),
//! // ).unwrap();
//!
//! // for dep in dependencies {
//! //     fetch_package_info(&(dep, "".to_string()), &mut visited_packages, &mut graph, &client, 2).unwrap();
//! // }
//! ```

use crate::dependency_graph::DependencyGraph;
use crates_io_api::{Crate, Error as CratesIoError, SyncClient};
use std::collections::HashMap;
use std::error::Error;
use toml::Value;

/// Represents a Rust package with its name, URL, dependencies, and internal status.
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub url: String,
    pub dependencies: Vec<(String, String)>,
    pub internal: bool,
}

impl Package {
    /// Creates a new Package instance with the given parameters.
    pub fn new(
        name: String,
        url: String,
        dependencies: Vec<(String, String)>,
        internal: bool,
    ) -> Self {
        Package {
            name,
            url,
            dependencies,
            internal,
        }
    }
}

/// Parses the dependencies from the content of a Cargo.toml file.
///
/// # Arguments
///
/// * `cargo_toml_content` - The content of the Cargo.toml file as a string.
///
/// # Returns
///
/// A Result containing a Vec of dependency names or an error if parsing fails.
pub fn parse_dependencies(
    cargo_toml_content: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cargo_toml: Value = cargo_toml_content.parse()?;

    if let Some(Value::Table(dependencies_table)) = cargo_toml.get("dependencies") {
        let dependencies: Vec<String> = dependencies_table
            .iter()
            .map(|(dep_name, _)| dep_name.clone())
            .collect();
        return Ok(dependencies);
    }

    Ok(Vec::new())
}

/// Fetches package information, including dependencies, from Crates.io and builds a dependency graph.
///
/// # Arguments
///
/// * `package_name` - A tuple containing the package name and its homepage URL.
/// * `visited_packages` - A mutable HashMap to store visited packages and prevent redundant fetching.
/// * `graph` - A mutable reference to a DependencyGraph where package information will be stored.
/// * `client` - A SyncClient instance for interacting with the Crates.io API.
/// * `depth` - The depth up to which dependencies should be fetched and added to the graph.
/// * `optional` - A boolean to scan optional dependencies only.
///
/// # Returns
///
/// A Result containing an optional Package or an error if the fetching process fails.
pub fn fetch_package_info(
    package_name: &(String, String),
    visited_packages: &mut HashMap<String, Package>,
    graph: &mut DependencyGraph,
    client: &SyncClient,
    depth: usize,
    optional: bool,
) -> Result<Option<Package>, Box<dyn Error>> {
    if let Some(package) = visited_packages.get(&package_name.0) {
        return Ok(Some(package.clone()));
    }

    let crate_info = client.get_crate(&package_name.0)?.crate_data;

    let homepage = crate_info.clone().homepage.unwrap_or("".to_string());
    let dependencies = list_dependencies(client, &crate_info, optional)?;

    let internal = package_name.0.starts_with("std");

    let package = Package::new(
        package_name.0.to_string(),
        homepage,
        dependencies.clone(),
        internal,
    );
    visited_packages.insert(package_name.0.to_string(), package.clone());

    let node_index = graph.add_package_to_graph(&package);

    // Add dependencies to the graph up to the specified depth
    if depth > 1 {
        for dependency in &dependencies {
            if let Some(child_package) = fetch_package_info(
                dependency,
                visited_packages,
                graph,
                client,
                depth - 1,
                optional,
            )? {
                let child_index = graph.add_package_to_graph(&child_package);
                graph.add_dependency_edge(node_index, child_index);
            }
        }
    }

    Ok(Some(package))
}

/// Lists dependencies for a given crate from the Crates.io API.
///
/// # Arguments
///
/// * `client` - A SyncClient instance for interacting with the Crates.io API.
/// * `crate_info` - A reference to the Crate information obtained from Crates.io.
/// * `optional` - A boolean to scan optional dependencies only.
///
/// # Returns
///
/// A Result containing a Vec of dependency tuples or an error if fetching fails.
fn list_dependencies(
    client: &SyncClient,
    crate_info: &Crate,
    optional: bool,
) -> Result<Vec<(String, String)>, CratesIoError> {
    let mut dependencies = Vec::new();

    for dep in client.crate_dependencies(&crate_info.id, &crate_info.max_version)? {
        if !dep.optional && !optional {
            dependencies.push((dep.crate_id.clone(), dep.req.to_string()));
        } else if optional && dep.optional {
            dependencies.push((dep.crate_id.clone(), dep.req.to_string()));
        }
    }

    Ok(dependencies)
}
