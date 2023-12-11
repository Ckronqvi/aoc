use std::collections::HashMap;


fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let moves: Vec<char> = input.split("\n\n").next().unwrap_or("").chars().collect();
    

    let mut moves_map: HashMap<&str,(&str, &str)> = HashMap::new();

    for line in input.lines().skip(2) {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        let key: &str = parts[0];
        let values: Vec<&str> = parts[1]
            .trim_matches(|c| c == ' ' || c == '(' || c == ')')
            .split(", ").collect::<Vec<&str>>();
        moves_map.insert(key, (values[0], values[1]));
    }

    let mut count = 0i32;
    let mut i = 0;
    let mut curr_value = "AAA";

    println!("Finished the hashmap, its size is: {}", moves_map.len());
    
    while i < moves.len() {
        
        let next_move: usize = if moves[i] == 'L' {0} else {1};
        count += 1;
        let next_value: &str = match next_move {
            0 => moves_map.get(curr_value).unwrap().0,
            1 => moves_map.get(curr_value).unwrap().1,
            _ => panic!("Invalid next_move value"),
        };

        if next_value == "ZZZ" {
            println!("Moves: {}", count);
            break;
        }

        curr_value = next_value;

        if i == moves.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }

    } 
}



