use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let total: i64;

    let mut hands: Vec<(Handtype, Vec<i32>, i32)> = Vec::new();

    for line in input.lines() {
        let li = line.split_whitespace().collect::<Vec<&str>>();

        let hand = li
            .get(0)
            .unwrap()
            .chars()
            .map(|c| card_to_number(c))
            .collect::<Vec<i32>>();
        let handtype: Handtype = count_handtype(hand.clone());
        let bid = li.get(1).unwrap().parse::<i32>().unwrap();
        hands.push((handtype, hand, bid));
    }
    hands.sort_by(|a, b| {
        a.0.cmp(&b.0).then_with(|| {
            // If Handtypes are equal, compare based on the cards in the hand
            for (card_a, card_b) in a.1.iter().zip(b.1.iter()) {
                let cmp_result = card_b.cmp(card_a);
                if cmp_result != std::cmp::Ordering::Equal {
                    return cmp_result;
                }
            }
            std::cmp::Ordering::Equal
        })
    });

    total = hands.iter().enumerate().map(|(i, hand)| {
        hand.2 as i64 * (hands.len() as i64 - i as i64)
    }).sum();

    print!("Total: {}", total);
}

fn card_to_number(card: char) -> i32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_string().parse::<i32>().unwrap(),
    }
}

fn count_handtype(numbers: Vec<i32>) -> Handtype {
    let mut number_counts: HashMap<i32, i32> = HashMap::new();

    for num in numbers {
        *number_counts.entry(num).or_insert(0) += 1;
    }

    let has_five_of_a_kind = number_counts.values().any(|&count| count == 5);
    let has_four_of_a_kind = number_counts.values().any(|&count| count == 4);
    let has_three_of_a_kind = number_counts.values().any(|&count| count == 3);
    let has_two_of_a_kind = number_counts.values().any(|&count| count == 2);
    let unique_labels = number_counts.len();

    match (has_five_of_a_kind, has_four_of_a_kind, has_three_of_a_kind, has_two_of_a_kind, unique_labels) {
        (true, _, _, _, _) => Handtype::FiveOfAKind,
        (_, _, true, true, _) => Handtype::FullHouse,
        (_, true, _, _, _) => Handtype::FourOfAKind,
        (false, false, false, true, 3) => Handtype::TwoPair,
        (_, _, true, false, _) => Handtype::ThreeOfAKind,
        (_, _, false, true, 4) => Handtype::OnePair,
        _ => Handtype::HighCard,
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Handtype {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
