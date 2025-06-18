mod enums;
mod locker;
use enums::Actions;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use openssl::{hash::MessageDigest, pkcs5::pbkdf2_hmac, rand::rand_bytes, symm::{decrypt, encrypt, Cipher}};
use rand::{distributions::Alphanumeric, Rng};
use std::{char, fs, io, path::Path};

const SPECIAL_CHARS: [char; 5] = ['!', '#', '*', '-', '&'];
const TEST_STRING: &[u8; 4] = b"test";

fn main() -> io::Result<()> {
    let password_locker = startup();

    main_loop(password_locker);

    shutdown();
    Ok(())
}

fn startup() -> locker::PasswordLocker {
    print!("\r\nWelcome to fpass!\r\n");    
    let password_locker = setup_configs();
    enable_raw_mode().expect("Unable to get terminal in raw mode");
    password_locker
}

fn setup_configs() -> locker::PasswordLocker {
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let folder_path = format!("{home_dir}/.fpass");

    let password_locker: locker::PasswordLocker;

    if Path::new(&folder_path).is_dir() {
        print!("Loading pre-existing configs...\r\n");
        password_locker = load_key(&folder_path);
    } else {
        println!("First time use, creating config folder at ~/.fpass\r\n");
        fs::create_dir(&folder_path).expect("Unable to create config directory");
        password_locker = set_up_key(&folder_path);
    }
    password_locker
}

fn shutdown() {
    println!("Goodbye!");
    disable_raw_mode().expect("Unable to disable terminal's raw mode");
}

fn main_loop(password_locker: locker::PasswordLocker) {
    let mut print_menu = true;
    loop {
        let action: Actions = get_action_from_cli(print_menu);
        print_menu = false;

        match action {
            Actions::QUIT => break,
            Actions::UNKNOWN => continue,
            Actions::NEWPASSOWRD => {
                create_password(&password_locker);
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

    if let Event::Key(key_event) = event::read().expect("cannot read key") {

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => Actions::QUIT,
            KeyCode::Char('n') => Actions::NEWPASSOWRD,
            _ => Actions::UNKNOWN,
        }   
    } else {
        Actions::UNKNOWN
    }

}

fn create_password(password_locker: &locker::PasswordLocker) {
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
    let password = encrypt_password(password, &password_locker);
    print!("\r\n\r\nEncrypted password: {password}\r\n\r\r");
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

fn encrypt_password(password: String, password_locker: &locker::PasswordLocker) -> String {
    let cipher = Cipher::aes_256_cbc();
    let bytes = encrypt(cipher, &password_locker.key, Some(&password_locker.iv), password.as_bytes()).expect("failed to encrypt");
    String::from_utf8_lossy(&bytes).into_owned()
}

fn load_key(home_dir: &String) -> locker::PasswordLocker {
    
    let salt: [u8; 8] = fs::read(format!("{home_dir}/aes_salt.bin")).expect("unable to read salt").try_into().expect("Salt malformatted");
    let iv: [u8; 16] = fs::read(format!("{home_dir}/aes_iv.bin")).expect("unable to read iv").try_into().expect("iv malformatted");
    let test_value = fs::read(format!("{home_dir}/encrypted.bin")).expect("unable to read test value");

    println!("Please enter master password...");
    let mut master_password = String::from("");
    io::stdin()
        .read_line(&mut master_password)
        .expect("Failed to read line");
    master_password = master_password.trim().to_string();

    let mut key = [0u8; 32];
    pbkdf2_hmac(
        master_password.as_bytes(), // Password bytes
        &salt,                          // Salt
        100_000,                 // Iteration count (higher = more secure, slower)
        MessageDigest::sha256(),// Hash function (SHA-256)
        &mut key,                   // Output key
    ).expect("Unable to generate key");
    let cipher = Cipher::aes_256_cbc();

    let decrypted = decrypt(cipher, &key, Some(&iv), &test_value).expect("decryption failed");
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));

    locker::PasswordLocker { salt: salt, key: key, iv: iv }
}

fn set_up_key(home_dir: &String) -> locker::PasswordLocker {
    println!("First time use, let's set up a master password...");
    let mut p1 = "1".to_string();
    let mut p2 = "2".to_string();

    while p1 != p2 {
        p1 = String::from("");
        p2 = String::from("");

        println!("Enter password: ");
        io::stdin()
            .read_line(&mut p1)
            .expect("Failed to read line");
        p1 = p1.trim().to_string();
        println!("Enter password again: ");
        io::stdin()
            .read_line(&mut p2)
            .expect("Failed to read line");
        p2 = p2.trim().to_string();

        if p1 != p2 {
            println!("Password don't match, try again");
        }
    }

    let mut salt = [0u8; 8];
    rand_bytes(&mut salt).expect("Unable to generate key");
    fs::write(format!("{home_dir}/aes_salt.bin"), &salt).expect("unable to write salt");

    let mut key = [0u8; 32];
    pbkdf2_hmac(
        p1.as_bytes(),              // Password bytes
        &salt,                          // Salt
        100_000,                 // Iteration count (higher = more secure, slower)
        MessageDigest::sha256(),// Hash function (SHA-256)
        &mut key,                   // Output key
    ).expect("Unable to generate key");
    fs::write(format!("{home_dir}/aes_key.bin"), &key).expect("unable to write key");

    let mut iv = [0u8; 16];
    rand_bytes(&mut iv).expect("Unable to generate key");
    fs::write(format!("{home_dir}/aes_iv.bin"), &iv).expect("unable to write iv");

    let cipher = Cipher::aes_256_cbc();
    let encrypted = encrypt(cipher, &key, Some(&iv), TEST_STRING).expect("failed to encrypt");
    println!("writing to {home_dir}...");
    fs::write(format!("{home_dir}/encrypted.bin"), &encrypted).expect("failed to encrypt");
    locker::PasswordLocker { key, salt, iv }
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
