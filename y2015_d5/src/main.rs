use std::fs;
use std::io::{BufRead, BufReader};

fn read_input_lines() -> Vec<String> {
    let filename = "input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

fn has_three_vowel(string: &str) -> bool {
    let vocals = ['a', 'e', 'i', 'o', 'u'];
    let ab: Vec<char> = string
        .chars()
        .filter(|x: &char| vocals.contains(x))
        .take(3)
        .collect();

    ab.len() >= 3
}

fn contains_double_chars(string: &str) -> bool {
    let mut prev_c = string.chars().next().unwrap();
    for c in string.chars().skip(1) {
        if c == prev_c {
            return true;
        }
        prev_c = c;
    }
    false
}

fn main() {
    let lines = read_input_lines();
    for line in lines {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_three_vowel() {
        assert_eq!(true, has_three_vowel("aei"));
        assert_eq!(true, has_three_vowel("xazegov"));
        assert_eq!(true, has_three_vowel("aeiouaeiouaeiou"));

        assert_eq!(false, has_three_vowel("iop"));
        assert_eq!(false, has_three_vowel("batman"));
        assert_eq!(false, has_three_vowel("super"));
    }

    #[test]
    fn test_contains_double_chars() {
        assert_eq!(true, contains_double_chars("xx"));
        assert_eq!(true, contains_double_chars("abcdde"));
        assert_eq!(true, contains_double_chars("aabbccdd"));

        assert_eq!(false, contains_double_chars("abcde"));
        assert_eq!(false, contains_double_chars("xkcd"));
    }
}
