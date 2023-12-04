use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let num_match = Regex::new(r"\d+").unwrap();
    let gear_match = Regex::new(r"\*").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        let cur_line_num_mapping: Vec<_> = num_match.find_iter(line).map(|m| {
            let num = m.as_str().parse::<u32>().unwrap();
            let start = m.start();
            let end = m.end();
            (start, end, num)
        }).collect();
        let prev_line = match i.checked_sub(1) {
            Some(i) => lines.get(i),
            None => None,
        }
        .unwrap_or(&"");
        let prev_line_num_mapping: Vec<_> = num_match.find_iter(prev_line).map(|m| {
            let num = m.as_str().parse::<u32>().unwrap();
            let start = m.start();
            let end = m.end();
            (start, end, num)
        }).collect();
        let next_line = lines.get(i + 1).unwrap_or(&"");
        let next_line_num_mapping: Vec<_> = num_match.find_iter(next_line).map(|m| {
            let num = m.as_str().parse::<u32>().unwrap();
            let start = m.start();
            let end = m.end();
            (start, end, num)
        }).collect();
        gear_match.find_iter(line).for_each(|m| {
            let gear = m.as_str();
            let idx = m.start();
            sum += calculate_gear(idx, &prev_line_num_mapping, &next_line_num_mapping, &cur_line_num_mapping);
        });
    }
    return sum.to_string();
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
        // If the current line has a number to the left or to the right
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
    fn it_works() {
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
