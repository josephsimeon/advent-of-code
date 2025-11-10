fn collect_digits_from_str(slice: &str) -> Vec<u32> {
    let conversion: Vec<u32> = slice
        .split('x')
        .filter_map(|digit| digit.parse::<u32>().ok())
        .collect();

    conversion
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2x3x4_conversion() {
        assert_eq!(collect_digits_from_str("2x3x4"), vec![2, 3, 4]);
    }
}
