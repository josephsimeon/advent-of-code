fn find_smallest_area(areas: Vec<u32>) -> u32 {
fn find_smallest_area(areas: &Vec<u32>) -> u32 {
    let mut smallest = areas[0];

    for area in areas.iter().skip(1) {
        if smallest > *area { smallest = *area };
    }

    smallest
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_smallest_2x3x4() {
        let test: Vec<u32> = vec![6, 8, 12];
        assert_eq!(find_smallest_area(&test), 6);
    }
}
