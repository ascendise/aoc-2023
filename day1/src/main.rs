use std::{fs::File, io::{BufReader, BufRead}};

mod trebuchet;

fn main() {
    let calibration_file = File::open("./calibration_values.txt")
        .expect("Cannot open file 'calibration_values.txt'");
    let sum: u32 = BufReader::new(calibration_file)
        .lines()
        .map(|l| trebuchet::recover(l.unwrap().as_str()))
        .sum();
    println!("Calibration Sum is: {sum}");
}
