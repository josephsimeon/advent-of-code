fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    static TEST_PUZZLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test() {
        let mut largest_joltage: Vec<u32> = Vec::new();
        let test: u32;

        for line in TEST_PUZZLE.lines() {
            let mut largest_number = 0;

            let mut first = 0;
            for i in 0..(line.len() - 1) {
                let parsed = line
                    .chars()
                    .nth(i)
                    .expect("could not get char")
                    .to_string()
                    .parse::<u8>()
                    .expect("Could not parse digit");

                if first < parsed {
                    first = parsed;
                    let mut second = 0;
                    for j in (i + 1)..line.len() {
                    let parsed = line
                        .chars()
                        .nth(j)
                        .expect("could not get char")
                        .to_string()
                        .parse::<u8>()
                        .expect("Could not parse digit");

                        if second < parsed {
                            second = parsed;

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

        test = largest_joltage.iter().sum::<u32>();
        assert_eq!(test, 357);
    }
}
