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

fn does_string_contain_double_letters(string: &str) -> bool {
    let chars_iter = string.chars();
    let chars_offset_iter = string.chars().skip(1);

    for (c1, c2) in chars_iter.zip(chars_offset_iter) {
        if c1 == c2 {
            return true;
        }
    }

    false
}

fn does_string_contain_bad(string: &str, bad_strings: &Vec<&str>) -> bool {
    for bad_string in bad_strings {
        if string.contains(bad_string) {
            return false;
        }
    }

    true
}

pub fn is_string_nice(unknown: &str, bad_strings: &Vec<&str>) -> bool {
    does_string_contain_vowels(unknown, 3) 
    & does_string_contain_double_letters(unknown)
    & does_string_contain_bad(unknown, bad_strings) 
}

fn does_string_contain_pair_substring(string: &str) -> bool {
    let mut chars = string.chars();
    let mut char_1 = chars.next().unwrap();
    let mut char_2 = chars.next().unwrap();

    for ch in chars {
        let substring = format!("{}{}", char_1, char_2);
        let occurences = string.match_indices(&substring).count();
        if occurences > 1 {
            return true;
        }

        char_1 = char_2;
        char_2 = ch;
    }

    false
}

fn does_string_contain_mirrored_substring(string: &str) -> bool {
    let mut chars = string.chars();
    let mut char_1 = chars.next().unwrap();
    let mut char_2 = chars.next().unwrap();
    let mut char_3 = chars.next().unwrap();

    for ch in chars {
        if char_1 == char_3 {
            return true;
        }

        char_1 = char_2;
        char_2 = char_3;
        char_3 = ch;
    }

    if char_1 == char_3 {
        return true;
    }

    false
}

pub fn is_string_new_nice(unknown: &str) -> bool {
    does_string_contain_pair_substring(unknown)
    & does_string_contain_mirrored_substring(unknown)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ugknbfddgicrmopn() {
        let test: &str = "ugknbfddgicrmopn";
        let bad: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
        assert_eq!(is_string_nice(test, &bad), true); 
    }

    #[test]
    fn test_aaa() {
        let test: &str = "aaa";
        let bad: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
        assert_eq!(is_string_nice(test, &bad), true); 
    }

    #[test]
    fn test_jchzalrnumimnmhp() {
        let test: &str = "jchzalrnumimnmhp";
        let bad: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
        assert_eq!(does_string_contain_double_letters(test), false);
        assert_eq!(is_string_nice(test, &bad), false);
    }

    #[test]
    fn test_haegwjzuvuyypxyu() {
        let test: &str = "haegwjzuvuyypxyu";
        let bad: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
        assert_eq!(does_string_contain_vowels(test, 3), true);
        assert_eq!(does_string_contain_double_letters(test), true);
        assert_eq!(does_string_contain_bad(test, &bad), false); 
    }

    #[test]
    fn test_qjhvhtzxzqqjkmpb() {
        let test: &str = "qjhvhtzxzqqjkmpb";
        assert_eq!(does_string_contain_pair_substring(test), true);
        assert_eq!(does_string_contain_mirrored_substring(test), true);
        assert_eq!(is_string_new_nice(test), true);
    }

    #[test]
    fn test_ieodomkazucvgmuy() {
        let test: &str = "ieodomkazucvgmuy";
        assert_eq!(does_string_contain_pair_substring(test), false);
        assert_eq!(does_string_contain_mirrored_substring(test), true);
        assert_eq!(is_string_new_nice(test), false);
    }

    #[test]
    fn test_a3() {
        let test: &str = "aaa";
        assert_eq!(is_string_new_nice(test), false);
    }

    #[test]
    fn test_a4() {
        let test: &str = "aaaa";
        assert_eq!(is_string_new_nice(test), true);
    }

    #[test]
    fn test_hpmbxtpfosbsjixt() {
        let test: &str = "hpmbxtpfosbsjixt";
        assert_eq!(is_string_new_nice(test), true);
    }

    #[test]
    fn test_dwmxqudvxqdenrur() {
        let test: &str = "dwmxqudvxqdenrur";
        assert_eq!(is_string_new_nice(test), true);
    }
}
