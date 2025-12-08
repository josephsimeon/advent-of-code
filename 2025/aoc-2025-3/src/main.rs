fn main() {
    println!("Hello, world!");
}

fn find_total_joltage(input: &str) -> u32 {
    let mut largest_joltage: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut largest_number = 0;

        let mut first = 0;
        for i in 0..(line.len() - 1) {
            let digit = get_digit_from_slice(line, i);

            if first < digit {
                first = digit;

                let mut second = 0;
                for j in (i + 1)..line.len() {
                    let digit = get_digit_from_slice(line, j);

                    if second < digit {
                        second = digit;

                        let number = first * 10 + second;
                        if largest_number < number {
                            largest_number = number;
                        }
                    }
                }
            }
        }

        largest_joltage.push(largest_number as u32);
    }

    largest_joltage.iter().sum::<u32>()
}

fn get_digit_from_slice(slice: &str, position: usize) -> u8 {
    slice
        .chars()
        .nth(position)
        .expect("Could not get char")
        .to_string()
        .parse::<u8>()
        .expect("Could not parse digit")
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_PUZZLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test() {
        assert_eq!(find_total_joltage(TEST_PUZZLE), 357);
    }
}
