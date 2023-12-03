use std::char;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";
    let file = File::open(file_path)?;
    let file_reader = BufReader::new(file);

    let mut total_sum = 0;
    let mut engine: Vec<Vec<char>> = Vec::new();

    for li in file_reader.lines() {
        let line = li?;
        let ch_vec: Vec<char> = line.chars().collect();
        engine.push(ch_vec);
    }


    let mut current_number = String::new();
    let mut number: u32;
    let mut start_i: i32 = -1;
    let mut max_x: usize;
    let mut min_x: usize;

    let mut is_engine_part: bool = false;

    for (row_index, row) in engine.iter().enumerate() {
        for (col_index, &cha) in row.iter().enumerate() {
            if cha.is_digit(10) {
                if start_i < 0 {
                    start_i = col_index as i32;
                }
                current_number.push(cha);
            } else if !current_number.is_empty() {
                min_x = if start_i > 0 {(start_i as usize) -1} else {0};
                max_x = col_index;
                start_i = -1;

                //check symbol from upper-level
                if row_index > 0 {
                    for n in min_x..=max_x {
                        let ch: char = engine.get(row_index-1).unwrap().get(n).unwrap().clone();
                        if !ch.is_digit(10) && ch != '.' {
                            is_engine_part = true;
                            break;
                        }
                    }      
                }

                //check below 
                if row_index < engine.len()-1 && !is_engine_part {
                    for n in min_x..=max_x {
                        let ch: char = engine.get(row_index+1).unwrap().get(n).unwrap().clone();
                        if !ch.is_digit(10) && ch != '.' {
                            is_engine_part = true;
                            break;
                        }   
                    }
                }

                //check left right
                if !is_engine_part {
                    let ch1: char = engine.get(row_index).unwrap().get(max_x).unwrap().clone();
                    let ch2: char = engine.get(row_index).unwrap().get(min_x).unwrap().clone();
                    if (!ch1.is_digit(10) && ch1 != '.') || (!ch2.is_digit(10) && ch2 != '.') {
                        is_engine_part = true;
                    }
                }

                if is_engine_part {
                    number = current_number.parse::<u32>().unwrap();
                    total_sum += number;
                }
                current_number.clear();
                is_engine_part = false;
            }

            //check last item
            if col_index ==  engine.get(0).unwrap().len()-1 && cha.is_digit(10){
                //check left
                let ch = *row.get((start_i-1) as usize).unwrap();
                if !ch.is_digit(10) && ch != '.' {
                    is_engine_part = true;
                }

                //check top
                if row_index > 0 {
                    for n in (start_i-1) as usize..=col_index {
                        let ch = *engine.get(row_index-1).unwrap().get(n).unwrap();
                        if !ch.is_digit(10) && ch != '.' {
                            is_engine_part = true;
                        }
                    }
                }

                //check beloq
                if row_index < engine.len() {
                    for n in (start_i-1) as usize..=col_index {
                        let ch = *engine.get(row_index+1).unwrap().get(n).unwrap();
                        if !ch.is_digit(10) && ch != '.' {
                            is_engine_part = true;
                        }
                    }
                }

                if is_engine_part {
                    number = current_number.parse::<u32>().unwrap();
                    total_sum += number;
                }
                current_number.clear();
                is_engine_part = false;
                start_i = -1;   
            }
        }
    }
    print!("Total: {}", total_sum);
    Ok(())
}
