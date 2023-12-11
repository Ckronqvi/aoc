
fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut all_numbers: Vec<Vec<i64>> = Vec::new();

    for line in input.lines() {
        all_numbers.push(
            line.split_ascii_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        );
    }

    let mut total: i64 = 0;
    for row in all_numbers {
        total += count_next(row);
    }

    print!("Total {}", total);
}

fn count_next(numbers: Vec<i64>) -> i64 {
    let mut extrapolated_lines: Vec<Vec<i64>> = Vec::new();
    let mut temp: Vec<i64> = numbers.clone();
    extrapolated_lines.push(numbers);
    loop {
        temp = get_next_line(&temp);
        if temp.iter().sum::<i64>() == 0 {
            break;
        }
        extrapolated_lines.push(temp.clone());
    }

    let mut i: i32 = (extrapolated_lines.len()-1) as i32;
    let mut prev = 0i64;
    while i >= 0 {
        let last: usize = extrapolated_lines[i as usize].len()-1;
        prev += extrapolated_lines[i as usize][last];
        i -= 1;
    }
    prev
}

fn get_next_line(numbers: &Vec<i64>) -> Vec<i64> {
    let mut new_numbers: Vec<i64> = Vec::new();

    for n in 0..numbers.len()-1 {
        new_numbers.push(numbers[n+1] - numbers[n]);
    }
    new_numbers
} 