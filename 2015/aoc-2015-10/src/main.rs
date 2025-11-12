fn look_and_say(num: usize) -> usize {
    let binding = num.to_string();
    let mut num_iter = binding.chars().peekable();
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

    say.parse::<usize>().unwrap()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_to_11() {
        let test = 1;
        assert_eq!(look_and_say(test), 11);
    }

    #[test]
    fn test_11_to_21() {
        let test = 11;
        assert_eq!(look_and_say(test), 21);
    }

    #[test]
    fn test_21_to_1211() {
        let test = 21;
        assert_eq!(look_and_say(test), 1211);
    }

    #[test]
    fn test_1211_to_111221() {
        let test = 1211;
        assert_eq!(look_and_say(test), 111221);
    }

    #[test]
    fn test_111221_to_312211() {
        let test = 111221;
        assert_eq!(look_and_say(test), 312211);
    }

    #[test]
    fn test_1_to_312211() {
        let mut test = 1;
        for _ in 0..4 {
            test = look_and_say(test);
        }

        assert_eq!(look_and_say(test), 312211);
    }
}
