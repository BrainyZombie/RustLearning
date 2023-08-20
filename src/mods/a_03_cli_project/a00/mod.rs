mod lib;
use lib::{run, Config};
use std::process;

pub fn main(args: Vec<String>) {
    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("CLI Read error: {e}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Running error: {e}");
        process::exit(1);
    }
}