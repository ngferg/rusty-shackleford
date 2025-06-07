use std::collections::HashMap;
use std::io;

fn main() {
    loop {
        println!("Enter your text: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.starts_with(".") || input.starts_with("_") {
            let alpha_string = get_alpha_string(input);

            println!("Decoded:\n{}", alpha_string)
        } else {
            let morse_string = get_morse_string(input);

            println!("Morse:\n{}", morse_string);
        }
    }
}

fn get_morse_string(input: String) -> String {
    let morse_map = get_alpha_to_morse_map();

    input
        .trim()
        .to_lowercase()
        .chars()
        .into_iter()
        .map(|c| morse_map.get(&c).unwrap_or(&"".to_string()).clone())
        .collect()
}

fn get_alpha_to_morse_map() -> HashMap<char, String> {
    HashMap::from([
        ('a', String::from("._ ")),
        ('b', String::from("_... ")),
        ('c', String::from("_._. ")),
        ('d', String::from("_.. ")),
        ('e', String::from(". ")),
        ('f', String::from(".._. ")),
        ('g', String::from("__. ")),
        ('h', String::from(".... ")),
        ('i', String::from(".. ")),
        ('j', String::from(".___ ")),
        ('k', String::from("_._ ")),
        ('l', String::from("._.. ")),
        ('m', String::from("__ ")),
        ('n', String::from("_. ")),
        ('o', String::from("___ ")),
        ('p', String::from(".__. ")),
        ('q', String::from("__._ ")),
        ('r', String::from("._. ")),
        ('s', String::from("... ")),
        ('t', String::from("_ ")),
        ('u', String::from(".._ ")),
        ('v', String::from("..._ ")),
        ('w', String::from(".__ ")),
        ('x', String::from("_.._ ")),
        ('y', String::from("_.__ ")),
        ('z', String::from("__.. ")),
        (' ', String::from("| ")),
    ])
}

fn get_alpha_string(input: String) -> String {
    let alpha_map = get_morse_to_alpha_map();

    input
        .trim()
        .to_lowercase()
        .split(" ")
        .into_iter()
        .map(|s| alpha_map.get(s).unwrap_or(&' '))
        .collect()
}

fn get_morse_to_alpha_map() -> HashMap<String, char> {
    get_alpha_to_morse_map()
        .iter()
        .map(|(k, v)| (v.clone().trim().to_string(), k.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let morse = get_morse_string("hello world".to_string());
        assert_eq!(".... . ._.. ._.. ___ | .__ ___ ._. ._.. _.. ", morse);
    }

    #[test]
    fn test_decode() {
        let alpha = get_alpha_string(".... . ._.. ._.. ___ | _ .... . ._. .".to_string());
        assert_eq!("hello there", alpha);
    }

    #[test]
    fn test_case_insensitivity() {
        let morse1 = get_morse_string("hOwDy".to_string());
        let morse2 = get_morse_string("HoWdY".to_string());
        assert_eq!(morse1, morse2);
    }
}
