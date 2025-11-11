fn look_and_say(num: &str) -> String {
    let mut num_iter = num.chars().peekable();
    let mut repeat: Vec<(char, u32)> = Vec::new();

    let mut count = 1;
    'look: while let Some(num) = num_iter.next() {
        if let Some(next) = num_iter.peek() {
            if num == *next {
                count += 1;
                continue 'look;
            }
        }

        repeat.push((num, count));
        count = 1;
    }

    let mut say: String = String::new();
    for r in repeat {
        say.push_str(&r.1.to_string());
        say.push(r.0);
    }

    say
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_to_11() {
        assert_eq!(look_and_say("1"), "11");
    }

    #[test]
    fn test_11_to_21() {
        assert_eq!(look_and_say("11"), "21");
    }

    #[test]
    fn test_21_to_1211() {
        assert_eq!(look_and_say("21"), "1211");
    }

    #[test]
    fn test_1211_to_111221() {
        assert_eq!(look_and_say("1211"), "111221");
    }

    #[test]
    fn test_111221_to_312211() {
        assert_eq!(look_and_say("111221"), "312211");
    }
}
