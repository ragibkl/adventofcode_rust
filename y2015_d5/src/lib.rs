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
    ab.len() == 0
}

pub fn is_nice(string: &str) -> bool {
    has_three_vowels(string) && has_double_chars(string) && has_no_invalid_terms(string)
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

    #[test]
    fn test_is_nice() {
        assert_eq!(true, is_nice("aaa"));
        assert_eq!(true, is_nice("ugknbfddgicrmopn"));

        assert_eq!(false, is_nice("jchzalrnumimnmhp"));
    }
}
