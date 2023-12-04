use regex::{Regex, Match};

fn main() {
    let input = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> String {
    let num_match = Regex::new(r"\d+").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        let prev_line = match i.checked_sub(1) {
            Some(i) => lines.get(i),
            None => None,
        }
        .unwrap_or(&"");
        let next_line = lines.get(i + 1).unwrap_or(&"");
        num_match.find_iter(line).for_each(|m| {
            let num = m.as_str().parse::<u32>().unwrap();
            let start = m.start();
            let end = m.end();
            if find_adjacent(start, end, prev_line, next_line, line) {
                sum += num;
            }
        });
    }
    return sum.to_string();
}

fn part2(input: &str) -> String {
    let num_match = Regex::new(r"\d+").unwrap();
    let gear_match = Regex::new(r"\*").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;
    fn format_number(m: Match<'_>) -> (usize, usize, u32) {
        let num = m.as_str().parse::<u32>().unwrap();
        let start = m.start();
        let end = m.end();
        (start, end, num)
    }
    for (i, line) in lines.iter().enumerate() {
        let cur_line_num_mapping: Vec<_> = num_match.find_iter(line).map(format_number).collect();
        let prev_line = match i.checked_sub(1) {
            Some(i) => lines.get(i),
            None => None,
        }
        .unwrap_or(&"");
        let prev_line_num_mapping: Vec<_> = num_match.find_iter(prev_line).map(format_number).collect();
        let next_line = lines.get(i + 1).unwrap_or(&"");
        let next_line_num_mapping: Vec<_> = num_match.find_iter(next_line).map(format_number).collect();
        gear_match.find_iter(line).for_each(|m| {
            let idx = m.start();
            sum += calculate_gear(idx, &prev_line_num_mapping, &next_line_num_mapping, &cur_line_num_mapping);
        });
    }
    return sum.to_string();
}

fn find_adjacent(
    start: usize,
    end: usize,
    top_line: &&str,
    bottom_line: &&str,
    current_line: &&str,
) -> bool {
    let symbol_regex = Regex::new(r"[^.\d]+").unwrap();
    // We can always assume that if the top and bottom lines exist, they will have the same length as 'current'
    // therefore, we can use the same start and end indices to get the adjacent characters
    let top = top_line
        .get(start.checked_sub(1).unwrap_or(0)..(end + 1).min(top_line.len()))
        .unwrap_or("");
    let bottom = bottom_line
        .get(start.checked_sub(1).unwrap_or(0)..(end + 1).min(bottom_line.len()))
        .unwrap_or("");
    let left = current_line
        .get(start.checked_sub(1).unwrap_or(0)..start)
        .unwrap_or("");
    let right = current_line.get(end..end + 1).unwrap_or("");
    return symbol_regex.is_match(top)
        || symbol_regex.is_match(bottom)
        || symbol_regex.is_match(left)
        || symbol_regex.is_match(right);
}

fn calculate_gear(
    idx: usize,
    top_line: &Vec<(usize, usize, u32)>,
    bottom_line: &Vec<(usize, usize, u32)>,
    current_line: &Vec<(usize, usize, u32)>,
) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    top_line.iter().for_each(|(start, end, num)| {
        if idx >= (*start).checked_sub(1).unwrap_or(0) && idx < *end + 1 {
            numbers.push(*num);
        }
    });
    bottom_line.iter().for_each(|(start, end, num)| {
        if idx >= (*start).checked_sub(1).unwrap_or(0) && idx < *end + 1{
            numbers.push(*num);
        }
    });
    current_line.iter().for_each(|(start, end, num)| {
        if idx == *end || idx + 1 == *start {
            numbers.push(*num);
        }
    });
    if numbers.len() == 2 {
        return numbers[0] * numbers[1];
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "4361".to_string());
    }

    #[test]
    fn part2_test() {
        let result = part2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "467835".to_string());
    }
}
