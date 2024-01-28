use itertools::Either;
use std::fs::File;
use std::io::{self, BufRead, Lines, Result};
use std::path::Path;

mod util;

use util::constants::VALID_DIGITS;
use util::structs::SubstrLocation;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    let mut calibration_sum: i32 = 0;
    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines(INPUT_FILE_PATH) {
        for line in lines.flatten() {
            let f = first_digit(&line, false);
            let l = first_digit(&line, true);
            let c = format!("{}{}", f, l);
            calibration_sum += match c.parse::<i32>() {
                Ok(val) => val,
                Err(e) => panic!("Trouble parsing int: {}", e),
            };

            let first = first_digit_or_word(&line);
            let last = last_digit_or_word(&line);
            let combined = format!("{}{}", first, last);

            sum += match combined.parse::<i32>() {
                Ok(val) => val,
                Err(e) => panic!("Trouble parsing int: {}", e),
            };
        }
    }

    println!("Calibration sum: {}", calibration_sum);
    println!("Total sum: {}", sum);
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

fn build_map(s: &String) -> Vec<SubstrLocation> {
    let mut results: Vec<SubstrLocation> = vec![];

    for item in VALID_DIGITS {
        let indices: Vec<_> = s.match_indices(&item.name).collect();

        for res in indices {
            let (idx, _) = res;
            let location = SubstrLocation {
                value: item.value,
                location: idx,
            };

            results.push(location);
        }
    }

    results
}

fn first_digit_or_word(s: &String) -> u32 {
    let mut results = build_map(s);

    results.sort_by(|a, b| a.location.cmp(&b.location));

    match results.first() {
        Some(v) => v.value as u32,
        None => panic!("No digit found in string: {}", s),
    }
}

fn last_digit_or_word(s: &String) -> u32 {
    let mut results = build_map(s);

    results.sort_by(|a, b| b.location.cmp(&a.location));

    match results.first() {
        Some(v) => v.value as u32,
        None => panic!("No digit found in string: {}", s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_digit_or_word() {
        let mut s = String::from("lkjdlfjone2three");

        assert_eq!(first_digit_or_word(&s), 1);

        s = String::from("two");
        assert_eq!(first_digit_or_word(&s), 2);

        s = String::from("akjsdklfjsdfjdtw1o");
        assert_eq!(first_digit_or_word(&s), 1);
    }

    #[test]
    fn test_last_digit_or_word() {
        let mut s = String::from("lkjdlfjone2three");

        assert_eq!(last_digit_or_word(&s), 3);

        s = String::from("lkjdlfjone2three1");
        assert_eq!(last_digit_or_word(&s), 1);

        s = String::from("lkjdlfjone2three1sdjfldkf");
        assert_eq!(last_digit_or_word(&s), 1);
        s = String::from("lkjdlfjone2three2sdjfldkf");
        assert_eq!(last_digit_or_word(&s), 2);
        s = String::from("lkjdlfjone2three3sdjfldkf");
        assert_eq!(last_digit_or_word(&s), 3);
        s = String::from("lkjdlfjone2three4sdjfldkf");
        assert_eq!(last_digit_or_word(&s), 4);
        s = String::from("lkjdlfjone2three5sdjfldkf");
        assert_eq!(last_digit_or_word(&s), 5);
        s = String::from("lkjdlfjone2three6sdjfldkf");
        assert_eq!(last_digit_or_word(&s), 6);
        s = String::from("lkjdlfjone2three7sdjfldkf");
        assert_eq!(last_digit_or_word(&s), 7);
        s = String::from("lkjdlfjone2three8sdjfldkf");
        assert_eq!(last_digit_or_word(&s), 8);
        s = String::from("lkjdlfjone2three9sdjfldkf");
        assert_eq!(last_digit_or_word(&s), 9);
    }
}
