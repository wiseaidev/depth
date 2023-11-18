# Depth

[![Crates.io](https://img.shields.io/crates/v/depth.svg)](https://crates.io/crates/depth)
[![License](https://img.shields.io/crates/l/depth.svg)](https://opensource.org/licenses/MIT)

> 🚀 `depth`: A command-line tool for fetching and visualizing dependency trees for Rust packages.

## 📖 Table of Contents

- [Installation](#-installation)
- [Usage](#-usage)
- [Features](#-features)
- [Examples](#-examples)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Installation

To install `depth`, use the following Cargo command:

```bash
cargo install --locked depth
```

## 🛠️ Usage

Use the `depth` command to visualize dependency trees. Here are some examples:

```bash
# Visualize dependencies at level 1
depth -c crate_name -l 2

# Or simply
depth -c crate_name
```

## ✨ Features

- **Fetch and Visualize Dependency Tree**: Fetch and visualize the dependency tree for a given Rust package using the Crates.io API.
- **Command-Line Tool**: Use the `depth` command-line tool to interactively explore and visualize dependencies.

## 🌟 Examples

```bash
# Visualize dependencies for the 'input_yew' crate at level 1
depth -c input_yew

# Or simply

depth -c yew -l 2
```

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/depth).
Your contributions help improve this crate for the community.

## 📄 License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
