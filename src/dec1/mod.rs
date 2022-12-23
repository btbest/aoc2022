use std::{path::Path, fs::read_to_string};

pub fn task1() {
    let data = read_to_string(Path::new("inputs").join("dec1_task1.txt")).unwrap();
    let highest_elf_value: i32 = data.split("\r\n\r\n").map(
        |single_elf_lines| single_elf_lines.split("\r\n").map(
            |line| line.parse::<i32>().unwrap_or_default()
        ).sum()
    ).max().unwrap();
    println!("Day 1, task 1. Highest calorie load carried by any elf: {:?}",highest_elf_value)
}