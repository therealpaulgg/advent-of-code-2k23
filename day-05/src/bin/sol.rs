use std::{collections::HashMap, str::Lines, cmp};

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> String {
    let mut seeds = input
        .lines()
        .next()
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut lines = input.lines();

    let maps = construct_maps(&mut lines);

    for map in maps {
        for i in 0..seeds.len() {
            let seed = seeds[i];
            for range in map.clone() {
                let (src, upper_bound, distance) = range;
                if seed >= src && seed < upper_bound {
                    seeds[i] = seed + distance;
                    break;
                }
            }
        }
    }
    seeds.sort();
    seeds[0].to_string()
}

fn construct_maps(lines: &mut Lines) -> Vec<Vec<(i64, i64, i64)>> {
    let map_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();

    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    // while there are still lines...
    loop {
        match lines.next() {
            Some(res) => {
                if map_re.find(res).is_some() {
                    // You cannot construct maps with an actual hashmap - must use ranges.
                    // Doing this with maps is very storage inefficient and didn't run
                    // in a reasonable amount of time.
                    let mut ranges: Vec<(i64, i64, i64)> = Vec::new();
                    loop {
                        match lines.next() {
                            Some(res) => {
                                let num_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
                                let m = num_re.captures(res);
                                if m.is_some() {
                                    let dat = m.unwrap();
                                    let dst = dat.get(1).unwrap().as_str().parse::<i64>().unwrap();
                                    let src = dat.get(2).unwrap().as_str().parse::<i64>().unwrap();
                                    let len = dat.get(3).unwrap().as_str().parse::<i64>().unwrap();
                                    let distance = dst - src;
                                    let upper_bound = src + len;
                                    ranges.push((src, upper_bound, distance));
                                } else {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }
                    maps.push(ranges);
                }
            }
            None => break,
        }
    }
    maps
}

fn part2(input: &str) -> String {
    // NO probably have to construct all the 'maps' before hand for part 2 since the number of seeds is so much.

    // thought: Seed ranges can correspond to multiple seed to soil ranges.
    // Find all those ranges, iterate over all of them, and drill down to try to find the smallest value.
    // Every 2 numbers is a pair, so this isnt a list of numbers, but a list of (i64, i64)
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let seed_ranges = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<(i64, i64)>>();

    let mut lines = input.lines();

    let maps = construct_maps(&mut lines);

    let mut mins: Vec<i64> = Vec::new();

    let mut location = 0;
    let mut seeker_val = location;
    loop {
        for map in maps.iter().rev() {
            for range in map.clone() {
                let (src, upper_bound, distance) = range;
                let len = upper_bound - src;
                let dst = src + distance;
                if seeker_val >= dst && seeker_val < dst + len {
                    seeker_val -= distance;
                    break;
                }
            }
        }
        // check if seed in seed range
        for seed_range in seed_ranges.clone() {
            if seeker_val >= seed_range.0 && seeker_val < seed_range.0 + seed_range.1 {
                return location.to_string();
            }
        }
        location += 1;
        seeker_val = location;
    }

    "unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE);
        assert_eq!(result, "35".to_string());
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE);
        assert_eq!(result, "46".to_string());
    }
}
