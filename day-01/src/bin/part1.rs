fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let numbers: Vec<String> = lines
        .map(|line| {
            // can use easy char approach since all numbers are just a char
            let chars: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            let first = chars.first().unwrap().to_string();
            let last = chars.last().unwrap().to_string();
            format!("{}{}", first, last)
        })
        .collect();
    numbers.into_iter().map(|number| number.parse::<u32>().unwrap()).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142".to_string());
    }
}
