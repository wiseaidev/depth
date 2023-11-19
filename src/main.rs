use clap::Parser;
use depth::cli::Cli;
use depth::visualize_dependency_tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let crate_ = &args.crate_;
    let levels = &args.levels;
    let optional = &args.optional;
    if *optional {
        if let Err(err) = visualize_dependency_tree(crate_, *levels + 1, true) {
            eprintln!("Error: {}", err);
        }
    } else if let Err(err) = visualize_dependency_tree(crate_, *levels + 1, false) {
        eprintln!("Error: {}", err);
    }

    Ok(())
}
