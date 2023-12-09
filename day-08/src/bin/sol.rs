use std::{clone, collections::HashMap};

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    lines.next();
    let pair = Regex::new(r"([A-Z]{3})\s=\s\(([A-Z]{3}),\s([A-Z]{3})\)").unwrap();
    let map: HashMap<_, _> = lines
        .map(|line| {
            let cap = pair.captures(line).unwrap();
            let res1 = cap.get(1).map(|x| x.as_str()).unwrap();
            let res2 = cap.get(2).map(|x| x.as_str()).unwrap();
            let res3 = cap.get(3).map(|x| x.as_str()).unwrap();
            (res1, (res2, res3))
        })
        .collect();

    let mut pointer = "AAA";
    let end = "ZZZ";
    let mut steps = 0;
    while pointer != end {
        let path = map.get(pointer).unwrap();
        let direction = directions.chars().nth(steps % directions.len()).unwrap();
        pointer = match direction {
            'L' => path.0,
            'R' => path.1,
            _ => panic!("Invalid direction"),
        };
        steps += 1;
    }

    steps.to_string()
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    lines.next();
    let pair = Regex::new(r"(\w{3})\s=\s\((\w{3}),\s(\w{3})\)").unwrap();
    let map: HashMap<_, _> = lines
        .map(|line| {
            let cap = pair.captures(line).unwrap();
            let res1 = cap.get(1).map(|x| x.as_str()).unwrap();
            let res2 = cap.get(2).map(|x| x.as_str()).unwrap();
            let res3 = cap.get(3).map(|x| x.as_str()).unwrap();
            (res1, (res2, res3))
        })
        .collect();

    let pointer_nodes = map.keys().filter(|x| x.ends_with('A'));

    let steps_total: Vec<_> = pointer_nodes
        .map(|node| traverse(directions, map.clone(), node, 0))
        .collect();

    lcm_vec(steps_total).to_string()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm_vec(numbers: Vec<usize>) -> usize {
    numbers
        .iter()
        .fold(1, |lcm, &number| lcm * number / gcd(lcm, number))
}

fn traverse(
    directions: &str,
    map: HashMap<&str, (&str, &str)>,
    current: &str,
    steps: usize,
) -> usize {
    if current.ends_with('Z') {
        return steps;
    }
    let path = map.get(current).unwrap();
    let direction = directions.chars().nth(steps % directions.len()).unwrap();
    let pointer = match direction {
        'L' => path.0,
        'R' => path.1,
        _ => panic!("Invalid direction"),
    };
    return traverse(directions, map, pointer, steps + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_test_1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, "2".to_string());
    }

    #[test]
    fn part1_test_2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, "6".to_string());
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE3);
        assert_eq!(result, "6".to_string());
    }
}
