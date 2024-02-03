#[derive(Debug)]
pub struct EngineSchematic {
    pub width: i32,
    pub height: i32,
    pub schematic: Vec<char>,
}

impl EngineSchematic {
    ///Givenvscode-terminal:/094bc4ade822814b23ec785064147005/2 an x, y, return the index of the row-ordered array
    pub fn get_offset(&self, x: i32, y: i32) -> usize {
        let offset = self.width * y;
        return (offset + x) as usize;
    }

    pub fn get_at(&self, x: i32, y: i32) -> char {
        let index = self.get_offset(x, y);

        return self.schematic[index as usize];
    }

    pub fn is_part_number(&self, x: i32, y: i32) -> bool {
        if !self.get_at(x, y).is_digit(10) {
            return false;
        }

        const OFFSET: [i32; 3] = [-1, 0, 1];

        for y_offset in OFFSET {
            for x_offset in OFFSET {
                let x = x + x_offset;
                let y = y + y_offset;

                if x >= 0 && y >= 0 && x < self.width && y < self.height {
                    let c = self.get_at(x, y);

                    if c != '.' && !c.is_numeric() {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    pub fn get_gear_ratio(&self, x: i32, y: i32) -> Option<i32> {
        if self.get_at(x, y) != '*' {
            return None;
        };

        const OFFSET: [i32; 3] = [-1, 0, 1];

        let mut digit_locations: Vec<(i32, i32)> = vec![];

        for y_offset in OFFSET {
            for x_offset in OFFSET {
                let x = x + x_offset;
                let y = y + y_offset;

                if x < 0 || y < 0 || x >= self.width || y > self.height {
                    // pass if coords are out-of-bounds
                    continue;
                };

                let c = self.get_at(x, y);
                if c.is_digit(10) {
                    digit_locations.push((x, y));
                };
            }
        }

        let mut ratios: Vec<i32> = vec![];
        let mut visited_locations: Vec<usize> = vec![];
        for (x, y) in digit_locations {
            let idx = self.get_offset(x, y);

            if visited_locations.contains(&idx) {
                continue;
            };

            let (val, start, end) = self.get_contiguous_digits(x, y);
            for i in start..end {
                visited_locations.push(i);
            }

            ratios.push(val);
        };

        if ratios.len() <= 1 {
            return None;
        }

        return ratios.into_iter().reduce(|a, b| a * b);
    }

    /// Gets all digits that are "neighbours" of the digit at (x, y)
    ///
    /// # Arguments
    /// `x` - The x coordinate
    ///
    /// `y` - The y coordinate
    ///
    /// # Returns
    /// ## Tuple
    /// i32 - the contiguous digits, as a number
    ///
    /// usize - the start index
    ///
    /// usize - the end index
    pub fn get_contiguous_digits(&self, x: i32, y: i32) -> (i32, usize, usize) {
        let distance_from_left = x;
        let distance_from_right = self.width - x;

        let mut left = 0;
        let mut right = 1;

        while left < distance_from_left {
            if self.get_at(x - left - 1, y).is_digit(10) {
                left += 1;
            } else {
                break;
            }
        }

        while right < distance_from_right {
            if self.get_at(x + right, y).is_digit(10) {
                right += 1;
            } else {
                break;
            }
        }

        let from = self.get_offset(x - left, y);
        let to = self.get_offset(x + right, y);

        let slice = self.schematic[from..to].iter().collect::<String>();

        let val = match slice.parse::<i32>() {
            Ok(val) => val,
            Err(_) => panic!("Could not parse {} into i32", slice),
        };

        return (val, from, to);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_at() {
        let schematic = EngineSchematic {
            width: 3,
            height: 3,
            schematic: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
        };

        assert_eq!(schematic.get_at(0, 0), 'a');
        assert_eq!(schematic.get_at(1, 0), 'b');
        assert_eq!(schematic.get_at(2, 0), 'c');
        assert_eq!(schematic.get_at(0, 1), 'd');
        assert_eq!(schematic.get_at(1, 1), 'e');
        assert_eq!(schematic.get_at(2, 1), 'f');
        assert_eq!(schematic.get_at(0, 2), 'g');
        assert_eq!(schematic.get_at(1, 2), 'h');
        assert_eq!(schematic.get_at(2, 2), 'i');
    }

    #[test]
    fn test_is_part_number() {
        let mut schematic = EngineSchematic {
            width: 3,
            height: 3,
            schematic: vec!['.', '.', '.', '.', '5', '.', '.', '.', '.'],
        };

        assert_eq!(
            schematic.width * schematic.height,
            schematic.schematic.len() as i32
        );

        assert!(!schematic.is_part_number(1, 1));

        schematic.schematic = vec!['*', '.', '.', '.', '5', '.', '.', '.', '.'];
        assert!(schematic.is_part_number(1, 1));

        schematic.schematic = vec!['.', '.', '.', '*', '5', '.', '.', '.', '.'];
        assert!(schematic.is_part_number(1, 1));

        schematic.schematic = vec!['.', '.', '.', '.', '5', '.', '.', '*', '.'];
        assert!(schematic.is_part_number(1, 1));

        schematic.schematic = vec!['.', '5', '.', '.', '.', '.', '.', '*', '.'];
        assert!(!schematic.is_part_number(1, 0));
        assert!(!schematic.is_part_number(1, 1));
    }

    #[test]
    fn test_get_contiguous_digits() {
        let mut e = EngineSchematic {
            width: 3,
            height: 3,
            schematic: vec!['.', '.', '.', '4', '2', '.', '.', '.', '.'],
        };

        assert_eq!(e.width * e.height, e.schematic.len() as i32);

        assert_eq!(e.get_contiguous_digits(1, 1), (42, 3, 5));

        e.schematic = vec!['.', '.', '6', '4', '2', '.', '.', '.', '.'];
        assert_eq!(e.get_contiguous_digits(1, 1), (42, 3, 5));

        e.schematic = vec!['.', '.', '.', '.', '.', '.', '1', '2', '3'];
        assert_eq!(e.get_contiguous_digits(2, 2), (123, 6, 9));
    }

    #[test]
    fn test_get_gear_ratio() {
        let input = vec![
            '4', '6', '7', '.', '.', '1', '1', '4', '.', '.', '.', '.', '.', '*', '.', '.', '.',
            '.', '.', '.', '.', '.', '3', '5', '.', '.', '6', '3', '3', '.', '.', '.', '.', '.',
            '.', '.', '#', '.', '.', '.', '6', '1', '7', '*', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '+', '.', '5', '8', '.', '.', '.', '5', '9', '2', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '7', '5', '5', '.', '.', '.', '.', '$', '.',
            '*', '.', '.', '.', '.', '.', '6', '6', '4', '.', '5', '9', '8', '.', '.',
        ];

        let schematic = EngineSchematic {
            height: 10,
            width: 10,
            schematic: input,
        };

        assert_eq!(schematic.get_gear_ratio(3, 0), None);
        assert_eq!(schematic.get_gear_ratio(3, 1), Some(16345));
        assert_eq!(schematic.get_gear_ratio(3, 4), None);
        assert_eq!(schematic.get_gear_ratio(5, 8), Some(755 * 598));
    }
}
