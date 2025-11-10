pub fn calculate_wrapping_paper(length: u32, width: u32, height: u32) -> u32 {
    let mut area: Vec<u32> = Vec::new();

    area.push(length * width);
    area.push(width * height);
    area.push(height * length);

    let smallest = find_smallest_area(&area);

    area.iter().map(|area| area * 2).sum::<u32>() + smallest
}

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
    fn test_2x3x4() {
        assert_eq!(calculate_wrapping_paper(2, 3, 4), 58);
    }
 
    #[test]
    fn test_smallest_2x3x4() {
        let test: Vec<u32> = vec![6, 8, 12];
        assert_eq!(find_smallest_area(&test), 6);
    }
}
