use std::{
    fs::File,
    io::{BufReader, prelude::*},
};

fn main() {
    let input = BufReader::new(File::open("input").expect("no such file"))
        .lines()
        .map(|l| l.expect("could not read line"))
        .collect();

    println!("{}", trebuchet_part1(&input));
    println!("{}", trebuchet_part2(&input));
}

pub fn trebuchet_part1(input: &Vec<String>) -> i32 {
    return 0;
}

pub fn trebuchet_part2(input: &Vec<String>) -> i32 {
    return 0;
}
