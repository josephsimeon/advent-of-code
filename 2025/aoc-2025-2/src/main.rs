fn main() {
    println!("Hello, world!");
}

fn get_id_range(range: &str) -> (u64, u64) {
        let numbers: Vec<&str> = line.split('-').collect();
        let start = numbers[0].parse::<u64>().expect("Unable to parse digits for start");
        let end = numbers[1].parse::<u64>().expect("Unable to parse digits for end");

        (start, end)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_PUZZLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test() {
        let mut invalid: Vec<u64> = Vec::new();

        for line in TEST_PUZZLE.split(',') {
            let range = get_id_range(line);

            for number in range.0..range.1 + 1 {
                let stringify = number.to_string();

                let half = stringify.len() / 2;
                if stringify[..half] == stringify[half..] {
                    invalid.push(number);
                }
            }
        }

        assert_eq!(invalid.iter().sum::<u64>(), 1227775554);
    }

    fn test_id_range() {
        let test = get_id_range("11-22");
        assert_eq!(test.0, 11);
        assert_eq!(test.1, 22);
    }
}
