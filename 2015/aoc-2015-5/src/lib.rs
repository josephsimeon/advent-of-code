fn does_string_contain_vowels(string: &str, num: u32) -> bool {
    let mut counter = 0;

    for ch in string.chars() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => counter += 1,
            _ => continue,
        }
    }

    if counter >= num {
        return true;
    }

    false
}

fn does_string_contain_double_letters(string: &str, num: u32) -> bool {
    let mut counter = 0;

    let mut old_ch = string.chars().next().unwrap();
    for ch in string.chars().skip(1) {
        if old_ch == ch { counter += 1 };
        old_ch = ch;
    }

    if counter >= num {
        return true;
    }

    false
}

fn does_string_contain_bad(string: &str, bad_strings: Vec<&str>) -> bool {
    for bad_string in bad_strings {
        if string.contains(bad_string) {
            return false;
        }
    }

    true
}

pub fn is_nice_string(unknown: &str, bad_strings: Vec<&str>) -> bool {
    does_string_contain_vowels(unknown, 3) 
    & does_string_contain_double_letters(unknown, 1)
    & does_string_contain_bad(unknown, bad_strings) 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ugknbfddgicrmopn() {
        let test: &str = "ugknbfddgicrmopn";
        let bad: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
        assert_eq!(is_nice_string(test, bad), true); 
    }

    #[test]
    fn test_aaa() {
        let test: &str = "aaa";
        let bad: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
        assert_eq!(is_nice_string(test, bad), true); 
    }

    #[test]
    fn test_jchzalrnumimnmhp() {
        let test: &str = "jchzalrnumimnmhp";
        let bad: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
        assert_eq!(does_string_contain_double_letters(test, 1), false);
        assert_eq!(is_nice_string(test, bad), false);
    }

    #[test]
    fn test_haegwjzuvuyypxyu() {
        let test: &str = "haegwjzuvuyypxyu";
        let bad: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
        assert_eq!(does_string_contain_vowels(test, 3), true);
        assert_eq!(does_string_contain_double_letters(test, 1), true);
        assert_eq!(does_string_contain_bad(test, bad), false); 
    }
}
