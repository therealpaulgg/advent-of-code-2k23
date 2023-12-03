use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;
    let game_id = Regex::new(r"Game (\d+):").unwrap();
    let blues = Regex::new(r"(\d+) blue").unwrap();
    let reds = Regex::new(r"(\d+) red").unwrap();
    let greens = Regex::new(r"(\d+) green").unwrap();
    let counts = lines.map(|line| {
        let game_id_str = game_id.captures(line).unwrap().get(1).unwrap().as_str();
        let game_id = game_id_str.parse::<u32>().unwrap();
        let possible = line.split(';').all(|set| {
            let blues_possible = blues
                .captures(set)
                .map_or("0", |f| f.get(1).unwrap().as_str())
                .parse::<u32>()
                .unwrap()
                <= blue_cubes;
            let reds_possible = reds
                .captures(set)
                .map_or("0", |f| f.get(1).unwrap().as_str())
                .parse::<u32>()
                .unwrap()
                <= red_cubes;
            let greens_possible = greens
                .captures(set)
                .map_or("0", |f| f.get(1).unwrap().as_str())
                .parse::<u32>()
                .unwrap()
                <= green_cubes;
            blues_possible && reds_possible && greens_possible
        });
        if possible {
            return game_id;
        }
        0
    });
    let sum = counts.sum::<u32>();
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
        assert_eq!(result, "8".to_string());
    }
}
