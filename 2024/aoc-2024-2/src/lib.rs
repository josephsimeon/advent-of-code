#[derive(Debug)]
pub struct Levels(i32, i32);

impl Levels {
    fn check_safe_levels(levels: &Vec<i32>) -> bool {
        if levels.len() <= 1 { return true };

        // check if all increasing
        let mut ascending: bool = true;
        'checking_ascending: for i in 0..(levels.len() - 1) {
            let curr = levels[i];
            let next = levels[i + 1];
            
            if curr >= next {
                ascending = false;
                break 'checking_ascending;
            }
        }

        // check if all decreasing
        let mut descending: bool = true;
        'checking_descending: for i in 0..(levels.len() - 1) {
            let curr = levels[i];
            let next = levels[i + 1];
            
            if curr <= next {
                descending = false;
                break 'checking_descending;
            }
        }

        // check if within threshold
        let mut within_threshold: bool = true;
        'checking_threshold: for i in 0..(levels.len() - 1) {
            let curr = levels[i];
            let next = levels[i + 1];
            let abs = (curr - next).abs();
            
            if !(1..=3).contains(&abs) {
                within_threshold = false;
                break 'checking_threshold
            }
        }

        (ascending || descending) && within_threshold
    }

    fn check_safe_levels_with_tolerance(levels: &Vec<i32>) -> bool {
        for i in 0..levels.len() {
            let mut toleranced_levels = levels.clone();
            toleranced_levels.remove(i);
            if Self::check_safe_levels(&toleranced_levels) { return true };
        }

        false
    }

    pub fn show_levels(readings: &str) -> Levels {
        let mut first_pass: i32 = 0;
        let mut second_pass: i32 = 0;

        for line in readings.lines() {
            let nums: Vec<i32> = line
                .split_whitespace()                     // Iterator through chars of &str by splitting whitespace
                .map(|x| x.parse::<i32>().unwrap())     // Parse each item to <i32> by unwrapping Result<i32, _>
                .collect();                             // Collect into vector

            if Self::check_safe_levels(&nums) { first_pass += 1 }
                else { if Self::check_safe_levels_with_tolerance(&nums) { second_pass += 1 }};
        }

        Levels(first_pass, first_pass + second_pass)
    }
}
