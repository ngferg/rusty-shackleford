use std::{ env, error::Error, fs };


pub fn run(query: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_text = fs::read_to_string(file_path)?;

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    let found_lines: Vec<&str>;
    if ignore_case {
        found_lines = search_case_insensitive(query, &file_text);
    } else {
        found_lines = search(query, &file_text);
    }
    
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
