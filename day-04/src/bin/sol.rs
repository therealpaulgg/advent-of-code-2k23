use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"Card\s+\d+: (.*) \| (.*)").unwrap();
    let scores = input.lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let winning = cap.get(1).unwrap().as_str();
        let player = cap.get(2).unwrap().as_str();
        let winning_nums = winning.split(' ').map(|n| n.parse::<u32>()).filter_map(|n| n.ok());
        let player_nums = player.split(' ').map(|n| n.parse::<u32>()).filter_map(|n| n.ok());
        // find the number of player_nums that are also in winning_nums. do it with a hashmap for fast runtime
        let mut winning_map = HashMap::new();
        winning_nums.for_each(|n| {
            winning_map.insert(n, true);
        });
        let mut score = 0;
        player_nums.for_each(|n| {
            if winning_map.contains_key(&n) {
                score += 1;
            }
        });
        if score == 0 {
            return 0
        }
        (2 as u32).pow(score - 1)
    });
    let sum = scores.sum::<u32>();
    sum.to_string()
}

fn part2(input: &str) -> String {
    let re = Regex::new(r"Card\s+\d+: (.*) \| (.*)").unwrap();
    let scaler = input.lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let winning = cap.get(1).unwrap().as_str();
        let player = cap.get(2).unwrap().as_str();
        let winning_nums = winning.split(' ').map(|n| n.parse::<u32>()).filter_map(|n| n.ok());
        let player_nums = player.split(' ').map(|n| n.parse::<u32>()).filter_map(|n| n.ok());
        // find the number of player_nums that are also in winning_nums. do it with a hashmap for fast runtime
        let mut winning_map = HashMap::new();
        winning_nums.for_each(|n| {
            winning_map.insert(n, true);
        });
        let mut score = 0;
        player_nums.for_each(|n| {
            if winning_map.contains_key(&n) {
                score += 1;
            }
        });
        score
    }).collect::<Vec<_>>();
    let mut copies = scaler.iter().map(|_n| 1).collect::<Vec<_>>();
    for i in 0..copies.len() {
        let num_copies = copies[i];
        let scalar = scaler[i];
        for j in i+1..i + scalar + 1 {
            copies[j] += num_copies;
        }
    }
    copies.into_iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "13".to_string());
    }

    #[test]
    fn part2_test() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "30".to_string());
    }
}
