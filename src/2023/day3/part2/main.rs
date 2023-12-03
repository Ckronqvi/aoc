use std::char;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";
    let file = File::open(file_path)?;
    let file_reader = BufReader::new(file);

    let mut engine: Vec<Vec<char>> = Vec::new();

    for li in file_reader.lines() {
        let line = li?;
        let ch_vec: Vec<char> = line.chars().collect();
        engine.push(ch_vec);
    }

    let mut current_number = String::new();
    let mut start_i: i32 = -1;
    let mut max_x: usize;
    let mut min_x: usize;

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (row_index, row) in engine.iter().enumerate() {
        for (col_index, &cha) in row.iter().enumerate() {
            if cha.is_digit(10) {
                if start_i < 0 {
                    start_i = col_index as i32;
                }
                current_number.push(cha);
            } else if !current_number.is_empty() {
                min_x = if start_i > 0 {
                    (start_i as usize) - 1
                } else {
                    0
                };
                max_x = col_index;
                start_i = -1;

                //check symbol from upper-level
                if row_index > 0 {
                    for n in min_x..=max_x {
                        let ch: char = engine.get(row_index - 1).unwrap().get(n).unwrap().clone();
                        if ch == '*' {
                            gears
                                .entry((row_index - 1, n))
                                .or_insert(Vec::new())
                                .push(current_number.parse::<u32>().unwrap());
                            current_number.clear();
                            break;
                        }
                    }
                }

                //check below
                if row_index < engine.len() - 1 && !current_number.is_empty() {
                    for n in min_x..=max_x {
                        let ch: char = engine.get(row_index + 1).unwrap().get(n).unwrap().clone();
                        if ch == '*' {
                            gears
                                .entry((row_index + 1, n))
                                .or_insert(Vec::new())
                                .push(current_number.parse::<u32>().unwrap());
                            current_number.clear();
                            break;
                        }
                    }
                }

                //check left right
                if !current_number.is_empty() {
                    let ch1: char = engine.get(row_index).unwrap().get(max_x).unwrap().clone();
                    let ch2: char = engine.get(row_index).unwrap().get(min_x).unwrap().clone();
                    if ch1 == '*' {
                        gears
                            .entry((row_index, max_x))
                            .or_insert(Vec::new())
                            .push(current_number.parse::<u32>().unwrap());
                        current_number.clear();
                    }

                    if ch2 == '*' {
                        gears
                            .entry((row_index, min_x))
                            .or_insert(Vec::new())
                            .push(current_number.parse::<u32>().unwrap());
                        current_number.clear();
                    }
                }
                current_number.clear();
            }

            //check last item
            if col_index == engine.get(0).unwrap().len() - 1 && cha.is_digit(10) {
                //check left
                let ch = *row.get((start_i - 1) as usize).unwrap();
                if ch == '*' {
                    gears
                        .entry((row_index, (start_i - 1).try_into().unwrap()))
                        .or_insert(Vec::new())
                        .push(current_number.parse::<u32>().unwrap());
                    current_number.clear();
                }
                //check top
                if row_index > 0 && !current_number.is_empty() {
                    for n in (start_i - 1) as usize..=col_index {
                        let ch = *engine.get(row_index - 1).unwrap().get(n).unwrap();
                        if ch == '*' {
                            gears
                                .entry((row_index-1, n.try_into().unwrap()))
                                .or_insert(Vec::new())
                                .push(current_number.parse::<u32>().unwrap());
                            current_number.clear();
                            break;
                        }
                    }
                }

                //check below
                if row_index < engine.len()  && !current_number.is_empty() {
                    for n in (start_i - 1) as usize..=col_index {
                        let ch = *engine.get(row_index + 1).unwrap().get(n).unwrap();
                        if ch == '*' {
                            gears
                                .entry((row_index + 1, n.try_into().unwrap()))
                                .or_insert(Vec::new())
                                .push(current_number.parse::<u32>().unwrap());
                            current_number.clear();
                            break;
                        }
                    }
                }
                current_number.clear();
                start_i = -1;
            }
        }
    }
    for ((row_index, col_index), values) in gears.iter() {
        println!(
            "Entry at ({}, {}): {:?}",
            row_index, col_index, values
        );
    }
    let sum_of_products: u32 = gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().product::<u32>())
        .sum();

    print!("Total: {}", sum_of_products);
    Ok(())
}
