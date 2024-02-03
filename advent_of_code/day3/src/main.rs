use std::io::{self, BufRead, Lines, Result as IOResult};
use std::fs::File;
use std::path::Path;

const INPUT_FILE_PATH: &str = "input.txt";

mod engine;

use engine::EngineSchematic;

fn main() {
    let schematic = file_to_schematic(INPUT_FILE_PATH);
    let total = sum_part_numbers(&schematic);
    let gear_ratio = sum_gear_ratios(&schematic);

    println!("Part number total: {}", total);
    println!("Gear ratio total: {}", gear_ratio);
}

fn sum_part_numbers(schematic: &EngineSchematic) -> i32 {
    let mut visited_indices: Vec<usize> = vec![];

    let mut total = 0;
    for y in 0..schematic.height {
        for x in 0..schematic.width {
            if schematic.is_part_number(x, y) && !visited_indices.contains(&schematic.get_offset(x, y)) {
                let (val, start, end) = schematic.get_contiguous_digits(x, y);
                for i in start..end {
                    visited_indices.push(i);
                };

                total += val;
            }
        }
    }

    return total;
}

fn sum_gear_ratios(schematic: &EngineSchematic) -> i32 {
    let mut gear_coords: Vec<(i32, i32)> = vec![];

    for y in 0..schematic.height {
        for x in 0..schematic.width {
            if schematic.get_at(x, y) == '*' {
                gear_coords.push((x, y));
            }
        }
    };

    let mut ratio = 0;
    for (x, y) in gear_coords {
        ratio += match schematic.get_gear_ratio(x, y) {
            Some(val) => val,
            None => 0,
        }
    };

    return ratio;
}

fn file_to_schematic(filepath: &str) -> EngineSchematic {
    let mut schematic = EngineSchematic {
        width: 0,
        height: 0,
        schematic: vec![],
    };

    if let Ok(mut lines) = read_lines(filepath) {
        while let Some(Ok(line)) = lines.next() {
            let chars = line_to_char(&line);

            if schematic.width == 0 {
                schematic.width = chars.len() as i32;
            }

            schematic.height += 1;
            schematic.schematic.append(&mut chars.clone());
        }
    };

    return schematic;
}

fn read_lines<P>(path: P) -> IOResult<Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn line_to_char(line: &str) -> Vec<char> {
    line.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_char() {
        let line = "..*..5";

        assert_eq!(line_to_char(line), vec!['.', '.', '*', '.', '.', '5'])
    }

    #[test]
    fn test_sum_part_numbers() {
        let s = file_to_schematic("test.txt");
        let total = sum_part_numbers(&s);

        assert_eq!(total, 4361);
    }

    #[test]
    fn test_sum_gear_ratios() {
        let s = file_to_schematic("test.txt");
        let total = sum_gear_ratios(&s);

        assert_eq!(total, 467835);
    }
}