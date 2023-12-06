use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|n| n.parse::<u32>())
        .filter_map(|n| n.ok());
    let distance = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|n| n.parse::<u32>())
        .filter_map(|n| n.ok());

    // zip times and distance together
    let ways = times.zip(distance).map(|(t, d)| {
        let mut ways = 0;
        for i in 1..t {
            let speed = i;
            let total_traveled = (t - i) * speed;
            if total_traveled > d {
                ways += 1
            }
        }
        ways
    });

    ways.product::<u64>().to_string()
}

fn part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE);
        assert_eq!(result, "288".to_string());
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE);
        assert_eq!(result, "30".to_string());
    }
}
