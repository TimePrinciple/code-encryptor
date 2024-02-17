use std::{env, fs::read_to_string, io::stdin, path::PathBuf};

use magic_crypt::{new_magic_crypt, MagicCryptTrait};

fn main() {
    // Read command line argument, make sure it's a file and it does exist
    let file_path = env::args()
        .nth(1)
        .expect("Argument: path/to/file should be specified");
    let file_path = PathBuf::from(file_path);
    if !file_path.is_file() {
        panic!("{:?} is not a file", file_path);
    }

    // Ask for a key to encrypt the file
    println!("Encrypt key:");
    println!("------------------------------------------------");
    // Read user input as the encrypt key
    let mut key = String::new();
    stdin().read_line(&mut key).expect("Unable to read input");
    key = key.trim().to_string();

    // Start encryption
    // Read content to be encrypted
    let content_to_encrypt = read_to_string(&file_path)
        .expect(format!("Operation reading file {:?} failed", file_path).as_str());
    // Construct encryptor
    let encryptor = new_magic_crypt!(&key, 256);
    // Encrypt the content
    let encrypted_content = encryptor.encrypt_str_to_base64(content_to_encrypt);
    println!("Content encrypted successfully with key: {}", key);
    println!("------------------------------------------------");
    println!("{}", encrypted_content);
}
