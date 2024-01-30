use std::fs::File;
use std::io::{self, BufRead, Lines, Result as IOResult};
use std::path::Path;

use std::num::ParseIntError;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    if let Ok(lines) = read_lines(INPUT_FILE_PATH) {
        for line in lines.flatten() {
            let game_num = get_game_number(&line);
            println!("{}", game_num.unwrap());
        }
    }
}

fn get_game_number(line: &String) -> Result<i32, ParseIntError> {
    let parts = line.split(":").collect::<Vec<&str>>();

    let num = parts[0].split_whitespace().collect::<Vec<&str>>()[1];

    Ok(num.parse::<i32>()?)
}

fn read_lines<P>(path: P) -> IOResult<Lines<io::BufReader<File>>>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_game_number() {
        let mut line = String::from("Game 1: foo bar baz bing bong");

        assert_eq!(get_game_number(&line).unwrap(), 1);

        line = String::from("Game 666: evil foo bar");
        assert_eq!(get_game_number(&line).unwrap(), 666);
    }
}
