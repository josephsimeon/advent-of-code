use std::fs::File;
use std::io::{BufReader, BufRead};

fn byte_count_minus_chars(string: &str) -> usize {
    let mut count = 0;
    let mut chars_iter = string.chars();

    while let Some(ch) = chars_iter.next() {
        match ch {
            '"' => (),
            '\\' => {
                let next_ch = chars_iter.next().unwrap();

                if next_ch == 'x' {
                    chars_iter.next();
                    chars_iter.next();
                }

                count += 1;
            },
            _ => count += 1,
        }
    }

   string.len() - count
}

fn main() {
        let buf = BufReader::new(File::open("src/input.txt").expect("Unable to open src/input.txt"));
        
        let mut sum = 0;

        for line in buf.lines() {
            match line {
                Ok(parsed) => sum += byte_count_minus_chars(&parsed),
                _ => (),
            }
        }

        println!("{}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let buf = BufReader::new(File::open("src/test.txt").expect("Unable to open src/test.txt"));
        
        let mut sum = 0;
        
        for line in buf.lines() {
            match line {
                Ok(parsed) => sum += byte_count_minus_chars(&parsed),
                _ => (),
            }
        }

        assert_eq!(sum, 12);
    }
}
