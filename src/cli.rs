use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Mahmoud Harmouch",
    version = "0.0.2",
    about = "Visualize crate.io dependencies as a Tree",
    name = "Visualize Deps Tree"
)]
pub struct Cli {
    /// Sets the package to display
    #[arg(short = 'c', long = "crate")]
    pub crate_: String,
    /// Sets the levels to display.
    #[arg(short = 'l', long = "levels", default_value_t = 1)]
    pub levels: usize,
}
