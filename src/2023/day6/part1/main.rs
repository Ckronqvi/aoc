
fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let inputline: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let first_speed = inputline.get(0).unwrap();
    let speeds_after_split: Vec<&str> = first_speed.split(":").collect::<Vec<&str>>();
    let times: Vec<u32> = speeds_after_split
        .get(1)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let distances: Vec<u32> = inputline
        .get(1)
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut total: i32 = 1;
    for i in 0..times.len() {
        let sum = calc_dist(times[i] as f32, distances[i] as f32);
        if sum.len() != 0 {
            total *= sum.len() as i32;
        }
    }

    println!("Total:{}", total);
}

fn calc_dist(n: f32, limit: f32) -> Vec<i32> {
    let mut result = Vec::new();

    for x in 0..=n as i32 {
        if x as f32 * (n - x as f32) > limit {
            result.push(x);
        }
    }

    result
}
