use std::io::Read;

use interprebear::*;
use clap::Parser;

#[derive(clap::Parser)]
struct Cli {
    /// Path to the file to be run.
    path: String,

    /// Should the interpreter step through what the bear is doing.
    #[arg(short, long)]
    step_through: bool
}

fn main() {
    let cli = Cli::parse();
    
    let path = &cli.path;
    let file = std::fs::File::open(path).expect("couldn't open file");
    let mut reader = std::io::BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf).expect("error reading from file");

    let chunk = interprebear::Parser::parse(&buf);
    if cli.step_through {
        Interpreter::step_through(chunk);
    } else {
        Interpreter::run(chunk);
    }
}

