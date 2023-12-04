use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";
    let file = File::open(file_path)?;
    let file_reader = BufReader::new(file);

    let mut sum: i32 = 0;

    for li in file_reader.lines() {
        let line: String = li?;
        let all_numbers: Vec<&str> = line
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split("|")
            .collect();
        let mut winning_numbers: Vec<u32> = all_numbers
            .get(0)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>())
            .collect::<Result<_, _>>()
            .unwrap();

        let mut my_numbers: Vec<u32> = all_numbers
            .get(1)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>())
            .collect::<Result<_, _>>()
            .unwrap();

        winning_numbers.sort();
        my_numbers.sort();
        
        sum += count_common_elements(&my_numbers, &winning_numbers);
    }

    print!("Total: {}", sum);
    Ok(())
}

fn count_common_elements(vec1: &[u32], vec2: &[u32]) -> i32 {
    let mut count = 0;
    let (mut i, mut j) = (0, 0);

    while i < vec1.len() && j < vec2.len() {
        if vec1[i] == vec2[j] {
            if count >= 1 {
                count *= 2;
            } else {count = 1}
            i += 1;
            j += 1;
        } else if vec1[i] < vec2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }

    count
}
