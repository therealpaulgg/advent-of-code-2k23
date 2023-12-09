fn main() {
    let input = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> String {
    let stuff = input.lines().map(|line| {
        let nums = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let mut patterns: Vec<Vec<i64>> = Vec::new();
        patterns.push(nums);
        let mut binding = patterns.clone();
        let mut most_recent  = binding.last().unwrap();
        while most_recent.iter().any(|x| *x != 0) {
            let mut new_vec: Vec<i64> = Vec::new();
            for i in 0..most_recent.len() - 1 {
                new_vec.push(most_recent[i + 1] - most_recent[i]);
            }
            patterns.push(new_vec);
            binding = patterns.clone();
            most_recent = binding.last().unwrap();
        }
        patterns.reverse();
        for i in 0..patterns.len() - 1 {
            // get current list
            let borrowed_patterns  = patterns.clone();
            let borrowed = borrowed_patterns.get(i).clone().unwrap();
            let last_1 = borrowed.last().unwrap();
            // get next list
            let pattern_to_add: &mut Vec<i64> = patterns.get_mut(i+1).unwrap();
            let last_2 = pattern_to_add.last().unwrap();
            pattern_to_add.push(last_1 + last_2);
        }
        *patterns.last().unwrap().last().unwrap()
    });
    stuff.sum::<i64>().to_string()
}

fn part2(input: &str) -> String {
    let stuff = input.lines().map(|line| {
        let nums = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let mut patterns: Vec<Vec<i64>> = Vec::new();
        patterns.push(nums);
        let mut binding = patterns.clone();
        let mut most_recent  = binding.last().unwrap();
        while most_recent.iter().any(|x| *x != 0) {
            let mut new_vec: Vec<i64> = Vec::new();
            for i in 0..most_recent.len() - 1 {
                new_vec.push(most_recent[i + 1] - most_recent[i]);
            }
            patterns.push(new_vec);
            binding = patterns.clone();
            most_recent = binding.last().unwrap();
        }
        patterns.reverse();
        for i in 0..patterns.len() - 1 {
            let borrowed_patterns  = patterns.clone();
            let borrowed = borrowed_patterns.get(i).clone().unwrap();
            let first_1 = borrowed.first().unwrap();
            let pattern_to_add: &mut Vec<i64> = patterns.get_mut(i+1).unwrap();
            let first_2 = pattern_to_add.first().unwrap();
            let new_val = &[first_2 - first_1];
            pattern_to_add.splice(0..0, new_val.iter().cloned());
        }
        *patterns.last().unwrap().first().unwrap()
    });
    stuff.sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE);
        assert_eq!(result, "114".to_string());
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE);
        assert_eq!(result, "2".to_string());
    }
}
