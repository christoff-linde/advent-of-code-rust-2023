use std::u32;

use itertools::{enumerate, Itertools};
use nom::ParseTo;

advent_of_code::solution!(6);

// Determine the optimal holding time to maximize the travel distance
// If achieved distance is <= than the record, just discard
// Multiply count of possible permutations with count of each race

fn max_distance(input: &str) -> u32 {
    let race_time = input.parse::<u32>().unwrap();

    // for index from 0 to race_time
    // 1. find the distance at that time

    let mut distance = 0;

    for index in 0..race_time {
        let rem_time = race_time - index;
        let max_speed = index;

        distance = rem_time * max_speed;

        dbg!(distance);
    }

    distance
}

fn valid_race_count(race_time: u32, race_distance_record: u32) -> u32 {
    let mut valid_races = 0;

    for index in 0..race_time {
        let rem_time = race_time - index;
        let max_speed = index;

        let distance = rem_time * max_speed;
        if distance > race_distance_record {
            valid_races += 1;
        }
    }

    valid_races
}

pub fn part_one(input: &str) -> Option<u32> {
    // read only first line of input
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let (_, races) = first_line.split_once(": ").unwrap();
    let races = races.split_whitespace().collect_vec();
    dbg!(&races);

    let second_line = lines.next().unwrap();
    let (_, distances) = second_line.split_once(": ").unwrap();
    let distances = distances.split_whitespace().collect_vec();
    dbg!(&distances);

    let mut possible_combinations = Vec::new();
    // iterate over both distances and races vectors
    for (index, data) in races.iter().enumerate() {
        let race_time = data.parse::<u32>().unwrap();
        let distance_record = distances[index].parse::<u32>().unwrap();

        let valid_race_count = valid_race_count(race_time, distance_record);
        possible_combinations.push(valid_race_count);
    }

    let total = possible_combinations.iter().product::<u32>();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
