#[derive(Debug)]
pub struct EngineSchematic {
    pub width: i32,
    pub height: i32,
    pub schematic: Vec<char>,
}

impl EngineSchematic {
    pub fn get_at(&self, x: i32, y: i32) -> char {
        let offset = self.width * y;
        let index = offset + x;

        return self.schematic[index as usize];
    }

    pub fn is_part_number(&self, x: i32, y: i32) -> bool {
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

    /**
     * Gets all digits that are "neighbours" of the digit at (x, y)
     */
    pub fn get_contiguous_digits(&self, x: i32, y: i32) -> i32 {
      let offset = self.width * y;
      let mut right = 0;
      let mut left = 0;

      while (offset + x) + left > offset {
        if self.get_at(x + left, y).is_digit(10) {
          left -= 1;
        } else {
          break;
        }
      };
      
      while (x + right) < (self.width) {
        if self.get_at(x + right, y).is_digit(10) {
          right += 1;
        } else {
          break;
        }
      };

      let from = (offset + x + left ) as usize;
      let to = (offset + x + right) as usize;

      let slice = &self.schematic[from..to].iter().collect::<String>();

      match slice.parse::<i32>() {
        Ok(val) => val,
        Err(e) => panic!("Could not parse '{}' into i32", e),
      }
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

      assert_eq!(schematic.width * schematic.height, schematic.schematic.len() as i32);

      assert!(!schematic.is_part_number(1, 1));

      schematic.schematic = vec!['*', '.', '.', '.', '5', '.', '.', '.', '.'];
      assert!(schematic.is_part_number(1, 1));

      schematic.schematic = vec!['.', '.', '.', '*', '5', '.', '.', '.', '.'];
      assert!(schematic.is_part_number(1, 1));

      schematic.schematic = vec!['.', '.', '.', '.', '5', '.', '.', '*', '.'];
      assert!(schematic.is_part_number(1, 1));

      schematic.schematic = vec!['.', '5', '.', '.', '.', '.', '.', '*', '.'];
      assert!(!schematic.is_part_number(1, 0));
    }

    #[test]
    fn test_get_contiguous_digits() {
      let mut e = EngineSchematic {
        width: 3,
        height: 3, 
        schematic: vec!['.', '.', '.', '4', '2', '.', '.', '.', '.']
      };

      assert_eq!(e.width * e.height, e.schematic.len() as i32);

      assert_eq!(e.get_contiguous_digits(1, 1), 42);

      e.schematic = vec!['.', '.', '6', '4', '2', '.', '.', '.', '.'];
      assert_eq!(e.get_contiguous_digits(1, 1), 42);

      e.schematic = vec!['.', '.', '.', '.', '.', '.', '1', '2', '3'];
      assert_eq!(e.get_contiguous_digits(2, 2), 123);
    }
}
