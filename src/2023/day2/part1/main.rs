use std::fs::*;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";
    let file = File::open(file_path)?;
    let mut total_sum = 0;
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;
    let mut possible = true;
    let file_reader = BufReader::new(file);
    for li in file_reader.lines() {
        let line = li?;

        let game_info: Vec<&str> = line.trim().split(':').collect();
        let game_number = game_info
            .get(0)?
            .trim()
            .trim_start_matches("Game")
            .trim()
            .parse::<u32>();

        for game_str in game_info.get(1).split(';') {
            let mut red_count = 0;
            let mut blue_count = 0;
            let mut green_count = 0;

            for part in game_str.split(',') {
                let tokens: Vec<&str> = part.trim().split_whitespace().collect();

                if let Some(amount_str) = tokens.get(0) {
                    if let Some(color) = tokens.get(1) {
                        if let Ok(amount) = amount_str.parse::<u32>() {
                            match color.to_lowercase().as_str() {
                                "red" => red_count += amount,
                                "blue" => blue_count += amount,
                                "green" => green_count += amount,
                                _ => (),
                            }
                        }
                    }
                }
            }

            if red_count > MAX_RED || green_count > MAX_GREEN || blue_count > MAX_BLUE {
                possible = false;
            }
        }

        if possible {
            total_sum += game_number;
            possible = true;
        }
    }
}
