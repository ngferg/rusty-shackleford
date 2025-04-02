use std::{ error::Error, fs };


pub fn run(query: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_text = fs::read_to_string(file_path)?;

    let found_lines = search(query, &file_text);

    for line in found_lines {
        println!("{line}");
    }

    Ok(())
}

pub fn parse_config(args: &[String]) -> Result<(&str, &str), &'static str> {
    if args.len() < 3 {
        return Err("Incorrect usage, try: minigrep searchString file.txt");
    }

    let query = &args[1];
    let file_path = &args[2];

    Ok((query, file_path))
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}