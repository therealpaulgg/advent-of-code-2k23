use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_numbers_from_line(line: &str) -> String {
    let word_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ];
    // create a mapping of the string representations to numbers
    let mapping = word_numbers.iter().enumerate().map(|(i, word)| (*word, (i % 9) + 1)).collect::<HashMap<_, _>>();
    let mut numbers: Vec<(usize, &&str)> = word_numbers
        .iter()
        // flat map is required because the inner function returns an iterator (multiple numbers)
        .flat_map(|word| {
            let re = Regex::new(word).unwrap();
            // cant return a reference to a local variable so have to collect
            re.find_iter(line).map(|mat| (mat.start(), word)).collect::<Vec<_>>()
        })
        .collect();
    // sort list to find first & last easily
    numbers.sort_by(|a, b| a.0.cmp(&b.0));
    let first = numbers.first().unwrap().1;
    let last = numbers.last().unwrap().1;
    format!("{}{}", mapping[first], mapping[last])
}

fn part2(input: &str) -> String {
    let lines = input.lines();
    let numbers: Vec<String> = lines
        // solution had to be revised because now numbers like 'one' are more than one character long
        .map(|line| get_numbers_from_line(line))
        .collect();
    numbers
        .into_iter()
        .map(|number| number.parse::<u32>().unwrap())
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, "281".to_string());
    }
}
