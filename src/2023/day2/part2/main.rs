use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";
    let file = File::open(file_path)?;
    let file_reader = BufReader::new(file);

    let mut total_sum = 0;

    for li in file_reader.lines() {
        let line = li?;
        let game_info: Vec<&str> = line.trim().split(':').collect();

        if let Some(game_str) = game_info.get(1) {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for part in game_str.split(';') {
                for token in part.trim().split(',') {
                    let tokens: Vec<&str> = token.trim().split_whitespace().collect();
                    if let (Some(amount_str), Some(color)) = (tokens.get(0), tokens.get(1)) {
                        if let Ok(amount) = amount_str.parse::<u32>() {
                            match color.to_lowercase().as_str() {
                                "red" => {
                                    if amount > min_red {
                                        min_red = amount
                                    }
                                }
                                "blue" => {
                                    if amount > min_blue {
                                        min_blue = amount
                                    }
                                }
                                "green" => {
                                    if amount > min_green {
                                        min_green = amount
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
            total_sum += min_blue * min_green * min_red;
        }
    }
    println!("Total Sum: {}", total_sum);

    Ok(())
}