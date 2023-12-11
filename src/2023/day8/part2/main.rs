use std::collections::HashMap;


fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let moves: Vec<char> = input.split("\n\n").next().unwrap_or("").chars().collect();
    

    let mut moves_map: HashMap<&str,(&str, &str)> = HashMap::new();
    let mut starting_values: Vec<&str> = Vec::new();

    for line in input.lines().skip(2) {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        let key: &str = parts[0];
        if key.chars().nth(2).unwrap() == 'A' {
            starting_values.push(key);
        }
        let values: Vec<&str> = parts[1]
            .trim_matches(|c| c == ' ' || c == '(' || c == ')')
            .split(", ").collect::<Vec<&str>>();
        moves_map.insert(key, (values[0], values[1]));
    }

    let mut count: i32 = 1i32;
    let mut i: usize = 0;
    let mut curret_values: Vec<&str> = starting_values.clone();
    let mut all_ended_with_z: bool;
    println!("Finished the hashmap, its size is: {}", moves_map.len());

    let mut loop_values: Vec<i32> = Vec::new();
    while i < moves.len() {
        let next_move: usize = if moves[i] == 'L' {0} else {1};
        let mut next_values: Vec<&str> = Vec::new();
        all_ended_with_z = true;

        for value in curret_values.clone() {
            let next_value: &str = match next_move {
                0 => moves_map.get(value).unwrap().0,
                1 => moves_map.get(value).unwrap().1,
                _ => panic!("Invalid next_move value"),
            };

            if !next_value.ends_with("Z") {
                all_ended_with_z = false;
            } else {
                if loop_values.len() < 6 {
                    loop_values.push(count);
                }
            }
            next_values.push(next_value);
        }
        curret_values = next_values.clone();

        if all_ended_with_z || loop_values.len() >= 6 { 
            break;
        }

        count += 1;

        if i == moves.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }

    } 
    println!("{:?}", loop_values);
    let total_steps = loop_values.iter().cloned().fold(1, |acc, x| lcm(acc, x as i64));
    println!("Total: {}", total_steps);
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}



