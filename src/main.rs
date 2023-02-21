//! fake-file

use clap::Parser;
use fake_file::{Structure, Strategy};

mod cli;

/// Main entry point.
fn main() {
    // Parse command line arguments. see args.rs
    let cli = cli::Args::parse();

    let s = Structure::new(cli.width, cli.depth, cli.target_size, Strategy::Balanced);
    s.generate(&cli.output).map_err(|e| {
        println!("Error Generating FS: {}", e);
    }).unwrap();
}
