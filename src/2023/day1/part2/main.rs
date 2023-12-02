use std::collections::HashMap;
use std::fs::*;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";
    let file = File::open(file_path)?;
    let mut total_sum = 0;
    let file_reader = BufReader::new(file);

    let spelled_out_digits: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    for li in file_reader.lines() {
    
        let line: String = li?;
        let mut first: u32 = 0;
        let mut last: u32 = 0;

        //find first
        for i in 0..line.len() {
            let current_char = line.chars().nth(i).unwrap();
            if current_char.is_numeric() {
                first = current_char.to_digit(10).unwrap();
                break;
            }

            if line.len() < 3 {continue;}
            if i < line.len() - 3 {
                let substr = &line[i..i+3];
                if let Some(number) = spelled_out_digits.get(substr) {
                    first = *number;
                    break;
                }
            }

            if line.len() < 4 {continue;}
            if i < line.len() - 4 {
                let substr = &line[i..i+4];
                if let Some(number) = spelled_out_digits.get(substr) {
                    first = *number;
                    break;
                }
            }

            if line.len() < 5 {continue;}
            if i < line.len() - 5 {
                let substr = &line[i..i+5];
                if let Some(number) = spelled_out_digits.get(substr) {
                    first = *number;
                    break;
                }
            }
        }

        //find last
        let mut lineRev: String = line.chars().rev().collect();
        for i in 0..lineRev.len() {
            let current_char = lineRev.chars().nth(i).unwrap();
        
            if current_char.is_numeric() {
                last = current_char.to_digit(10).unwrap();
                break;
            }
        
            if i + 3 <= line.len() {
                let substr = &lineRev[i..i + 3];
                if let Some(number) = spelled_out_digits.get(&substr.chars().rev().collect::<String>().as_str()) {
                    last = *number;
                    break;
                }
            }
        
            if i + 4 <= line.len() {
                let substr = &lineRev[i..i + 4];
                if let Some(number) = spelled_out_digits.get(&substr.chars().rev().collect::<String>().as_str()) {
                    last = *number;
                    break;
                }
            }
        
            if i + 5 <= line.len() {
                let substr = &lineRev[i..i + 5];
                if let Some(number) = spelled_out_digits.get(&substr.chars().rev().collect::<String>().as_str()) {
                    last = *number;
                    break;
                }
            }
        }
        
        total_sum += first * 10 + last;
    }

    println!("Total Sum: {}", total_sum);
    Ok(())
}