use itertools::Either;
use std::fs::File;
use std::io::{self, BufRead, Lines, Result};
use std::path::Path;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines(INPUT_FILE_PATH) {
        for line in lines.flatten() {
            let first = first_digit(&line, false);
            let last = first_digit(&line, true);
            let combined = format!("{}{}", first, last);
            sum += match combined.parse::<i32>() {
                Ok(val) => val,
                Err(e) => panic!("Trouble parsing int: {}", e),
            };
        }
    }

    println!("Final value: {}", sum);
}

fn read_lines<P>(path: P) -> Result<Lines<io::BufReader<File>>>
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

fn first_digit(s: &String, reverse: bool) -> u32 {
    let mut digit: u32 = 0;

    let eval = if reverse {
        Either::Left(s.chars().rev())
    } else {
        Either::Right(s.chars())
    };

    for c in eval {
        if c.is_numeric() {
            digit = match c.to_digit(10) {
                Some(digit) => digit,
                None => panic!(),
            };
            break;
        }
    }

    digit
}
