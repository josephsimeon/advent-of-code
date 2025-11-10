use std::collections::HashMap;
use aoc_2015_3::Location;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_east_by_1() {
        let mut houses: HashMap<(i32, i32), u32> = HashMap::new();

        let mut house = (0, 0);
        houses.entry(house).and_modify(|presents| *presents += 1).or_insert(1);

        house.move_direction('>');
        houses.entry(house).and_modify(|presents| *presents += 1).or_insert(1);

        let test = houses.get(&(1, 0)).unwrap();
        assert_eq!(*test, 1);
        assert_eq!(houses.len(), 2);
    }

    #[test]
    fn test_turn_left() {
        let mut houses: HashMap<(i32, i32), u32> = HashMap::new();

        let mut house = (0, 0); 
        houses.entry(house).and_modify(|presents| *presents += 1).or_insert(1);

        for ch in "^>v<".chars() {
            house.move_direction(ch);
            houses.entry(house).and_modify(|presents| *presents += 1).or_insert(1);
        }

        let test = houses.get(&(0, 0)).unwrap();
        assert_eq!(*test, 2);
        assert_eq!(houses.len(), 4);
    }
}
