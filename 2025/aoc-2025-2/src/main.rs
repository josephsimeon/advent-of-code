fn main() {
    println!("Hello, world!");
}

fn get_id_range(range: &str) -> (u64, u64) {
        let numbers: Vec<&str> = range.split('-').collect();
        let start = numbers[0].parse::<u64>().expect("Unable to parse digits for start");
        let end = numbers[1].parse::<u64>().expect("Unable to parse digits for end");

        (start, end)
}

fn process_range_for_invalid_ids(range: (u64, u64)) -> Vec<u64> {
    let mut invalid: Vec<u64> = Vec::new();

    for number in range.0..range.1 + 1 {
        let stringify = number.to_string();

        let half = stringify.len() / 2;
        if stringify[..half] == stringify[half..] {
            invalid.push(number);
        }
    }

    invalid
}

fn search_for_invalid_ids(ids: &str) -> u64 {
    let mut invalid: Vec<u64> = Vec::new();

    for line in ids.split(',') {
        let range = get_id_range(line);

        let mut ids = process_range_for_invalid_ids(range);
        invalid.append(&mut ids);
    }

    invalid.iter().sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_PUZZLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test() {
        assert_eq!(search_for_invalid_ids(TEST_PUZZLE), 1227775554);
    }

    #[test]
    fn test_id_range() {
        let test = get_id_range("11-22");
        assert_eq!(test.0, 11);
        assert_eq!(test.1, 22);
    }

    #[test]
    fn test_invalid_ids() {
        let test = process_range_for_invalid_ids((11, 22));
        assert_eq!(test[0], 11);
        assert_eq!(test[1], 22);
    }
}
