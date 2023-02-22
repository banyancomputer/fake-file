use clap::{Parser};
use fake_file::Strategy;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Structure depth", default_value = "4")]
    pub depth: usize,
    #[arg(short, long, help = "Structure width", default_value = "4")]
    pub width: usize,
    #[arg(short, long, help = "Structure size", default_value = "1024")]
    pub size: usize,
    #[arg(long, help = "Strategy", default_value = "Simple")]
    pub strategy: Strategy,
    #[arg(short, long, help = "Output path", default_value = "")]
    pub output: String,
    #[arg(short, long, help = "Force overwrite", default_value = "false")]
    pub force: bool,
    #[arg(short, long, help = "Verbose output", default_value = "false")]
    pub verbose: bool,
}