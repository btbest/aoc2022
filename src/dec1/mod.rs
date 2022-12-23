use std::{path::{Path, PathBuf}, fs::File, io::{BufReader, BufRead}};

pub fn task1() {
    let data = load_from_file(Path::new("inputs").join("dec1_task1.txt"));
    println!("{:?}", data);
}

fn load_from_file(file_path: PathBuf) {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap_or_else())
        .collect();
}