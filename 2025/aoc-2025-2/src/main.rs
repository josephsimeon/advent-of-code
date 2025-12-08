fn main() {
    println!("{}", search_for_invalid_ids(PUZZLE));
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

fn does_substring_repeat(number: u64) -> bool {
    let stringify = number.to_string();
    let len = stringify.len();

    for n in 1..=(len / 2) {
        let pattern = &stringify[..n];

        if len % n == 0 {
            for i in (n..len).step_by(n) {
                if pattern == &stringify[i..(i + n)] { return true };
            }
        }
    }

    false
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

    #[test]
    fn test_id_11() {
        assert!(does_substring_repeat(11));
    }

    #[test]
    fn test_id_121212() {
        assert!(does_substring_repeat(121212));
    }

    #[test]
    fn test_id_12222() {
        assert!(!does_substring_repeat(12222));
    }
}

static PUZZLE: &str = "8284583-8497825,7171599589-7171806875,726-1031,109709-251143,1039-2064,650391-673817,674522-857785,53851-79525,8874170-8908147,4197684-4326484,22095-51217,92761-107689,23127451-23279882,4145708930-4145757240,375283-509798,585093-612147,7921-11457,899998-1044449,3-19,35-64,244-657,5514-7852,9292905274-9292965269,287261640-287314275,70-129,86249864-86269107,5441357-5687039,2493-5147,93835572-94041507,277109-336732,74668271-74836119,616692-643777,521461-548256,3131219357-3131417388";
