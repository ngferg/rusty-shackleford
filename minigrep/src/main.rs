use std::{env, process};

use minigrep::{parse_config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(query, file_path) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
