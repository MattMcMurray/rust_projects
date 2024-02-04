use std::io::{self, BufRead, Lines, Result as IOResult};
use std::fs::File;
use std::path::Path;

mod scratchcard;

use scratchcard::Scratchcard;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    let total = process_input(INPUT_FILE_PATH);

    println!("Total: {}", total);
}

fn process_input(filepath: &str) -> i32 {
    let mut total: i32 = 0;

    let mut lines = read_lines(filepath).unwrap();

    while let Some(Ok(line)) = lines.next() {
        let scratchcard = Scratchcard::new(&line).unwrap();

        total += scratchcard.calculate_score();
    }

    return total;
}

fn read_lines<P>(path: P) -> IOResult<Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let r = process_input("test.txt");
        assert_eq!(r, 13);
    }
}