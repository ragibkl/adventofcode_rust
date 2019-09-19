use std::fs;
use std::io::{BufRead, BufReader};

fn read_input_lines() -> Vec<String> {
    let filename = "input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

fn has_three_vowels(string: &str) -> bool {
    let vocals = ['a', 'e', 'i', 'o', 'u'];
    let ab: Vec<char> = string
        .chars()
        .filter(|x: &char| vocals.contains(x))
        .take(3)
        .collect();

    ab.len() >= 3
}

fn has_double_chars(string: &str) -> bool {
    let mut prev_c = string.chars().next().unwrap();
    for c in string.chars().skip(1) {
        if c == prev_c {
            return true;
        }
        prev_c = c;
    }
    false
}

fn has_no_invalid_terms(string: &str) -> bool {
    let excludes = ["ab", "cd", "pq", "xy"];
    let ab: Vec<&str> = excludes
        .iter()
        .filter(|x| string.contains(*x))
        .take(1)
        .map(|x| *x)
        .collect();
    println!("ab = {:?}", ab);
    ab.len() == 0
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
    fn test_has_three_vowels() {
        assert_eq!(true, has_three_vowels("aei"));
        assert_eq!(true, has_three_vowels("xazegov"));
        assert_eq!(true, has_three_vowels("aeiouaeiouaeiou"));

        assert_eq!(false, has_three_vowels("iop"));
        assert_eq!(false, has_three_vowels("batman"));
        assert_eq!(false, has_three_vowels("super"));
    }

    #[test]
    fn test_has_double_chars() {
        assert_eq!(true, has_double_chars("xx"));
        assert_eq!(true, has_double_chars("abcdde"));
        assert_eq!(true, has_double_chars("aabbccdd"));

        assert_eq!(false, has_double_chars("abcde"));
        assert_eq!(false, has_double_chars("xkcd"));
    }

    #[test]
    fn test_has_no_invalid_terms() {
        assert_eq!(true, has_no_invalid_terms("rust"));
        assert_eq!(true, has_no_invalid_terms("python"));

        assert_eq!(false, has_no_invalid_terms("ab"));
        assert_eq!(false, has_no_invalid_terms("sfdcdsdf"));
    }
}
