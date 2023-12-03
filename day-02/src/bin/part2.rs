use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let blues = Regex::new(r"(\d+) blue").unwrap();
    let reds = Regex::new(r"(\d+) red").unwrap();
    let greens = Regex::new(r"(\d+) green").unwrap();
    let powers = lines.map(|line| {
        let blue_min = blues
            .captures_iter(line)
            .map(|f| f.get(1).unwrap().as_str().parse::<u32>().unwrap()).max().unwrap();
        let red_min = reds
            .captures_iter(line)
            .map(|f| f.get(1).unwrap().as_str().parse::<u32>().unwrap()).max().unwrap();
        let green_min = greens
            .captures_iter(line)
            .map(|f| f.get(1).unwrap().as_str().parse::<u32>().unwrap()).max().unwrap();
        blue_min * red_min * green_min
    });
    let sum = powers.sum::<u32>();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286".to_string());
    }
}
