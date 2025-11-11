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

fn new_byte_count_minus_length(string: &str) -> usize {
    let mut counter = 0;
    let mut new_string: String = String::new();

    for (i, ch) in string.chars().enumerate() {
        match ch {
            '"' => {
                if i == 0 {
                    new_string.push('"');
                    counter += 1;
                }

                new_string.push_str("\"");
                counter += 2;

                if i + 1 == string.len() {
                    new_string.push('"');
                    counter += 1;
                }
            },
            '\\' => {
                new_string.push_str("\\\\");
                counter += 2;
            },
            _ => {
                new_string.push(ch);
                counter += 1;
            },
        }
    }

    counter - string.len()
}

fn main() {
        let buf = BufReader::new(File::open("src/input.txt").expect("Unable to open src/input.txt"));
        
        let mut sum = (0, 0);

        for line in buf.lines() {
            match line {
                Ok(parsed) => {
                    sum.0 += byte_count_minus_chars(&parsed);
                    sum.1 += new_byte_count_minus_length(&parsed);
                },
                _ => (),
            }
        }

        println!("{:?}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
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

    #[test]
    fn test_2() {
        let buf = BufReader::new(File::open("src/test.txt").expect("Unable to open src/test.txt"));
        
        let mut sum = 0;
        
        for line in buf.lines() {
            match line {
                Ok(parsed) => sum += new_byte_count_minus_length(&parsed),
                _ => (),
            }
        }

        assert_eq!(sum, 19);
    }
}
