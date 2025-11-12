fn look_and_say(num: &String) -> String {
    let mut num_iter = num.chars().peekable();
    let mut say: String = String::new();

    let mut count = 1;
    'look: while let Some(num) = num_iter.next() {
        if let Some(next) = num_iter.peek() {
            if num == *next {
                count += 1;
                continue 'look;
            }
        }
        
        let s = format!("{count}{num}");
        say.push_str(&s);
        count = 1;
    }

    say
}

fn main() {
    let mut say: String = "1321131112".to_string();

    for _ in 0..40 {
        say = look_and_say(&say);
    }

    print!("{}", say.len());

    for i in 0..10 {
        say = look_and_say(&say);
    }

    println!(", {}", say.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_to_11() {
        let test: String = "1".to_string();
        assert_eq!(look_and_say(&test), "11");
    }

    #[test]
    fn test_11_to_21() {
        let test: String = "11".to_string();
        assert_eq!(look_and_say(&test), "21");
    }

    #[test]
    fn test_21_to_1211() {
        let test: String = "21".to_string();
        assert_eq!(look_and_say(&test), "1211");
    }

    #[test]
    fn test_1211_to_111221() {
        let test: String = "1211".to_string();
        assert_eq!(look_and_say(&test), "111221");
    }

    #[test]
    fn test_111221_to_312211() {
        let test: String = "111221".to_string();
        assert_eq!(look_and_say(&test), "312211");
    }

    #[test]
    fn test_1_to_312211() {
        let mut test: String = "1".to_string();

        for _ in 0..4 {
            test = look_and_say(&test);
        }

        assert_eq!(look_and_say(&test), "312211");
    }
}
