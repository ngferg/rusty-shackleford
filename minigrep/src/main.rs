use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        panic!("Incorrect usage, try: minigrep searchString file.txt")
    }

    let query = &args[1];
    let file_path = &args[2];
    println!("Searching {} for {}...", file_path, query);
}
