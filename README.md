# Depth

[![Crates.io](https://img.shields.io/crates/v/depth.svg)](https://crates.io/crates/depth)
[![Crates.io Downloads](https://img.shields.io/crates/d/depth)](https://crates.io/crates/depth)
![Rust](https://img.shields.io/badge/rust-stable-orange)
[![License](https://img.shields.io/crates/l/depth.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/wiseaidev/depth/workflows/Rust/badge.svg)](https://github.com/wiseaidev/depth/actions)

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
$ depth -c crate_name -l 1

# Or simply
$ depth -c crate_name
```

## ✨ Features

- **Fetch and Visualize Dependency Tree**: Fetch and visualize the dependency tree for a given Rust package using the Crates.io API.
- **Command-Line Tool**: Use the `depth` command-line tool to interactively explore and visualize dependencies.
- **Colorful Visualization**: Enhance the visualization with color-coding to represent different types of dependencies or levels of importance.
- **Multilevel Dependency Exploration**: Dive deeper into dependencies by supporting multilevel exploration, allowing you to inspect dependencies at various levels of depth.
- **Dependency Version Information**: Display version information for each dependency, including the version used by the current package.

Feel free to tailor these features based on the specific needs and goals of your Rust package management tool.
## 🌟 Examples

```bash
# Visualize dependencies for the 'input_yew' crate at level 1
$ depth -c input_yew
Dependencies for package 'input_yew':
 ├── input_yew - ()
    ├── web-sys - (^0.3.64)
    ├── yew - (^0.20.0)

# Or

$ depth -c yew -l 1
Dependencies for package 'input_yew':
 ├── input_yew - ()
    ├── web-sys - (^0.3.64)
    ├── yew - (^0.20.0)

$ depth -c input_yew -l 2
Dependencies for package 'input_yew':
 ├── input_yew - ()
    ├── web-sys - (^0.3.64)
    ├── yew - (^0.20.0)
    ├── web-sys - (https://rustwasm.github.io/wasm-bindgen/web-sys/index.html)
       ├── js-sys - (^0.3.65)
       ├── wasm-bindgen - (^0.2.88)
       ├── wasm-bindgen-futures - (^0.4.38)
       ├── wasm-bindgen-test - (^0.3.38)
       ├── js-sys - (https://rustwasm.github.io/wasm-bindgen/)
       ├── wasm-bindgen - (https://rustwasm.github.io/)
       ├── wasm-bindgen-futures - (https://rustwasm.github.io/wasm-bindgen/)
       ├── wasm-bindgen-test - ()
    ├── yew - (https://yew.rs)
       ├── futures - (^0.3)
       ├── gloo - (^0.10)
       ├── implicit-clone - (^0.4.1)
       ├── indexmap - (^2)
       ├── js-sys - (^0.3)
       ├── prokio - (^0.1.0)
       ├── rustversion - (^1)
       ├── serde - (^1)
       ├── slab - (^0.4)
       ├── thiserror - (^1.0)
       ├── tokio - (^1.32)
       ├── tracing - (^0.1.37)
       ├── trybuild - (^1)
       ├── wasm-bindgen - (^0.2)
       ├── wasm-bindgen-futures - (^0.4)
       ├── wasm-bindgen-test - (^0.3)
       ├── web-sys - (^0.3)
       ├── yew-macro - (^0.21.0)
       ├── console_error_panic_hook - ()
       ├── futures - (https://rust-lang.github.io/futures-rs)
       ├── gloo - (https://gloo-rs.web.app/)
       ├── implicit-clone - (https://github.com/yewstack/implicit-clone)
       ├── indexmap - ()
       ├── prokio - ()
       ├── rustversion - ()
       ├── serde - (https://serde.rs)
       ├── slab - ()
       ├── thiserror - ()
       ├── tokio - (https://tokio.rs)
       ├── tracing - (https://tokio.rs)
       ├── trybuild - ()
       ├── yew-macro - (https://github.com/yewstack/yew)
```

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/depth).
Your contributions help improve this crate for the community.

## 📄 License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
