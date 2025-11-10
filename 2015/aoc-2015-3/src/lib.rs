pub trait Location {
    fn move_direction(&mut self, direction: char) -> &Self;
}

impl Location for (i32, i32) {
    fn move_direction(&mut self, direction: char) -> &Self {
        match direction {
            '^' => self.1 += 1,
            '>' => self.0 += 1,
            'v' => self.1 -= 1,
            '<' => self.0 -= 1,
            _ => (),
        }

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_north_by_1() {
        let mut test = (0, 0);
        test.move_direction('^');
        assert_eq!(test, (0, 1));
    }

    #[test]
    fn test_move_east_by_1() {
        let mut test = (0, 0);
        test.move_direction('>');
        assert_eq!(test, (1, 0));
    }

    #[test]
    fn test_move_south_by_1() {
        let mut test = (0, 0);
        test.move_direction('v');
        assert_eq!(test, (0, -1));
    }

    #[test]
    fn test_move_west_by_1() {
        let mut test = (0, 0);
        test.move_direction('<');
        assert_eq!(test, (-1, 0));
    }
}
