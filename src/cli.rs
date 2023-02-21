use clap::{Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Output path")]
    pub output: PathBuf,
    #[arg(short, long, help = "Structure depth", default_value = "4")]
    pub depth: usize,
    #[arg(short, long, help = "Structure width", default_value = "4")]
    pub width: usize,
    #[arg(short, long, help = "Structure size", default_value = "1024")]
    pub target_size: usize,
}