pub fn calculate_wrapping_paper(length: u32, width: u32, height: u32) -> u32 {
    let area = get_areas(length, width, height);

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

fn get_areas(length: u32, width: u32, height: u32) -> Vec<u32> {
    let mut area: Vec<u32> = Vec::new();

    area.push(length * width);
    area.push(width * height);
    area.push(height * length);

    area
}

fn find_smallest_sides(length: u32, width: u32, height: u32) -> (u32, u32) {
    let mut smallest = length;
    let mut small = width;

    if smallest > height {
        smallest = height;
    } else if small > height {
        small = height;
    }

    (smallest, small)
}

pub fn calculate_ribbon_paper(length: u32, width: u32, height: u32) -> u32 {
    let sides = find_smallest_sides(length, width, height);

    (length * width * height) + (sides.0 + sides.0 + sides.1 + sides.1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2x3x4() {
        assert_eq!(calculate_wrapping_paper(2, 3, 4), 58);
    }

    #[test]
    fn test_1x1x10() {
        assert_eq!(calculate_wrapping_paper(1, 1, 10), 43);
    }
 
    #[test]
    fn test_smallest_2x3x4() {
        let test: Vec<u32> = vec![6, 8, 12];
        assert_eq!(find_smallest_area(&test), 6);
    }

    #[test]
    fn test_2x3x4_areas() {
        assert_eq!(get_areas(2, 3, 4), vec![6, 12, 8]);
    }

    #[test]
    fn test_2x3x4_smallest_sides() {
        assert_eq!(find_smallest_sides(2, 3, 4), (2, 3));
    }

    #[test]
    fn test_1x1x10_smallest_sides() {
        assert_eq!(find_smallest_sides(1, 1, 10), (1, 1));
    }

    #[test]
    fn test_1x7x5_smallest_sides() {
        assert_eq!(find_smallest_sides(1, 7, 5), (1, 5));
    }

    #[test]
    fn test_2x3x4_ribbon() {
        assert_eq!(calculate_ribbon_paper(2, 3, 4), 34);
    }

    #[test]
    fn test_1x1x10_ribbon() {
        assert_eq!(calculate_ribbon_paper(1, 1, 10), 14);
    }
}
