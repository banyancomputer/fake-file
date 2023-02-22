//! fake-file

use std::path::PathBuf;
use clap::Parser;
use fake_file::{Structure};

mod cli;

/// Main entry point.
fn main() {
    // Parse command line arguments. see args.rs
    let cli = cli::Args::parse();
    let mut binding = Structure::new(cli.width, cli.depth, cli.size);
    let s = binding.with_strategy(cli.strategy);

    // If the output path is not specified, use the to_path_string() method
    // to get the current working directory.
    let output = if cli.output == "" {
        PathBuf::from(s.to_path_string())
    } else {
        PathBuf::from(cli.output)
    };

    // If the output path exists
    if output.exists() {
        // If the force flag is not set, exit with an error.
        if !cli.force {
            println!("Output path already exists. Use --force to overwrite.");
            std::process::exit(0);
        }
        // Delete the output path
        std::fs::remove_dir_all(&output).unwrap();
    }


    s.generate(& output).map_err(|e| {
        println!("Error Generating FS: {}", e);
    }).unwrap();
}
