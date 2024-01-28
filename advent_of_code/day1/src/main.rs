use std::fs::File;
use std::io::{self, BufRead, Lines, Result};
use std::path::Path;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    if let Ok(lines) = read_input_file_lines(INPUT_FILE_PATH) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

fn read_input_file_lines<P>(path: P) -> Result<Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // the `?` syntax at the end of an operation returning a `Result`
    // is equivalent to a `match` expression where the `Err(err)`
    // branch expands to an early `return Err(...)`
    // and the `Ok` branch expands to an `ok` expression
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
