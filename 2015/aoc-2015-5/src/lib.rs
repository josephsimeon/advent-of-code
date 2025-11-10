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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_haegwjzuvuyypxyu() {
        let test: &str = "haegwjzuvuyypxyu";
        let bad: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
        assert_eq!(does_string_contain_vowels(test, 3), true);
    }
}
