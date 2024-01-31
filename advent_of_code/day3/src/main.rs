use std::io::{self, BufRead, Lines, Result as IOResult};
use std::fs::File;
use std::path::Path;

const INPUT_FILE_PATH: &str = "input.txt";

mod engine;

use engine::EngineSchematic;

fn main() {
    let mut schematic = EngineSchematic {
        width: 0,
        height: 0,
        schematic: vec![],
    };
    
    if let Ok(mut lines) = read_lines(INPUT_FILE_PATH) {
        while let Some(Ok(line)) = lines.next() {
            let mut chars = line_to_char(&line);

            if schematic.width == 0 {
                schematic.width = chars.len() as i32;
            }

            schematic.height += 1;
            schematic.schematic.append(&mut chars);
        }
    }
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
}