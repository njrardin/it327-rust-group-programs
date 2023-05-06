use std::env;
use std::process;

use bnf_parser::Config;

fn main() {
    // gets args and stores in vector
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = bnf_parser::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
