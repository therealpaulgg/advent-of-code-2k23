use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::hash_map;

fn main() {
    let input = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn get_map(input: &str, use_jokers: bool) -> BTreeMap<Hand, Vec<(Hand, &str, usize)>> {
    let cap = Regex::new(r"(\w+) (\d+)").unwrap();
    let hands = input.lines().map(|line| {
        let capt = cap.captures(line).unwrap();
        let hand = capt.get(1).unwrap().as_str();
        let num = capt.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let hand_type = classify_hand(hand, use_jokers);
        (hand_type, hand, num)
    });
    // map by hand type
    let mut hand_map: BTreeMap<Hand, Vec<(Hand, &str, usize)>> = BTreeMap::new();
    hands.clone().for_each(|hand| {
        let (hand_type, hand, num) = hand;
        let entry = hand_map.entry(hand_type).or_insert(Vec::new());
        entry.push((hand_type, hand, num));
    });
    hand_map
}

fn part1(input: &str) -> String {
    let card_value_mapping = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    let hand_map = get_map(input, false);
    let mut all_vals: Vec<(Hand, &str, usize)> = Vec::new();
    for mut thing in hand_map.clone() {
        thing.1.sort_by(|a, b| {
            for i in 0..5 {
                let char_a = a.1.chars().nth(i).unwrap();
                let char_b = b.1.chars().nth(i).unwrap();
                let cmp = card_value_mapping
                    .get(&char_a)
                    .unwrap()
                    .cmp(card_value_mapping.get(&char_b).unwrap());
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            Ordering::Equal
        });
        all_vals.append(&mut thing.1)
    }
    let mut total: usize = 0;
    for i in 0..all_vals.len() {
        total += all_vals.get(i).unwrap().2 * (i + 1)
    }
    total.to_string()
}

fn part2(input: &str) -> String {
    let card_value_mapping = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 1),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    let hand_map = get_map(input, true);
    let mut all_vals: Vec<(Hand, &str, usize)> = Vec::new();
    for mut thing in hand_map.clone() {
        thing.1.sort_by(|a, b| {
            for i in 0..5 {
                let char_a = a.1.chars().nth(i).unwrap();
                let char_b = b.1.chars().nth(i).unwrap();
                let cmp = card_value_mapping
                    .get(&char_a)
                    .unwrap()
                    .cmp(card_value_mapping.get(&char_b).unwrap());
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            Ordering::Equal
        });
        all_vals.append(&mut thing.1)
    }
    let mut total: usize = 0;
    for i in 0..all_vals.len() {
        total += all_vals.get(i).unwrap().2 * (i + 1)
    }
    total.to_string()
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn classify_hand(hand: &str, use_jokers: bool) -> Hand {
    // Hand contains one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    // every hand has 5 characters.
    let mut hand_map = HashMap::new();
    for c in hand.chars() {
        let count = hand_map.entry(c).or_insert(0);
        *count += 1;
    }

    if use_jokers {
        let mut target = hand.chars().nth(0).unwrap();
        for c in hand.chars() {
            if c != 'J' && (hand_map[&c] > hand_map[&target] || target == 'J') {
                target = c;
            }
        }
        let jokers = hand_map.get(&'J');
        if jokers.is_some() && target != 'J' {
            hand_map.insert(target, hand_map.get(&target).unwrap() + jokers.unwrap());
            hand_map.remove(&'J');
        }
    }

    if hand_map.keys().len() == 1 {
        return Hand::FiveOfAKind;
    }

    if hand_map.keys().len() == 2 {
        let keys: Vec<_> = hand_map.keys().collect();
        let four_of_a_kind_key = keys.iter().find(|&&k| hand_map.get(k) == Some(&4));
        if four_of_a_kind_key.is_some() {
            return Hand::FourOfAKind;
        }
        return Hand::FullHouse;
    }
    if hand_map.keys().len() == 3 {
        let keys: Vec<_> = hand_map.keys().collect();
        let three_of_a_kind_key = keys.iter().find(|&&k| hand_map.get(k) == Some(&3));
        if three_of_a_kind_key.is_some() {
            return Hand::ThreeOfAKind;
        }
        for i in 0..3 {
            for j in i + 1..3 {
                if hand_map.get(keys[i]) == Some(&2) && hand_map.get(keys[j]) == Some(&2) {
                    return Hand::TwoPairs;
                }
            }
        }
    }
    if hand_map.keys().len() == 4 {
        for key in hand_map.keys() {
            if hand_map.get(key) == Some(&2) {
                return Hand::OnePair;
            }
        }
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
        assert_eq!(result, "5905".to_string());
    }

    #[test]
    fn five_of_a_kind() {
        let result = classify_hand("22222", false);
        assert_eq!(result, Hand::FiveOfAKind);
    }

    #[test]
    fn four_of_a_kind() {
        let result = classify_hand("22223", false);
        assert_eq!(result, Hand::FourOfAKind);
    }

    #[test]
    fn full_house() {
        let result = classify_hand("22233", false);
        assert_eq!(result, Hand::FullHouse);
    }

    #[test]
    fn three_of_a_kind() {
        let result = classify_hand("22234", false);
        assert_eq!(result, Hand::ThreeOfAKind);
    }

    #[test]
    fn two_pairs() {
        let result = classify_hand("22544", false);
        assert_eq!(result, Hand::TwoPairs);
    }

    #[test]
    fn one_pair() {
        let result = classify_hand("22345", false);
        assert_eq!(result, Hand::OnePair);
    }

    #[test]
    fn high_card() {
        let result = classify_hand("23456", false);
        assert_eq!(result, Hand::HighCard);
    }
}
