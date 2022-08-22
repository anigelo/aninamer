mod lib;

use std::path::PathBuf;
use clap::Parser;
use lib::rename;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // Path to operate on, defaults to current directory
    #[clap(value_parser)]
    path: Option<String>,
    // Tests the renaming without applying
    #[clap(long)]
    test: bool
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
    let path = match cli.path {
        Some(user_path) if PathBuf::from(&user_path).exists() => PathBuf::from(user_path),
        _ => std::env::current_dir().unwrap()
    };
    rename(path, !cli.test);
}


