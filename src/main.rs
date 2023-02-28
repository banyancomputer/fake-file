//! fake-file

use clap::Parser;
use fake_file::Structure;
use std::path::PathBuf;

mod cli;

/// Main entry point.
fn main() {
    // Parse command line arguments. see args.rs
    let cli = cli::Args::parse();
    let s = Structure::new(cli.width, cli.depth, cli.size, cli.strategy);

    // If the output path is not specified, use the to_path_string() method
    // to get the current working directory.

    let mut output = PathBuf::from(cli.output);
    output.push(s.to_path_string());

    // If the output path exists
    if output.exists() {
        // If the force flag is not set, exit with an error.
        if !cli.force {
            println!("Output path already exists. Use --force to overwrite.");
            println!("Path: {:?}", output);
            std::process::exit(0);
        }
        if cli.verbose {
            println!("WARN: Output path already exists. Overwriting.");
            println!("Path: {:?}", output);
        }
        // Delete the output path
        std::fs::remove_dir_all(&output).unwrap();
    }

    // If verbose output is enabled, print the structure
    if cli.verbose {
        println!("Generating File Structure: {:?}", s);
        println!("Output Path: {:?}", output);
    }

    let now = std::time::Instant::now();

    s.generate(&output)
        .map_err(|e| {
            println!("Error Generating File Structure: {}", e);
        })
        .unwrap();

    let elapsed = now.elapsed().as_secs_f64();
    if cli.verbose {
        println!("Completed in: {:?} s", elapsed);
    }
}
