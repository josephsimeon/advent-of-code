fn find_hash(secret_key: &str, pattern: &str) -> u64 {
    let mut counter: u64 = 0;

    'find_hash: loop {
        let hash = md5::compute(format!("{}{}", secret_key, counter).as_bytes());
        let hash_hex = format!("{:x}", hash);

        if hash_hex.starts_with(pattern) {
            break 'find_hash;
        }

        counter += 1;
    }

    counter
}


fn main() {
    println!("{}", find_hash("yzbqklnj", "00000"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_abcdef() {
        assert_eq!(find_hash("abcdef", "00000"), 609043);
    }

    #[test]
    fn test_pqrstuv() {
        assert_eq!(find_hash("pqrstuv", "00000"), 1048970);
    }
}
