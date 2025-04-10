mod enums;
use enums::Actions;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::{distributions::Alphanumeric, Rng};
use std::{char, fs, io, path::Path};

const SPECIAL_CHARS: [char; 5] = ['!', '#', '*', '-', '&'];


fn main() -> io::Result<()> {
    startup();

    main_loop();

    shutdown();
    Ok(())
}

fn startup() {
    print!("\r\nWelcome to fpass!\r\n");    
    setup_configs();
    enable_raw_mode().expect("Unable to get terminal in raw mode");
}

fn setup_configs() {
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let folder_path = format!("{home_dir}/.fpass");

    if Path::new(&folder_path).is_dir() {
        print!("Loading pre-existing configs...\r\n");
    } else {
        println!("First time use, creating config folder at ~/.fpass\r\n");
        fs::create_dir(folder_path).expect("Unable to create config directory");
    }
}

fn shutdown() {
    println!("Goodbye!");
    disable_raw_mode().expect("Unable to disable terminal's raw mode");
}

fn main_loop() {
    let mut print_menu = true;
    loop {
        let action: Actions = get_action_from_cli(print_menu);
        print_menu = false;

        match action {
            Actions::QUIT => break,
            Actions::UNKNOWN => continue,
            Actions::NEWPASSOWRD => {
                create_password();
                print_menu = true;
            },
        }
    }
}

fn get_action_from_cli(print_menu: bool) -> Actions {
    if print_menu {
        print!("Main menu:\r\n");
        print!("(q)uit\r\n");
        print!("(n)ew password\r\n");
    }

    if let Event::Key(key_event) = event::read().expect("cannot ready key") {

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => Actions::QUIT,
            KeyCode::Char('n') => Actions::NEWPASSOWRD,
            _ => Actions::UNKNOWN,
        }   
    } else {
        Actions::UNKNOWN
    }

}

fn create_password() {
    disable_raw_mode().expect("Unable to disable terminal's raw mode");

    let len: u16;

    loop {
        print!("How long would you like the password to be:\r\n");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let len_result = input.trim().parse();
        match len_result {
            Ok(l) => {
                len = l;
                break;
            },
            Err(_) => continue,
        }
    }
    enable_raw_mode().expect("Unable to get terminal in raw mode");
    print!("\r\nWould you like special chars? (y)es or (n)o\r\n");
    let special_chars_allowed: bool;
    
    if let Event::Key(key_event) = event::read().expect("cannot ready key") {
        match key_event.code {
            KeyCode::Char('y') => special_chars_allowed = true,
            _ => special_chars_allowed = false,
        }
    } else {
        special_chars_allowed = false;
    }

    let password = generate_password(len, special_chars_allowed);
    print!("\r\n\r\nNew password generated: {password}\r\n\r\r");
}

fn generate_password(len: u16, special_chars_allowed: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut password = "".to_string();
    let mut has_special_char = false;

    for i in 0..len {
        if rng.gen_bool(0.2) && special_chars_allowed {
            password.push(SPECIAL_CHARS[rng.gen_range(0..SPECIAL_CHARS.len())]);
            has_special_char = true;
        } else {
            if i == len - 1 && special_chars_allowed && !has_special_char {
                password.push(SPECIAL_CHARS[rng.gen_range(0..SPECIAL_CHARS.len())]);
            } else {
                password.push(rng.sample(Alphanumeric) as char);
            }
        }
    }
    password
}










#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_len_works() {
        for len in 1..=100 {
            let special_password = generate_password(len, true);
            let normal_password = generate_password(len, false);

            assert_eq!(len, special_password.len() as u16);
            assert_eq!(len, normal_password.len() as u16);
        }
    }

    #[test]
    fn password_special_has_special_chars() {
        for _ in 0..100 {
            let special_password = generate_password(5, true);
            let mut has_special_char = false;

            for c in SPECIAL_CHARS {
                has_special_char |= special_password.contains(c);
            }

            assert!(has_special_char);
        }
    }

    #[test]
    fn password_regular_has_no_special_chars() {
        for _ in 0..100 {
            let special_password = generate_password(5, false);
            let mut has_special_char = false;

            for c in SPECIAL_CHARS {
                has_special_char |= special_password.contains(c);
            }

            assert!(!has_special_char);
        }
    }
}
