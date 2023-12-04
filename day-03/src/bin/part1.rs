use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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
}
