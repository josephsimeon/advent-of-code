fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    static TEST_PUZZLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_1() {
        let mut dial: i8 = 50;
        let mut counter: u8 = 0;

        for line in TEST_PUZZLE.lines() {
            let sign = &line[0..1];
            let digits = &line[1..];
            let mut num = digits.parse::<i8>().expect("Unable to parse {digits}, not a valid number");

            if sign == "L" { num *= -1 };

            dial = (dial - num) % 100;

            if dial == 0 { counter += 1 };
        }

        assert_eq!(counter, 3);
    }
}
