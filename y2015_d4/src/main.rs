extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn hash_int(int: i32, prefix: &str) -> String {
    let mut hasher = Md5::new();
    // let prefix = "bgvyzdsv";
    hasher.input_str(&prefix);
    hasher.input_str(&int.to_string());
    let hash = hasher.result_str();
    return hash.to_string();
}

fn check_is_valid(int: i32, zeros: i32, prefix: &str) -> bool {
    let hash = hash_int(int, prefix);
    let start_str = (0..zeros).map(|_| "0").collect::<String>();
    hash.starts_with(&start_str)
}

fn main() {
    let input = "bgvyzdsv";
    for i in 1.. {
        if check_is_valid(i, 5, &input) {
            println!("i = {}", i);
            break;
        }
    }

    for i in 1.. {
        if check_is_valid(i, 6, &input) {
            println!("i = {}", i);
            break;
        }
    }
}
