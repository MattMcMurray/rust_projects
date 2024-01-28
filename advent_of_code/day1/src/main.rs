use std::fs::File;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    open_input(&INPUT_FILE_PATH);
}

fn open_input(path: &str) -> File {
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };

    file
}
