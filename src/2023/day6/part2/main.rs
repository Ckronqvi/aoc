
fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let inputline: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let first_speed = inputline.get(0).unwrap();
    let speeds_after_split: Vec<&str> = first_speed.split(":").collect::<Vec<&str>>();
    let times: Vec<u64> = speeds_after_split
        .get(1)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();


    let distances: Vec<u64> = inputline
        .get(1)
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let time = concat_and_parse(times);
    let distance = concat_and_parse(distances);
    let total: usize = count_solutions(time as f64, distance as f64);
    

    println!("Total:{}", total);
}

fn count_solutions(n: f64, limit: f64) -> usize {
    let a = -1.0;
    let b = n;
    let c = -limit;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        // No real solutions
        return 0;
    }

    let sqrt_discriminant = discriminant.sqrt();

    let x1 = ((-b + sqrt_discriminant) / (2.0 * a)) as i64;
    let x2 = ((-b - sqrt_discriminant) / (2.0 * a)) as i64;

    // Count the whole number solutions
    let count = (x1..=x2).filter(|&x| x as f64 * (n - x as f64) > limit).count();

    count
}

fn concat_and_parse(numbers: Vec<u64>) -> u64 {
    let concatenated_str: String = numbers
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join("");

    let result: Result<u64, std::num::ParseIntError> = concatenated_str.parse();
    result.unwrap()
}
