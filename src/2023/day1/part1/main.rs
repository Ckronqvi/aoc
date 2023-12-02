use std::fs::*;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";
    let file = File::open(file_path)?;
    let mut total_sum = 0;
    let file_reader = BufReader::new(file);
    for li in file_reader.lines() {
        let line = li?;
        let first_digit = line
            .chars()
            .find(|c| c.is_digit(10))
            .and_then(|c| c.to_digit(10))
            .unwrap();
        let last_digit = line
            .chars()
            .rev()
            .find(|c| c.is_digit(10))
            .and_then(|c| c.to_digit(10))
            .unwrap();

        total_sum += first_digit * 10 + last_digit;
    }

    println!("Total Sum: {}", total_sum);
    Ok(())
}