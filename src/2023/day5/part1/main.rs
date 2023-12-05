use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let file_path = "src/inputreal.txt";
    let file = File::open(file_path)?;
    let mut file_reader = BufReader::new(file);

    let mut input: String = String::new();
    file_reader.read_to_string(&mut input)?;
    let (seeds, maps) = parse_input(&input);

    let mut dest: Vec<u64> = Vec::new();
    for seed in seeds {
        let mut prev: u64;
        let mut current: u64 = seed;

        for k in 0..maps.len() {
            prev = current;
            for n in 0..maps.get(k).unwrap().len() {
                let source_start_range: u64 = maps[k][n][1];
                let range: u64 = source_start_range + maps[k][n][2];
                let dest_start_range: u64 =  maps[k][n][0];
    
                if current >= source_start_range && current <= range {
                    current = current - source_start_range + dest_start_range;
                    break;
                } else {
                    current = prev;
                }
                
            }
        }
        dest.push(current);
    }

    print!("Total: {:?}", dest.iter().min());
    Ok(())
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    let all: Vec<_> = input.split("\n\n").collect();

    //parse seeds
    let mut seeds: Vec<u64> = all
        .get(0)
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    seeds.sort();

    let mut all_the_maps: Vec<Vec<Vec<u64>>> = Vec::new();

    for k in 1..all.len() {
        let map: Vec<Vec<u64>> = all
            .get(k)
            .unwrap()
            .split("\n")
            .filter(|line| line.chars().all(|c| c.is_digit(10) || c.is_whitespace()))
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|n| n.parse::<u64>().ok())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();
        all_the_maps.push(map);
    }
    (seeds, all_the_maps)
}
