use std::fs::File;
use std::io::{self, BufRead, Lines, Result as IOResult};
use std::path::Path;

mod util;

use util::structs::GameConfig;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    const CONFIG: GameConfig = GameConfig {
        red: 12,
        green: 13,
        blue: 14,
    };

    println!("{:?}", CONFIG);

    let mut total = 0;
    let mut total_pwr = 0;
    if let Ok(lines) = read_lines(INPUT_FILE_PATH) {
        for line in lines.flatten() {
            let res = analyze_game(&line, &CONFIG);
            total += res.unwrap_or(0);

            let valid = match res {
                Some(_) => true,
                None => false,
            };

            let (_, game) = split_game(&line).unwrap_or_else(|| { (0, "") });
            let power = get_game_power(game);
            total_pwr += power;


            println!("{} is valid? => {}", line, valid);
            println!("\t => with power: {}", power);
        }
    }

    println!("Total: {}", total);
    println!("Total power: {}", total_pwr);
}

fn analyze_game(line: &str, config: &GameConfig) -> Option<i32> {
    let (game_num, game) = split_game(line)?;

    let rolls = split_rolls(game);

    for roll in rolls {
        let plays = split_plays(roll);

        for play in plays {
            if !roll_valid(play, config) {
                return None;
            }
        }
    };

    Some(game_num)
}

fn get_game_power(game: &str) -> i32 {
    let mut min_conf = GameConfig {
        red: 0,
        green: 0,
        blue: 0,
    };

    let rolls = split_rolls(game);

    for roll in rolls {
        let plays = split_plays(roll);

        for play in plays {
            let (color, value) = split_color_value(play);

            match color {
                "red" => {
                    if value > min_conf.red {
                        min_conf.red = value;
                    }
                },
                "green" => {
                    if value > min_conf.green {
                        min_conf.green = value;
                    }
                },
                "blue" => {
                    if value > min_conf.blue {
                        min_conf.blue = value;
                    }
                },
                color => panic!("unknown color: {}", color),
            };
        }
    }

    min_conf.red * min_conf.green * min_conf.blue
}

fn split_game(line: &str) -> Option<(i32, &str)>{
    let parts = line.split(":").collect::<Vec<&str>>();

    let num = parts[0].split_whitespace().collect::<Vec<&str>>()[1];

    let game_num = match num.parse::<i32>() {
        Ok(num) => num,
        Err(e) => return None,
    };

    let game = parts[1].trim();

    Some((game_num, game))
}

fn split_plays(line: &str) -> Vec<&str> {
    line.split(";").map(|s| s.trim()).collect::<Vec<&str>>()
}

fn split_rolls(play: &str) -> Vec<&str> {
    play.split(",").map(|p| p.trim()).collect::<Vec<&str>>()
}

fn split_color_value(roll: &str) -> (&str, i32) {
    let tokens = roll.split_whitespace().collect::<Vec<&str>>();

    let value = match tokens[0].parse::<i32>() {
        Ok(v) => v,
        Err(e) => panic!("Couldn't parse {} into int: {}", tokens[0], e),
    };

    let color = tokens[1]; 

    (color, value)
}

fn roll_valid(roll: &str, config: &GameConfig) -> bool {
    let (color, value) = split_color_value(roll);

    match color.trim() {
        "red" => value <= config.red,
        "green" => value <= config.green,
        "blue" => value <= config.blue,
        other => panic!("Unknown color: {}", other)
    }
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
    fn test_analyze_game() {
        let config = GameConfig {
            red: 10,
            green: 10,
            blue: 10
        };

        let mut game = "Game 1: 1 blue, 2 red, 3 green; 10 blue, 10 red, 10 green";
        assert_eq!(analyze_game(&game, &config), Some(1));

        game = "Game 2: 1 blue, 11 red, 3 green; 10 blue, 10 red, 10 green";
        assert_eq!(analyze_game(&game, &config), None);
    }

    #[test]
    fn test_get_game_number() {
        let mut line = String::from("Game 1: foo bar baz bing bong");

        assert_eq!(split_game(&line).unwrap(), (1, "foo bar baz bing bong"));

        line = String::from("Game 666: evil foo bar");
        assert_eq!(split_game(&line).unwrap(), (666, "evil foo bar"));
    }

    #[test]
    fn test_split_plays() {
        let line = String::from("12 blue, 4 red; 2 red, 8 green");

        let plays = split_plays(&line);
        assert_eq!(plays[0], "12 blue, 4 red");
        assert_eq!(plays[1], "2 red, 8 green");
    }

    #[test]
    fn test_split_rolls() {
        let line = String::from("12 blue, 4 red, 8 green");

        let rolls = split_rolls(&line);
        assert_eq!(rolls[0], "12 blue");
        assert_eq!(rolls[1], "4 red");
        assert_eq!(rolls[2], "8 green");
    }

    #[test]
    fn test_roll_valid() {
        let mut roll = "100 blue";
        let config = GameConfig {
            blue: 10,
            red: 10,
            green: 10,
        };

        assert!(!roll_valid(&roll, &config));

        roll = "10 blue";
        assert!(roll_valid(&roll, &config));
        roll = "1 blue";
        assert!(roll_valid(&roll, &config));
    }

    #[test]
    fn test_minimum_necessary_config() {
        let mut play = "3 red, 3 blue; 10 blue, 10 red; 10 green, 5 blue";

        let mut res = get_game_power(play);
        assert_eq!(res.red, 10);
        assert_eq!(res.green, 10);
        assert_eq!(res.blue, 10);

        play = "1 red, 2 green, 3 blue; 2 red, 3 green, 4 blue; 3 red, 4 green, 5 blue";
        res = get_game_power(play);
        assert_eq!(res.red, 3);
        assert_eq!(res.green, 4);
        assert_eq!(res.blue, 5);
    }
}
