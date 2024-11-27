
use std::io::Read;

use interprebear::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: interprebear [path]");
        return;
    }
    
    let path = &args[1];
    let file = std::fs::File::open(path).expect("couldn't open file");
    let mut reader = std::io::BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf).expect("error reading from file");

    let chunk = Parser::parse(&buf);
    if args.len() > 2 && args[2] == "-d" {
        Interpreter::step_through(chunk);
    } else {
        Interpreter::run(chunk);
    }
}

