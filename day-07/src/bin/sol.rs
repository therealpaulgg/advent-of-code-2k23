use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> String {
    let cap = Regex::new(r"(\w+) (\d+)").unwrap();
    let hands = input.lines().map(|line| {
        let capt = cap.captures(line).unwrap();
        let hand = capt.get(1).unwrap().as_str();
        let num = capt.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let hand_type = classify_hand(hand);
        (hand_type, hand, num)
    });
    let col  = hands.collect::<Vec<_>>();
    // map by hand type
    let mut hand_map: HashMap<Hand, (Hand, &str, u32)> = HashMap::new();
    hands.for_each(|hand| {
        let (hand_type, hand, num) = hand;
        let entry = hand_map.entry(hand_type).or_insert((hand_type, hand, num));
        if entry.2 < num {
            *entry = (hand_type, hand, num);
        }
    });
    "".to_string()
}

fn part2(input: &str) -> String {
    "".to_string()
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Hand {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn classify_hand(hand: &str) -> Hand {
    // Hand contains one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    // every hand has 5 characters.
    // check if all characters in the hand are the same
    if hand.chars().all(|c| c == hand.chars().next().unwrap()) {
        return Hand::FiveOfAKind;
    }
    let mut handMap = HashMap::new();
    for c in hand.chars() {
        let count = handMap.entry(c).or_insert(0);
        *count += 1;
    }
    if handMap.keys().len() == 2 {
        // can be 4ofakind or fullhouse
        let first = handMap.keys().nth(0).unwrap();
        let second = handMap.keys().nth(1).unwrap();
        if handMap.get(first) == Some(&4) || handMap.get(second) == Some(&4) {
            return Hand::FourOfAKind;
        }
        return Hand::FullHouse;
    }
    if handMap.keys().len() == 3 {
        // can be 3ofakind or twopairs
        let first = handMap.keys().nth(0).unwrap();
        let second = handMap.keys().nth(1).unwrap();
        let third = handMap.keys().nth(2).unwrap();
        if handMap.get(first) == Some(&3)
            || handMap.get(second) == Some(&3)
            || handMap.get(third) == Some(&3)
        {
            return Hand::ThreeOfAKind;
        }
        return Hand::TwoPairs;
    }
    if handMap.keys().len() == 4 {
        return Hand::OnePair;
    }
    Hand::HighCard
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE);
        assert_eq!(result, "6440".to_string());
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE);
        assert_eq!(result, "71503".to_string());
    }

    #[test]
    fn five_of_a_kind() {
        let result = classify_hand("22222");
        assert_eq!(result, Hand::FiveOfAKind);
    }

    #[test]
    fn four_of_a_kind() {
        let result = classify_hand("22223");
        assert_eq!(result, Hand::FourOfAKind);
    }

    #[test]
    fn full_house() {
        let result = classify_hand("22233");
        assert_eq!(result, Hand::FullHouse);
    }

    #[test]
    fn three_of_a_kind() {
        let result = classify_hand("22234");
        assert_eq!(result, Hand::ThreeOfAKind);
    }

    #[test]
    fn two_pairs() {
        let result = classify_hand("22244");
        assert_eq!(result, Hand::TwoPairs);
    }

    #[test]
    fn one_pair() {
        let result = classify_hand("22345");
        assert_eq!(result, Hand::OnePair);
    }

    #[test]
    fn high_card() {
        let result = classify_hand("23456");
        assert_eq!(result, Hand::HighCard);
    }
}
