use std::fs::File;
use std::io::{self, BufRead, BufReader};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";
    let file = File::open(file_path)?;
    let file_reader = BufReader::new(file);

    let mut total_sum = 0;
    let mut possible = true;

    for li in file_reader.lines() {
        let line = li?;
        let game_info: Vec<&str> = line.trim().split(':').collect();

        if let (Some(game_number_str), Some(game_str)) = (game_info.get(0), game_info.get(1)) {
            if let Ok(game_number) = game_number_str
                .trim_start_matches("Game")
                .trim()
                .parse::<u32>()
            {

                for part in game_str.split(';') {
                    let mut red_count = 0;
                    let mut blue_count = 0;
                    let mut green_count = 0;
                    for token in part.trim().split(',') {
                        let tokens: Vec<&str> = token.trim().split_whitespace().collect();
                        if let (Some(amount_str), Some(color)) = (tokens.get(0), tokens.get(1)) {
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
                    if red_count > MAX_RED || green_count > MAX_GREEN || blue_count > MAX_BLUE {
                        possible = false;
                    }
                }

                if possible {
                    total_sum += game_number;
                }
                possible = true;
            }
        }
    }

    println!("Total Sum: {}", total_sum);

    Ok(())
}